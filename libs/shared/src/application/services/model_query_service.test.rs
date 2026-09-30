use std::sync::Arc;

use async_trait::async_trait;

use super::*;
use crate::{
    application::{
        inputs::discover_models::SearchExternalModelsInput,
        ports::model::{ExternalModelPage, ModelRepository},
    },
    domain::entities::model::{
        external_model::{ExternalModel, ModelLocator, ModelProvider},
        fixtures::full_external_model,
        ModelError,
    },
    shared_kernel::{constants::GLOBAL_TENANT, identifiers::ExternalModelId},
};

struct TestModelRepository {
    found: Option<Model>,
    page: Vec<Model>,
}

#[async_trait]
impl ModelRepository for TestModelRepository {
    async fn save(&self, _model: &Model) -> Result<(), ModelRepositoryError> {
        Ok(())
    }

    async fn update(&self, _model: &Model) -> Result<(), ModelRepositoryError> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        _tenant_id: &str,
        _id: Uuid,
    ) -> Result<Option<Model>, ModelRepositoryError> {
        Ok(self.found.clone())
    }

    async fn find_by_external_model_id(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _external_model_id: &ExternalModelId,
    ) -> Result<Option<Model>, ModelRepositoryError> {
        Ok(None)
    }

    async fn find_by_artifact_id(
        &self,
        _artifact_id: &Uuid,
    ) -> Result<Option<Model>, ModelRepositoryError> {
        Ok(None)
    }

    async fn list_by_owner(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError> {
        Ok(ModelPage {
            models: self.page.clone(),
            count: Some(self.page.len() as u64),
            cursor: None,
        })
    }

    async fn list_shared(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _input: &ListModelsInput,
    ) -> Result<ModelPage, ModelRepositoryError> {
        Ok(ModelPage {
            models: self.page.clone(),
            count: Some(self.page.len() as u64),
            cursor: None,
        })
    }
}

struct TestExternalModelRepository {
    models: Vec<ExternalModel>,
}

#[async_trait]
impl ExternalModelRepository for TestExternalModelRepository {
    async fn save(&self, _model: &ExternalModel) -> Result<(), ExternalModelRepositoryError> {
        Ok(())
    }

    async fn update(&self, _model: &ExternalModel) -> Result<(), ExternalModelRepositoryError> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ExternalModelId,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(self.models.iter().find(|model| model.id() == id).cloned())
    }

    async fn find_by_ids(
        &self,
        ids: &[ExternalModelId],
    ) -> Result<Vec<ExternalModel>, ExternalModelRepositoryError> {
        Ok(self
            .models
            .iter()
            .filter(|model| ids.contains(model.id()))
            .cloned()
            .collect())
    }

    async fn find_by_provider_and_locator(
        &self,
        _provider: &ModelProvider,
        _locator: &ModelLocator,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(None)
    }

    async fn search(
        &self,
        _input: &SearchExternalModelsInput,
    ) -> Result<ExternalModelPage, ExternalModelRepositoryError> {
        Ok(ExternalModelPage {
            external_models: self.models.clone(),
            count: None,
            cursor: None,
        })
    }
}

#[tokio::test]
async fn private_model_owned_by_another_principal_is_concealed(
) -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let model = owned_model("another-owner", Visibility::Private, *external_model.id())?;

    let service = service(Some(model), Vec::new(), vec![external_model]);

    let result = service
        .get_model(&RequestContext::system(None), Uuid::now_v7())
        .await;

    assert!(matches!(result, Err(ModelQueryServiceError::NotFound)));

    Ok(())
}

#[tokio::test]
async fn public_model_is_hydrated_for_a_non_owner() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let model = owned_model("another-owner", Visibility::Public, *external_model.id())?;

    let service = service(Some(model), Vec::new(), vec![external_model.clone()]);

    let output = service
        .get_model(&RequestContext::system(None), Uuid::now_v7())
        .await?;

    assert_eq!(output.external_model.id(), external_model.id());

    Ok(())
}

#[tokio::test]
async fn dangling_external_reference_is_a_data_integrity_error(
) -> Result<(), Box<dyn std::error::Error>> {
    let model = owned_model("mlhub", Visibility::Private, ExternalModelId::new())?;

    let service = service(None, vec![model], Vec::new());

    let result = service
        .list_owned(
            &RequestContext::system(None),
            &ListModelsInput::new(None, None, None),
        )
        .await;

    assert!(matches!(
        result,
        Err(ModelQueryServiceError::MissingExternalModel)
    ));

    Ok(())
}

fn service(
    found: Option<Model>,
    page: Vec<Model>,
    external_models: Vec<ExternalModel>,
) -> ModelQueryService {
    ModelQueryService::new(
        Arc::new(TestModelRepository { found, page }),
        Arc::new(TestExternalModelRepository {
            models: external_models,
        }),
    )
}

fn owned_model(
    owner: &str,
    visibility: Visibility,
    external_model_id: ExternalModelId,
) -> Result<Model, ModelError> {
    Model::create(
        GLOBAL_TENANT.into(),
        owner.into(),
        "model".into(),
        None,
        external_model_id,
        visibility,
    )
}
