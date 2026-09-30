use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use super::*;
use crate::{
    application::{
        inputs::{discover_models::SearchExternalModelsInput, model::ListModelsInput},
        ports::model::{ExternalModelPage, ModelPage},
    },
    domain::entities::model::{
        external_model::{ExternalModel, ModelLocator, ModelProvider},
        fixtures::full_external_model,
    },
    shared_kernel::enums::Visibility,
};

#[derive(Default)]
struct TestModelRepository {
    existing: Mutex<Option<Model>>,
    save_error: Mutex<bool>,
    saved: Mutex<Vec<Model>>,
}

#[async_trait]
impl ModelRepository for TestModelRepository {
    async fn save(&self, model: &Model) -> Result<(), ModelRepositoryError> {
        if *self
            .save_error
            .lock()
            .unwrap_or_else(|error| error.into_inner())
        {
            return Err(ModelRepositoryError::ModelAlreadyInCollection);
        }

        self.saved
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push(model.clone());

        Ok(())
    }

    async fn update(&self, _model: &Model) -> Result<(), ModelRepositoryError> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        _tenant_id: &str,
        _id: uuid::Uuid,
    ) -> Result<Option<Model>, ModelRepositoryError> {
        Ok(None)
    }

    async fn find_by_external_model_id(
        &self,
        _tenant_id: &str,
        _owner: &str,
        _external_model_id: &crate::shared_kernel::identifiers::ExternalModelId,
    ) -> Result<Option<Model>, ModelRepositoryError> {
        Ok(self
            .existing
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .clone())
    }

    async fn find_by_artifact_id(
        &self,
        _artifact_id: &uuid::Uuid,
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
            models: Vec::new(),
            count: None,
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
            models: Vec::new(),
            count: None,
            cursor: None,
        })
    }
}

struct TestExternalModelRepository {
    found: Option<ExternalModel>,
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
        _id: &crate::shared_kernel::identifiers::ExternalModelId,
    ) -> Result<Option<ExternalModel>, ExternalModelRepositoryError> {
        Ok(self.found.clone())
    }

    async fn find_by_ids(
        &self,
        _ids: &[crate::shared_kernel::identifiers::ExternalModelId],
    ) -> Result<Vec<ExternalModel>, ExternalModelRepositoryError> {
        Ok(Vec::new())
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
            external_models: Vec::new(),
            count: None,
            cursor: None,
        })
    }
}

#[tokio::test]
async fn missing_external_model_is_rejected() {
    let service = ModelCreationService::new(
        Arc::new(TestModelRepository::default()),
        Arc::new(TestExternalModelRepository { found: None }),
    );

    let context = RequestContext::system(None);

    let result = service
        .create_model(
            &context,
            input(crate::shared_kernel::identifiers::ExternalModelId::new()),
        )
        .await;

    assert!(matches!(
        result,
        Err(ModelCreationServiceError::ExternalModelNotFound)
    ));
}

#[tokio::test]
async fn existing_pairing_returns_collection_conflict() -> Result<(), Box<dyn std::error::Error>> {
    let external_model = full_external_model();

    let existing = Model::create(
        "__GLOBAL__".into(),
        "mlhub".into(),
        "existing".into(),
        None,
        *external_model.id(),
        Visibility::Private,
    )?;

    let model_repository = Arc::new(TestModelRepository {
        existing: Mutex::new(Some(existing)),
        ..Default::default()
    });

    let service = ModelCreationService::new(
        model_repository,
        Arc::new(TestExternalModelRepository {
            found: Some(external_model.clone()),
        }),
    );

    let result = service
        .create_model(&RequestContext::system(None), input(*external_model.id()))
        .await;

    assert!(matches!(
        result,
        Err(ModelCreationServiceError::ModelAlreadyInCollection)
    ));

    Ok(())
}

#[tokio::test]
async fn duplicate_key_race_maps_to_collection_conflict() {
    let external_model = full_external_model();

    let model_repository = Arc::new(TestModelRepository {
        save_error: Mutex::new(true),
        ..Default::default()
    });

    let service = ModelCreationService::new(
        model_repository,
        Arc::new(TestExternalModelRepository {
            found: Some(external_model.clone()),
        }),
    );

    let result = service
        .create_model(&RequestContext::system(None), input(*external_model.id()))
        .await;

    assert!(matches!(
        result,
        Err(ModelCreationServiceError::ModelAlreadyInCollection)
    ));
}

fn input(
    external_model_id: crate::shared_kernel::identifiers::ExternalModelId,
) -> CreateModelInput {
    CreateModelInput {
        name: "my model".into(),
        description: None,
        external_model_id,
        visibility: Visibility::Private,
    }
}
