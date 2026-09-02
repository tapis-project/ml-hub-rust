use crate::{
    application::outputs::dataset::DatasetQueryOutput,
    domain::entities::dataset::{
        Dataset as DomainDataset, DatasetItem as DomainDatasetItem,
        DatasetProvider as DomainProvider, HuggingFaceRepoLocator,
    },
    presentation::http::v1::responses::datasets::{Dataset, DatasetProvider},
    shared_kernel::{enums::Visibility, value_objects::Tags},
};
use uuid::Uuid;

#[test]
fn response_populates_only_provider_selected_locator() -> Result<(), Box<dyn std::error::Error>> {
    let domain = DomainDataset::register(
        "tenant".into(),
        "owner".into(),
        Vec::new(),
        DomainProvider::HuggingFace(HuggingFaceRepoLocator::new(
            "owner/repo".into(),
            "abc".into(),
        )?),
        Vec::new(),
        0,
        Visibility::Private,
    )?;

    let response = Dataset::from(domain);

    assert!(matches!(response.provider, DatasetProvider::HuggingFace));
    assert!(response.huggingface_repo_locator.is_some());
    assert!(response.tapis_system_locator.is_none());
    assert_eq!(response.item_count, 0);

    Ok(())
}

#[test]
fn registration_response_keeps_every_item() -> Result<(), Box<dyn std::error::Error>> {
    let items = dataset_items(51)?;
    let domain = DomainDataset::register(
        "tenant".into(),
        "owner".into(),
        Vec::new(),
        DomainProvider::HuggingFace(HuggingFaceRepoLocator::new(
            "owner/repo".into(),
            "abc".into(),
        )?),
        items,
        51,
        Visibility::Private,
    )?;

    let response = Dataset::from(domain);

    assert_eq!(response.items.len(), 51);
    assert_eq!(response.item_count, 51);

    Ok(())
}

#[test]
fn query_response_uses_mongodb_projected_items_and_complete_count(
) -> Result<(), Box<dyn std::error::Error>> {
    let output = DatasetQueryOutput {
        id: Uuid::now_v7(),
        tenant_id: "tenant".into(),
        owner: "owner".into(),
        tags: Tags::reconstitute(Vec::new())?,
        provider: DomainProvider::HuggingFace(HuggingFaceRepoLocator::reconstitute(
            "owner/repo".into(),
            "abc".into(),
        )?),
        items: dataset_items(50)?,
        item_count: 51,
        size: 51,
        visibility: Visibility::Private,
    };

    let response = Dataset::from(output);

    assert_eq!(response.items.len(), 50);
    assert_eq!(response.item_count, 51);
    assert_eq!(
        response.items.first().map(|item| item.path.as_str()),
        Some("item-0")
    );
    assert_eq!(
        response.items.last().map(|item| item.path.as_str()),
        Some("item-49")
    );

    Ok(())
}

fn dataset_items(count: usize) -> Result<Vec<DomainDatasetItem>, Box<dyn std::error::Error>> {
    (0..count)
        .map(|index| DomainDatasetItem::new(format!("item-{index}"), 1).map_err(Into::into))
        .collect()
}
