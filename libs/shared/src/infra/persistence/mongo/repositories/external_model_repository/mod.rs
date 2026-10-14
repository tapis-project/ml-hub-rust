use std::time::Duration;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson, Bson, Document, Regex, Uuid},
    Client, Collection,
};

use crate::{
    application::{
        inputs::discover_models::{NumericRange, SearchCriterion, SearchExternalModelsInput},
        ports::{
            errors::InfrastructureError,
            model::{
                ExternalModelPage, ExternalModelRepository as ExternalModelRepositoryPort,
                ExternalModelRepositoryError,
            },
        },
    },
    domain::entities::model::external_model::ExternalModelId,
    domain::entities::model::external_model::{
        ExternalModel as DomainExternalModel, ModelLocator, ModelProvider,
    },
    infra::persistence::mongo::{
        database::EXTERNAL_MODEL_COLLECTION,
        documents::external_model::{ExternalModel, ModelProvider as DocumentModelProvider},
        documents::{
            deployment::DeploymentModality as DocumentDeploymentModality,
            deployment_option::ServingRuntime as DocumentServingRuntime,
        },
    },
};

pub struct ExternalModelRepository {
    collection: Collection<ExternalModel>,
}

impl ExternalModelRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        Self {
            collection: client
                .database(&db_name)
                .collection(EXTERNAL_MODEL_COLLECTION),
        }
    }

    async fn find_one(
        &self,
        filter: Document,
    ) -> Result<Option<DomainExternalModel>, ExternalModelRepositoryError> {
        self.collection
            .find_one(filter)
            .await
            .map_err(map_error)?
            .map(TryInto::try_into)
            .transpose()
            .map_err(map_conversion_error)
    }
}

#[async_trait]
impl ExternalModelRepositoryPort for ExternalModelRepository {
    async fn save(&self, model: &DomainExternalModel) -> Result<(), ExternalModelRepositoryError> {
        let document = ExternalModel::try_from(model).map_err(map_conversion_error)?;

        self.collection
            .insert_one(document)
            .await
            .map_err(map_error)?;

        Ok(())
    }

    async fn update(
        &self,
        model: &DomainExternalModel,
    ) -> Result<(), ExternalModelRepositoryError> {
        let document = ExternalModel::try_from(model).map_err(map_conversion_error)?;

        self.collection
            .replace_one(doc! { "id": &document.id }, document)
            .await
            .map_err(map_error)?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ExternalModelId,
    ) -> Result<Option<DomainExternalModel>, ExternalModelRepositoryError> {
        self.find_one(doc! { "id": Uuid::from_bytes(*id.as_uuid().as_bytes()) })
            .await
    }

    async fn find_by_ids(
        &self,
        ids: &[ExternalModelId],
    ) -> Result<Vec<DomainExternalModel>, ExternalModelRepositoryError> {
        let ids = ids
            .iter()
            .map(|id| Uuid::from_bytes(*id.as_uuid().as_bytes()))
            .collect::<Vec<_>>();

        let mut cursor = self
            .collection
            .find(doc! { "id": { "$in": ids } })
            .await
            .map_err(map_error)?;

        let mut models = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            models.push(document.try_into().map_err(map_conversion_error)?);
        }

