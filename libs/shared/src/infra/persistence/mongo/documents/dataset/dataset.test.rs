use crate::{
    domain::entities::dataset::{
        Dataset as DomainDataset, DatasetProvider as DomainProvider, HuggingFaceRepoLocator,
    },
    infra::persistence::mongo::documents::dataset::{Dataset, DatasetProvider, TapisSystemLocator},
    shared_kernel::enums::Visibility,
};

#[test]
fn document_round_trip_preserves_provider_locator() -> Result<(), Box<dyn std::error::Error>> {
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

    let document = Dataset::from(&domain);
    let restored = DomainDataset::try_from(document)?;

    assert!(matches!(
        restored.provider(),
        DomainProvider::HuggingFace(_)
    ));

    Ok(())
}

#[test]
fn document_rejects_mismatched_provider_locator() -> Result<(), Box<dyn std::error::Error>> {
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
