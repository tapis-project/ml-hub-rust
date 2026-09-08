use crate::domain::entities::dataset::{
    Dataset, DatasetError, DatasetItem, DatasetProvider, HuggingFaceRepoLocator,
    ReconstituteDatasetProps,
};
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::identifiers::traits::UrnGenerator;

fn provider() -> Result<DatasetProvider, Box<dyn std::error::Error>> {
    Ok(DatasetProvider::HuggingFace(HuggingFaceRepoLocator::new(
        "owner/repository".into(),
        "abc123".into(),
    )?))
}

#[test]
fn register_dataset_generates_v7_identity_and_urn() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        "dataset-a".into(),
        Some("Description".into()),
        vec!["training".into()],
        provider()?,
        vec![DatasetItem::new("data.json".into(), 10)?],
        10,
        Visibility::Private,
    )?;

    assert_eq!(dataset.id().get_version_num(), 7);
    assert_eq!(dataset.name(), "dataset-a");
    assert_eq!(dataset.description(), Some("Description"));

    assert_eq!(
        dataset.urn().as_str(),
        format!("urn:mlhub:v1:tenant-a:dataset:{}", dataset.id())
    );

    Ok(())
}

#[test]
fn register_dataset_preserves_a_declared_size_that_differs_from_item_sizes(
) -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        "dataset-a".into(),
        None,
        Vec::new(),
        provider()?,
        vec![DatasetItem::new("data.json".into(), 10)?],
        11,
        Visibility::Private,
    )?;

    assert_eq!(dataset.size(), 11);

    Ok(())
}

#[test]
fn register_dataset_allows_an_empty_dataset_with_a_nonzero_size(
) -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        "dataset-a".into(),
        None,
        Vec::new(),
        provider()?,
        Vec::new(),
        10,
        Visibility::Private,
    )?;

    assert!(dataset.items().is_empty());
    assert_eq!(dataset.size(), 10);

    Ok(())
}

#[test]
fn register_dataset_rejects_duplicate_item_paths() -> Result<(), Box<dyn std::error::Error>> {
    let result = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        "dataset-a".into(),
        None,
        Vec::new(),
        provider()?,
        vec![
            DatasetItem::new("data.json".into(), 5)?,
            DatasetItem::new("data.json".into(), 5)?,
        ],
        10,
        Visibility::Private,
    );

    assert!(matches!(result, Err(DatasetError::DuplicateItemPath(path)) if path == "data.json"));

    Ok(())
}

#[test]
fn register_dataset_does_not_sum_item_sizes() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        "dataset-a".into(),
        None,
        Vec::new(),
        provider()?,
        vec![
            DatasetItem::new("a".into(), u64::MAX)?,
            DatasetItem::new("b".into(), 1)?,
        ],
        u64::MAX,
        Visibility::Private,
    )?;

    assert_eq!(dataset.items().len(), 2);
    assert_eq!(dataset.size(), u64::MAX);

    Ok(())
}

#[test]
fn reconstitute_preserves_a_declared_size_that_differs_from_item_sizes(
) -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::reconstitute(ReconstituteDatasetProps {
        id: uuid::Uuid::now_v7(),
        tenant_id: "tenant-a".into(),
        owner: "owner-a".into(),
        name: "dataset-a".into(),
        description: None,
        tags: Vec::new(),
        provider: provider()?,
        items: vec![DatasetItem::reconstitute("data.json".into(), 10)?],
        size: 11,
        visibility: Visibility::Private,
    })?;

    assert_eq!(dataset.size(), 11);

    Ok(())
}

#[test]
fn register_dataset_rejects_an_empty_name() -> Result<(), Box<dyn std::error::Error>> {
    let result = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        String::new(),
        None,
        Vec::new(),
        provider()?,
        Vec::new(),
        0,
        Visibility::Private,
    );

    assert!(matches!(result, Err(DatasetError::EmptyName)));

    Ok(())
}

#[test]
fn reconstitute_rejects_an_empty_name_as_a_data_integrity_error(
) -> Result<(), Box<dyn std::error::Error>> {
    let result = Dataset::reconstitute(ReconstituteDatasetProps {
        id: uuid::Uuid::now_v7(),
        tenant_id: "tenant-a".into(),
        owner: "owner-a".into(),
        name: String::new(),
        description: None,
        tags: Vec::new(),
        provider: provider()?,
        items: Vec::new(),
        size: 0,
        visibility: Visibility::Private,
    });

    assert!(matches!(result, Err(DatasetError::DataIntegrityError(_))));

    Ok(())
}