        Ok(models)
    }

    async fn find_by_provider_and_locator(
        &self,
        provider: &ModelProvider,
        locator: &ModelLocator,
    ) -> Result<Option<DomainExternalModel>, ExternalModelRepositoryError> {
        let filter = match (provider, locator) {
            (ModelProvider::HuggingFace, ModelLocator::HuggingFace(locator)) => doc! {
                "provider": to_bson(&DocumentModelProvider::HuggingFace).map_err(map_error)?,
                "huggingface_repo_locator.id": locator.id(),
                "huggingface_repo_locator.sha": locator.sha(),
            },
            (ModelProvider::Tapis, ModelLocator::Tapis(locator)) => doc! {
                "provider": to_bson(&DocumentModelProvider::Tapis).map_err(map_error)?,
                "tapis_system_locator.site_id": locator.site_id(),
                "tapis_system_locator.tenant_id": locator.tenant_id(),
                "tapis_system_locator.system_id": locator.system_id(),
                "tapis_system_locator.path": locator.path(),
            },
            _ => return Ok(None),
        };

        self.find_one(filter).await
    }

    async fn search(
        &self,
        input: &SearchExternalModelsInput,
    ) -> Result<ExternalModelPage, ExternalModelRepositoryError> {
        let mut filter = if input.criteria.is_empty() {
            Document::new()
        } else {
            doc! { "$or": input.criteria.iter().map(criterion_filter).collect::<Result<Vec<_>, _>>()? }
        };

        let count_filter = filter.clone();

        if let Some(cursor) = input.options.cursor() {
            filter
                .extend(doc! { "_id": { "$gt": ObjectId::parse_str(cursor).map_err(map_error)? } });
        }

        let use_deployment_options = input.criteria.iter().any(uses_deployment_options);

        let mut pipeline = Vec::new();

        if use_deployment_options {
            pipeline.push(deployment_option_lookup());
        }

        pipeline.extend([
            doc! { "$match": filter },
            doc! { "$sort": { "_id": 1 } },
            doc! { "$limit": i64::from(input.options.limit()) + 1 },
            doc! { "$unset": "deployment_options" },
        ]);

        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .map_err(map_error)?;

        let mut documents = Vec::new();

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            documents.push(from_document::<ExternalModel>(document).map_err(map_error)?);
        }

        let next_cursor = if documents.len() > usize::from(input.options.limit()) {
            documents.pop();
            documents
                .last()
                .and_then(|document| document._id.map(|id| id.to_hex()))
        } else {
            None
        };

        let external_models = documents
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(map_conversion_error)?;

        let count = if input.options.include_count() && use_deployment_options {
            let mut count_pipeline = vec![deployment_option_lookup()];

            count_pipeline.extend([doc! { "$match": count_filter }, doc! { "$count": "count" }]);

            let mut count_cursor = self
                .collection
                .aggregate(count_pipeline)
                .max_time(Duration::from_secs(2))
                .await
                .map_err(map_error)?;

            let count = count_cursor
                .try_next()
                .await
                .map_err(map_error)?
                .map(|document| match document.get("count") {
                    Some(Bson::Int32(count)) => Ok(u64::from(*count as u32)),
                    Some(Bson::Int64(count)) => Ok(*count as u64),
                    value => Err(format!("Invalid deployment option count: {value:?}")),
                })
                .transpose()
                .map_err(map_error)?
                .unwrap_or(0);

            Some(count)
        } else if input.options.include_count() {
            Some(
                self.collection
                    .count_documents(count_filter)
                    .max_time(Duration::from_secs(2))
                    .await
                    .map_err(map_error)?,
            )
        } else {
            None
        };

        Ok(ExternalModelPage {
            external_models,
            count,
            cursor: next_cursor,
        })
    }
}

fn criterion_filter(criterion: &SearchCriterion) -> Result<Document, ExternalModelRepositoryError> {
    let mut filter = Document::new();

    if let Some(provider) = &criterion.provider {
        let provider = match provider {
            ModelProvider::HuggingFace => DocumentModelProvider::HuggingFace,
            ModelProvider::Tapis => DocumentModelProvider::Tapis,
        };

        filter.insert("provider", to_bson(&provider).map_err(map_error)?);
    }

    insert_text(
        &mut filter,
        "metadata.derived.name",
        criterion.name.as_deref(),
    );
    insert_text(
        &mut filter,
        "metadata.derived.author",
        criterion.author.as_deref(),
    );
    insert_text(
        &mut filter,
        "metadata.derived.license",
        criterion.license.as_deref(),
    );
    insert_any_text(
        &mut filter,
        "metadata.derived.inference_runtimes",
        &criterion.inference_runtimes,
    );
    insert_any_text(&mut filter, "metadata.derived.tags", &criterion.tags);
    insert_any(
        &mut filter,
        "metadata.derived.task_types",
        &criterion.task_types,
    )?;
    insert_range(&mut filter, "metadata.derived.size", &criterion.size)?;
    insert_range(&mut filter, "metadata.derived.likes", &criterion.likes)?;
    insert_range(
        &mut filter,
        "metadata.derived.downloads",
        &criterion.downloads,
    )?;

    let mut and_conditions = Vec::new();

    let deployment_option_filter = deployment_option_filter(criterion)?;

    if !deployment_option_filter.is_empty() {
        and_conditions
            .push(doc! { "deployment_options": { "$elemMatch": deployment_option_filter } });
    }

    if let Some(has_deployment_options) = criterion.has_deployment_options {
        and_conditions.push(doc! {
            "deployment_options": if has_deployment_options {
                doc! { "$ne": [] }
            } else {
                doc! { "$size": 0 }
            },
        });
    }

    if !and_conditions.is_empty() {
        filter.insert("$and", and_conditions);
    }

    Ok(filter)
}

