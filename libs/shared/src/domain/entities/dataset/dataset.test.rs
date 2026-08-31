use crate::domain::entities::dataset::{
    Dataset, DatasetError, DatasetItem, DatasetProvider, HuggingFaceRepoLocator,
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
        vec!["training".into()],
        provider()?,
        vec![DatasetItem::new("data.json".into(), 10)?],
        10,
        Visibility::Private,
    )?;

    assert_eq!(dataset.id().get_version_num(), 7);

    assert_eq!(
        dataset.urn().as_str(),
        format!("urn:mlhub:v1:tenant-a:dataset:{}", dataset.id())
    );

    Ok(())
}

#[test]
fn register_dataset_requires_exact_item_size_sum() -> Result<(), Box<dyn std::error::Error>> {
    let result = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        Vec::new(),
        provider()?,
        vec![DatasetItem::new("data.json".into(), 10)?],
        11,
        Visibility::Private,
    );

    assert!(matches!(
        result,
        Err(DatasetError::SizeMismatch {
            declared: 11,
            calculated: 10
        })
    ));

    Ok(())
}

#[test]
fn register_dataset_allows_empty_zero_size_dataset() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        Vec::new(),
        provider()?,
        Vec::new(),
        0,
        Visibility::Private,
    )?;

    assert!(dataset.items().is_empty());
    assert_eq!(dataset.size(), 0);

    Ok(())
}

#[test]
fn register_dataset_rejects_duplicate_item_paths() -> Result<(), Box<dyn std::error::Error>> {
    let result = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
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
fn register_dataset_rejects_item_size_overflow() -> Result<(), Box<dyn std::error::Error>> {
    let result = Dataset::register(
        "tenant-a".into(),
        "owner-a".into(),
        Vec::new(),
        provider()?,
        vec![
            DatasetItem::new("a".into(), u64::MAX)?,
            DatasetItem::new("b".into(), 1)?,
        ],
        u64::MAX,
        Visibility::Private,
    );

    assert!(matches!(result, Err(DatasetError::ItemSizeOverflow)));

    Ok(())
}
