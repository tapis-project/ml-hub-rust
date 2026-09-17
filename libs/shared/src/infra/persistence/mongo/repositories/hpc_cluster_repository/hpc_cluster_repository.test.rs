use mongodb::bson::{doc, oid::ObjectId, Uuid};

use super::*;

#[test]
fn list_pipeline_scopes_to_data_center_and_projects_summary(
) -> Result<(), Box<dyn std::error::Error>> {
    let input = ListHpcClustersInput::new(DomainDataCenter::Tacc, Some(25), None, None);

    let pipeline = list_pipeline(list_filter(&input)?, &input)?;

    assert_eq!(pipeline[0], doc! { "$match": { "data_center": "Tacc" } });
    assert_eq!(pipeline[2], doc! { "$limit": 26_i64 });
    assert_eq!(
        pipeline[3]["$project"]["name"],
        mongodb::bson::Bson::Int32(1)
    );

    Ok(())
}

#[test]
fn list_pipeline_applies_cursor() -> Result<(), Box<dyn std::error::Error>> {
    let cursor = ObjectId::new();
    let input =
        ListHpcClustersInput::new(DomainDataCenter::Tacc, None, Some(cursor.to_hex()), None);

    let pipeline = list_pipeline(list_filter(&input)?, &input)?;

    assert_eq!(
        pipeline[0],
        doc! { "$match": { "data_center": "Tacc", "_id": { "$gt": cursor } } }
    );

    Ok(())
}

#[test]
fn summary_page_returns_cursor_only_when_more_results_exist() {
    let first_id = ObjectId::new();
    let second_id = ObjectId::new();
    let documents = vec![
        HpcClusterSummary {
            _id: first_id,
            id: Uuid::new(),
            enabled: true,
            name: "Vista".into(),
            data_center: DataCenter::Tacc,
        },
        HpcClusterSummary {
            _id: second_id,
            id: Uuid::new(),
            enabled: false,
            name: "Frontera".into(),
            data_center: DataCenter::Tacc,
        },
    ];

    let (summaries, cursor) = summaries_to_page(documents, 1);

    assert_eq!(summaries.len(), 1);
    assert!(summaries[0].enabled);
    assert_eq!(cursor, Some(first_id.to_hex()));
}
