use mongodb::bson::{oid::ObjectId, DateTime, Uuid};

use super::{documents_to_page, list_filter, list_pipeline};
use crate::{
    application::{
        inputs::deployment_option::ListDeploymentOptionsInput,
        ports::deployment_option::DeploymentOptionRepositoryError,
    },
    domain::entities::model::external_model::ExternalModelId,
    infra::persistence::mongo::documents::{
        deployment::DeploymentModality,
        deployment_option::{
            DeploymentOption, DeploymentTargetType, HpcClusterQueueReference, ServingRuntime,
        },
    },
};

fn document(id: ObjectId, external_model_id: Uuid) -> DeploymentOption {
    DeploymentOption {
        _id: Some(id),
        id: Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes()),
        external_model_id,
        supported_deployment_modalities: vec![DeploymentModality::Batch],
        deployment_target_type: DeploymentTargetType::HpcClusterQueue,
        hpc_cluster_queue: Some(HpcClusterQueueReference {
            hpc_cluster_id: Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes()),
            batch_scheduler_queue_id: Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes()),
        }),
        serving_runtime: ServingRuntime::FlexServ,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    }
}

#[test]
fn rejects_invalid_cursor() {
    let external_model_id = ExternalModelId::new();

    let input = ListDeploymentOptionsInput::new(None, Some("invalid".into()), None);

    let result = list_filter(&external_model_id, &input);

    assert!(matches!(
        result,
        Err(DeploymentOptionRepositoryError::InvalidCursor)
    ));
}

#[test]
fn pipeline_filters_orders_and_fetches_one_extra_document() -> Result<(), Box<dyn std::error::Error>>
{
    let external_model_id = ExternalModelId::new();

    let cursor = ObjectId::new();

    let input = ListDeploymentOptionsInput::new(Some(25), Some(cursor.to_hex()), None);

    let filter = list_filter(&external_model_id, &input)?;

    let pipeline = list_pipeline(filter, &input);

    assert_eq!(pipeline[1], mongodb::bson::doc! { "$sort": { "_id": 1 } });
    assert_eq!(pipeline[2], mongodb::bson::doc! { "$limit": 26_i64 });
    assert_eq!(
        pipeline[0]["$match"]["_id"]["$gt"],
        mongodb::bson::Bson::ObjectId(cursor)
    );

    Ok(())
}

#[test]
fn builds_page_and_cursor_in_document_order() -> Result<(), Box<dyn std::error::Error>> {
    let external_model_id = Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes());

    let first_id = ObjectId::new();

    let second_id = ObjectId::new();

    let (options, cursor) = documents_to_page(
        vec![
            document(first_id, external_model_id),
            document(second_id, external_model_id),
        ],
        1,
    )?;

    assert_eq!(options.len(), 1);
    assert_eq!(cursor, Some(first_id.to_hex()));

    Ok(())
}