fn deployment_option_filter(
    criterion: &SearchCriterion,
) -> Result<Document, ExternalModelRepositoryError> {
    let mut filter = Document::new();

    if !criterion.serving_runtimes.is_empty() {
        let runtimes = criterion
            .serving_runtimes
            .iter()
            .map(|runtime| match runtime {
                crate::domain::entities::deployment_option::ServingRuntime::FlexServ => {
                    DocumentServingRuntime::FlexServ
                }
            })
            .collect::<Vec<_>>();

        filter.insert(
            "serving_runtime",
            doc! { "$in": to_bson(&runtimes).map_err(map_error)? },
        );
    }

    if !criterion.hpc_cluster_ids.is_empty() {
        let ids = criterion
            .hpc_cluster_ids
            .iter()
            .map(|id| Uuid::from_bytes(*id.as_bytes()))
            .collect::<Vec<_>>();

        filter.insert("hpc_cluster_queue.hpc_cluster_id", doc! { "$in": ids });
    }

    if !criterion.batch_scheduler_queue_ids.is_empty() {
        let ids = criterion
            .batch_scheduler_queue_ids
            .iter()
            .map(|id| Uuid::from_bytes(*id.as_bytes()))
            .collect::<Vec<_>>();

        filter.insert(
            "hpc_cluster_queue.batch_scheduler_queue_id",
            doc! { "$in": ids },
        );
    }

    if !criterion.supported_deployment_modalities.is_empty() {
        let modalities = criterion
            .supported_deployment_modalities
            .iter()
            .cloned()
            .map(DocumentDeploymentModality::from)
            .collect::<Vec<_>>();

        filter.insert(
            "supported_deployment_modalities",
            doc! { "$in": to_bson(&modalities).map_err(map_error)? },
        );
    }

    Ok(filter)
}

fn uses_deployment_options(criterion: &SearchCriterion) -> bool {
    !criterion.serving_runtimes.is_empty()
        || !criterion.hpc_cluster_ids.is_empty()
        || !criterion.batch_scheduler_queue_ids.is_empty()
        || !criterion.supported_deployment_modalities.is_empty()
        || criterion.has_deployment_options.is_some()
}

fn deployment_option_lookup() -> Document {
    doc! {
        "$lookup": {
            "from": crate::infra::persistence::mongo::database::DEPLOYMENT_OPTION_COLLECTION,
            "localField": "id",
            "foreignField": "external_model_id",
            "as": "deployment_options",
        },
    }
}

fn insert_text(filter: &mut Document, path: &str, value: Option<&str>) {
    if let Some(value) = value {
        filter.insert(path, exact_regex(value));
    }
}

fn insert_any<T: serde::Serialize>(
    filter: &mut Document,
    path: &str,
    values: &[T],
) -> Result<(), ExternalModelRepositoryError> {
    if !values.is_empty() {
        filter.insert(path, doc! { "$in": to_bson(values).map_err(map_error)? });
    }

    Ok(())
}

fn insert_any_text(filter: &mut Document, path: &str, values: &[String]) {
    if !values.is_empty() {
        filter.insert(
            path,
            doc! { "$in": values.iter().map(|value| exact_regex(value)).collect::<Vec<_>>() },
        );
    }
}

fn insert_range<T: Copy + TryInto<i64>>(
    filter: &mut Document,
    path: &str,
    range: &NumericRange<T>,
) -> Result<(), ExternalModelRepositoryError>
where
    <T as TryInto<i64>>::Error: std::fmt::Display,
{
    let mut document = Document::new();

    if let Some(min) = range.min {
        document.insert("$gte", min.try_into().map_err(map_error)?);
    }
    if let Some(max) = range.max {
        document.insert("$lte", max.try_into().map_err(map_error)?);
    }

    if !document.is_empty() {
        filter.insert(path, document);
    }

    Ok(())
}

fn exact_regex(value: &str) -> Bson {
    Bson::RegularExpression(Regex {
        pattern: format!("^{}$", regex::escape(value)),
        options: "i".into(),
    })
}

fn map_error(error: impl std::fmt::Display) -> ExternalModelRepositoryError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] ExternalModel persistence error: {}",
        infrastructure_error.error_id(),
        error
    );

    infrastructure_error.into()
}

fn map_conversion_error(error: impl std::fmt::Display) -> ExternalModelRepositoryError {
    map_error(error)
}

#[cfg(test)]
#[path = "external_model_repository.test.rs"]
mod external_model_repository_test;
