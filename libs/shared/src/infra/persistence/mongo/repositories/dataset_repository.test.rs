use super::{
    dataset_id_filter, dataset_query_pipeline, owner_filter, shared_filter, tenant_filter,
    DATASET_QUERY_ITEM_LIMIT,
};
use crate::shared_kernel::constants::GLOBAL_TENANT;
use mongodb::bson::{doc, Bson};

#[test]
fn dataset_query_pipeline_limits_items_and_counts_the_complete_array(
) -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = dataset_query_pipeline(doc! { "tenant_id": "tenant" }, false);
    let projection = pipeline
        .get(1)
        .and_then(|stage| stage.get_document("$project").ok())
        .ok_or_else(|| std::io::Error::other("Dataset query should contain a projection"))?;

    assert_eq!(
        projection.get("items"),
        Some(&Bson::Document(doc! {
            "$slice": ["$items", DATASET_QUERY_ITEM_LIMIT]
        }))
    );
    assert_eq!(
        projection.get("item_count"),
        Some(&Bson::Document(doc! {
            "$toLong": { "$size": "$items" }
        }))
    );

    Ok(())
}

#[test]
fn single_dataset_query_limits_results_after_tenant_scoped_match(
) -> Result<(), Box<dyn std::error::Error>> {
    let id = mongodb::bson::Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes());
    let filter = dataset_id_filter("tenant", id);
    let pipeline = dataset_query_pipeline(filter.clone(), true);

    assert_eq!(pipeline.first(), Some(&doc! { "$match": filter }));
    assert_eq!(pipeline.last(), Some(&doc! { "$limit": 1 }));

    Ok(())
}

#[test]
fn dataset_list_filters_are_tenant_scoped() {
    assert_eq!(
        owner_filter("tenant", "owner"),
        doc! { "tenant_id": "tenant", "owner": "owner" }
    );
    assert_eq!(
        shared_filter("tenant", Bson::String("Public".into())),
        doc! { "tenant_id": "tenant", "visibility": "Public" }
    );
    assert_eq!(
        tenant_filter(GLOBAL_TENANT),
        doc! { "tenant_id": GLOBAL_TENANT }
    );
}

#[test]
fn global_dataset_pipeline_keeps_the_item_projection() {
    let pipeline = dataset_query_pipeline(tenant_filter(GLOBAL_TENANT), false);

    assert_eq!(
        pipeline.first(),
        Some(&doc! { "$match": { "tenant_id": GLOBAL_TENANT } })
    );
    assert!(pipeline.get(1).is_some_and(|stage| {
        stage
            .get_document("$project")
            .is_ok_and(|projection| projection.contains_key("items"))
    }));
}
