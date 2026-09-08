use crate::{
    application::outputs::dataset::DatasetQueryOutput,
    domain::entities::dataset::{
        Dataset as DomainDataset, DatasetError, DatasetProvider as DomainProvider,
        HuggingFaceRepoLocator,
    },
    infra::persistence::mongo::documents::dataset::{
        Dataset, DatasetProvider, DatasetQuery,
        HuggingFaceRepoLocator as DocumentHuggingFaceLocator, TapisSystemLocator,
    },
    infra::persistence::mongo::documents::visibility::Visibility as DocumentVisibility,
    shared_kernel::enums::Visibility,
};

#[test]
fn document_round_trip_preserves_provider_locator() -> Result<(), Box<dyn std::error::Error>> {
    let domain = DomainDataset::register(
        "tenant".into(),
        "owner".into(),
        "dataset".into(),
        Some("Description".into()),
        Vec::new(),
        DomainProvider::HuggingFace(HuggingFaceRepoLocator::new(
            "owner/repo".into(),
            "abc".into(),
        )?),
        Vec::new(),
        0,
        Visibility::Private,
    )?;

    let document = Dataset::from(&domain);
    let restored = DomainDataset::try_from(document)?;

    assert!(matches!(
        restored.provider(),
        DomainProvider::HuggingFace(_)
    ));
    assert_eq!(restored.name(), "dataset");
    assert_eq!(restored.description(), Some("Description"));

    Ok(())
}

#[test]
fn document_rejects_mismatched_provider_locator() -> Result<(), Box<dyn std::error::Error>> {
    let domain = DomainDataset::register(
        "tenant".into(),
        "owner".into(),
        "dataset".into(),
        None,
        Vec::new(),
        DomainProvider::HuggingFace(HuggingFaceRepoLocator::new(
            "owner/repo".into(),
            "abc".into(),
        )?),
        Vec::new(),
        0,
        Visibility::Private,
    )?;

    let mut document = Dataset::from(&domain);

    document.provider = DatasetProvider::Tapis;
    document.huggingface_repo_locator = None;
    document.tapis_system_locator = Some(TapisSystemLocator {
        site_id: "site".into(),
        tenant_id: "tenant".into(),
        system_id: "system".into(),
        path: "/data".into(),
    });
    document.huggingface_repo_locator = Some(
        crate::infra::persistence::mongo::documents::dataset::HuggingFaceRepoLocator {
            id: "owner/repo".into(),
            sha: "abc".into(),
        },
    );

    assert!(DomainDataset::try_from(document).is_err());

    Ok(())
}

#[test]
fn query_document_rejects_mismatched_provider_locator() {
    let document = DatasetQuery {
        _id: None,
        id: mongodb::bson::Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes()),
        tenant_id: "tenant".into(),
        owner: "owner".into(),
        name: "dataset".into(),
        description: None,
        tags: Vec::new(),
        provider: DatasetProvider::Tapis,
        huggingface_repo_locator: Some(DocumentHuggingFaceLocator {
            id: "owner/repo".into(),
            sha: "abc".into(),
        }),
        tapis_system_locator: None,
        items: Vec::new(),
        item_count: 0,
        size: 0,
        visibility: DocumentVisibility::Private,
    };

    assert!(DatasetQueryOutput::try_from(document).is_err());
}

#[test]
fn query_document_rejects_an_empty_name() {
    let document = DatasetQuery {
        _id: None,
        id: mongodb::bson::Uuid::from_bytes(*uuid::Uuid::now_v7().as_bytes()),
        tenant_id: "tenant".into(),
        owner: "owner".into(),
        name: String::new(),
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
        visibility: DocumentVisibility::Private,
    };

    assert!(matches!(
        DatasetQueryOutput::try_from(document),
        Err(DatasetError::DataIntegrityError(_))
    ));
}
