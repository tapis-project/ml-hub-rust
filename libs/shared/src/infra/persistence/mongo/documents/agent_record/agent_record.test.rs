use crate::domain::entities::agent_record::{
    AgentArtifactType, AgentInterface, AgentProvider, AgentSkill, ArtifactLocator, Capabilities,
    LivenessProbeConfiguration, MessageBinding, Protocol, test_fixtures::AgentRecordBuilder,
};

use super::AgentRecord as AgentRecordDocument;

#[test]
fn test_agent_record_document_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let skill = AgentSkill::new(
        "geospatial-search".into(),
        "Geospatial search".into(),
        "Searches geospatial data.".into(),
        vec!["geospatial".into()],
        vec!["Find flood zones in Travis County.".into()],
    )?;
    let agent_record = AgentRecordBuilder::new()
        .with_interfaces(vec![AgentInterface::new(
            "rest".into(),
            Some("REST interface".into()),
            Protocol::RestHttp,
            Some(MessageBinding::HttpJson),
            Some(LivenessProbeConfiguration::RestHttp {
                route: "/healthcheck".into(),
                timeout_seconds: 10,
            }),
        )])
        .with_capabilities(Capabilities::new(true, true))
        .with_provider(AgentProvider::new(
            "Example Geo Services Inc.".into(),
            "https://www.examplegeoservices.com".into(),
        ))
        .with_artifact_locators(vec![ArtifactLocator::new(
            AgentArtifactType::DockerImage,
            "tapis://example/agent:1.0.0".into(),
        )])
        .with_skills(vec![skill])
        .with_icon_url("https://example.com/icon.svg".into())
        .with_documentation_url("https://example.com/docs".into())
        .build_new()?;

    let document = AgentRecordDocument::from(&agent_record);
    let reconstituted = crate::domain::entities::agent_record::AgentRecord::try_from(document)?;

    assert_eq!(reconstituted.id(), agent_record.id());
    assert_eq!(reconstituted.name(), agent_record.name());
    assert_eq!(reconstituted.tenant_id(), agent_record.tenant_id());
    assert_eq!(reconstituted.owner(), agent_record.owner());
    assert_eq!(reconstituted.description(), agent_record.description());
    assert!(reconstituted.supports_streaming());
    assert!(reconstituted.supports_push_notifications());
    assert_eq!(
        reconstituted.provider_organization(),
        Some("Example Geo Services Inc.")
    );
    assert_eq!(
        reconstituted
            .artifact_locators()
            .first()
            .map(ArtifactLocator::url),
        Some("tapis://example/agent:1.0.0")
    );
    assert_eq!(
        reconstituted.skills().first().map(AgentSkill::id),
        Some("geospatial-search")
    );
    assert_eq!(
        reconstituted.icon_url(),
        Some("https://example.com/icon.svg")
    );
    assert_eq!(
        reconstituted.documentation_url(),
        Some("https://example.com/docs")
    );
    assert!(matches!(
        reconstituted.interfaces().first().liveness_probe_config(),
        Some(LivenessProbeConfiguration::RestHttp {
            route,
            timeout_seconds: 10,
        }) if route == "/healthcheck"
    ));

    Ok(())
}
