use super::*;
use shared::application::inputs::dataset::{DatasetProviderInput, VisibilityInput};

fn record() -> HuggingFaceDatasetRecord {
    HuggingFaceDatasetRecord {
        id: "owner/dataset".into(),
        sha: "abc123".into(),
        tags: vec!["text".into()],
        private: false,
        gated: false,
        siblings: vec![
            HuggingFaceRepoSibling {
                rfilename: "data/train.parquet".into(),
                size: Some(10),
            },
            HuggingFaceRepoSibling {
                rfilename: "README.md".into(),
                size: Some(2),
            },
        ],
    }
}

#[test]
fn transforms_open_dataset_snapshot() -> Result<(), TransformDatasetError> {
    let input = RegisterDatasetInput::try_from(record())?;

    assert_eq!(input.size, 12);
    assert_eq!(input.items.len(), 2);
    assert!(matches!(input.visibility, VisibilityInput::Public));
    assert!(matches!(
        input.provider,
        DatasetProviderInput::HuggingFace(locator)
            if locator.id == "owner/dataset" && locator.sha == "abc123"
    ));

    Ok(())
}

#[test]
fn filters_deduplicates_and_caps_provider_tags() -> Result<(), TransformDatasetError> {
    let mut value = record();
    value.tags = (0..=MAX_TAGS)
        .map(|index| format!("tag-{index}"))
        .chain([
            "tag-0".into(),
            String::new(),
            "x".repeat(MAX_TAG_LENGTH_BYTES + 1),
        ])
        .collect();

    let input = RegisterDatasetInput::try_from(value)?;

    assert_eq!(input.tags.len(), MAX_TAGS);
    assert_eq!(input.tags[0], "tag-0");
    assert_eq!(input.tags[MAX_TAGS - 1], format!("tag-{}", MAX_TAGS - 1));

    Ok(())
}

#[test]
fn rejects_private_and_gated_datasets() {
    let mut private = record();
    private.private = true;

    let mut gated = record();
    gated.gated = true;

    assert_eq!(
        RegisterDatasetInput::try_from(private).err(),
        Some(TransformDatasetError::Private)
    );
    assert_eq!(
        RegisterDatasetInput::try_from(gated).err(),
        Some(TransformDatasetError::Gated)
    );
}

#[test]
fn rejects_items_without_sizes_and_size_overflow() {
    let mut missing = record();
    missing.siblings[0].size = None;

    let mut overflowing = record();
    overflowing.siblings[0].size = Some(u64::MAX);
    overflowing.siblings[1].size = Some(1);

    assert_eq!(
        RegisterDatasetInput::try_from(missing).err(),
        Some(TransformDatasetError::MissingItemSize(
            "data/train.parquet".into()
        ))
    );
    assert_eq!(
        RegisterDatasetInput::try_from(overflowing).err(),
        Some(TransformDatasetError::ItemSizeOverflow)
    );
}
