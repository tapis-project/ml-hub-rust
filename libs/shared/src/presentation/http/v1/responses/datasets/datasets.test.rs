use crate::{
    domain::entities::dataset::{
        Dataset as DomainDataset, DatasetProvider as DomainProvider, HuggingFaceRepoLocator,
    },
    presentation::http::v1::responses::datasets::{Dataset, DatasetProvider},
    shared_kernel::enums::Visibility,
};

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

    Ok(())
}
