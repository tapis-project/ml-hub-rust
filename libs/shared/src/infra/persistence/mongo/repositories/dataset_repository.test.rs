use super::{
    dataset_documents_to_page, dataset_id_filter, dataset_list_pipeline, dataset_query_pipeline,
    owner_filter, shared_filter, tenant_filter, DATASET_QUERY_ITEM_LIMIT,
};
use crate::{
    application::inputs::dataset::ListDatasetsInput,
    infra::persistence::mongo::documents::{
        dataset::{
            DatasetProvider, DatasetQuery, HuggingFaceRepoLocator as DocumentHuggingFaceLocator,
        },
        visibility::Visibility as DocumentVisibility,
    },
    shared_kernel::constants::GLOBAL_TENANT,
};
use mongodb::bson::{doc, oid::ObjectId, Bson};

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
    assert_eq!(projection.get("name"), Some(&Bson::Int32(1)));
    assert_eq!(projection.get("description"), Some(&Bson::Int32(1)));

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
fn global_dataset_pipeline_applies_cursor_sort_limit_and_item_projection(
) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = "000000000000000000000001";
    let input = ListDatasetsInput::new(Some(25), Some(cursor.into()), Some(false));
    let pipeline = dataset_list_pipeline(tenant_filter(GLOBAL_TENANT), &input)?;
    let cursor = ObjectId::parse_str(cursor)?;

    assert_eq!(
        pipeline.first(),
        Some(&doc! {
            "$match": {
                "tenant_id": GLOBAL_TENANT,
                "_id": { "$gt": cursor },
            }
        })
    );
    assert_eq!(pipeline.get(1), Some(&doc! { "$sort": { "_id": 1 } }));
    assert_eq!(pipeline.get(2), Some(&doc! { "$limit": 26_i64 }));
    assert!(pipeline.get(3).is_some_and(|stage| {
        stage
            .get_document("$project")
            .is_ok_and(|projection| projection.contains_key("items"))
    }));

    Ok(())
}

#[test]
fn dataset_page_returns_a_cursor_only_when_an_extra_document_exists(
) -> Result<(), Box<dyn std::error::Error>> {
    let first_id = ObjectId::parse_str("000000000000000000000001")?;
    let second_id = ObjectId::parse_str("000000000000000000000002")?;

    let (datasets, cursor) =
        dataset_documents_to_page(vec![query_document(first_id), query_document(second_id)], 1)?;

    assert_eq!(datasets.len(), 1);
    assert_eq!(cursor, Some(first_id.to_hex()));

    let (datasets, cursor) = dataset_documents_to_page(vec![query_document(first_id)], 1)?;

    assert_eq!(datasets.len(), 1);
    assert!(cursor.is_none());

    Ok(())
}

fn query_document(id: ObjectId) -> DatasetQuery {
    DatasetQuery {
        _id: Some(id),
        id: mongodb::bson::Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes()),
        tenant_id: "tenant".into(),
        owner: "owner".into(),
        name: "dataset".into(),
        description: None,
        tags: Vec::new(),
        provider: DatasetProvider::HuggingFace,
        huggingface_repo_locator: Some(DocumentHuggingFaceLocator {
            id: "owner/repo".into(),
            sha: "abc".into(),
        }),
        tapis_system_locator: None,
        items: Vec::new(),
        item_count: 0,
        size: 0,
        visibility: DocumentVisibility::Public,
    }
}
