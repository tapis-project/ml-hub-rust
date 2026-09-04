use super::{DatasetProvider, ListDatasetsQueryParams, RegisterDatasetBody, Scope};
use crate::application::inputs::dataset::{ListDatasetsInput, RegisterDatasetInput};
use validator::Validate;

fn body_json(provider: &str, locators: &str) -> String {
    format!(
        r#"{{"name":"dataset","description":"Description","provider":"{provider}",{locators}"items":[{{"path":"data.json","size":10}}],"size":10}}"#
    )
}

#[test]
fn request_accepts_matching_huggingface_locator() -> Result<(), Box<dyn std::error::Error>> {
    let body: RegisterDatasetBody = serde_json::from_str(&body_json(
        "HuggingFace",
        r#""huggingface_repo_locator":{"id":"owner/repo","sha":"abc"},"#,
    ))?;

    body.validate()?;

    assert!(matches!(body.provider, DatasetProvider::HuggingFace));

    let input = RegisterDatasetInput::try_from(body)?;

    assert_eq!(input.name, "dataset");
    assert_eq!(input.description.as_deref(), Some("Description"));

    Ok(())
}

#[test]
fn request_rejects_provider_locator_mismatch() -> Result<(), Box<dyn std::error::Error>> {
    let body: RegisterDatasetBody = serde_json::from_str(&body_json(
        "Tapis",
        r#""huggingface_repo_locator":{"id":"owner/repo","sha":"abc"},"#,
    ))?;

    assert!(body.validate().is_err());

    Ok(())
}

#[test]
fn request_rejects_duplicate_item_paths() -> Result<(), Box<dyn std::error::Error>> {
    let body: RegisterDatasetBody = serde_json::from_str(
        r#"{"name":"dataset","provider":"HuggingFace","huggingface_repo_locator":{"id":"owner/repo","sha":"abc"},"items":[{"path":"a","size":1},{"path":"a","size":1}],"size":2}"#,
    )?;

    assert!(body.validate().is_err());

    Ok(())
}

#[test]
fn request_requires_a_nonempty_name() -> Result<(), Box<dyn std::error::Error>> {
    let missing = serde_json::from_str::<RegisterDatasetBody>(
        r#"{"provider":"HuggingFace","huggingface_repo_locator":{"id":"owner/repo","sha":"abc"},"items":[],"size":0}"#,
    );

    assert!(missing.is_err());

    let body: RegisterDatasetBody = serde_json::from_str(
        r#"{"name":"","provider":"HuggingFace","huggingface_repo_locator":{"id":"owner/repo","sha":"abc"},"items":[],"size":0}"#,
    )?;

    assert!(body.validate().is_err());

    Ok(())
}

#[test]
fn list_query_accepts_global_scope() -> Result<(), Box<dyn std::error::Error>> {
    let query: ListDatasetsQueryParams = serde_json::from_str(r#"{"scope":"Global"}"#)?;

    assert!(matches!(query.scope, Scope::Global));

    Ok(())
}

#[test]
fn list_query_uses_default_pagination_options() -> Result<(), Box<dyn std::error::Error>> {
    let query: ListDatasetsQueryParams = serde_json::from_str(r#"{}"#)?;
    let input = ListDatasetsInput::from(&query);

    assert_eq!(input.limit(), ListDatasetsInput::DEFAULT_LIMIT);
    assert!(input.cursor().is_none());
    assert!(!input.include_count());

    Ok(())
}

#[test]
fn list_query_caps_limit_and_preserves_cursor_and_count() -> Result<(), Box<dyn std::error::Error>>
{
    let query: ListDatasetsQueryParams =
        serde_json::from_str(r#"{"limit":101,"cursor":"cursor","include_count":true}"#)?;
    let input = ListDatasetsInput::from(&query);

    assert_eq!(input.limit(), ListDatasetsInput::MAX_LIMIT);
    assert_eq!(input.cursor(), Some("cursor"));
    assert!(input.include_count());

    Ok(())
}
