use crate::domain::entities::agent::test_fixtures::AgentBuilder;
use crate::domain::entities::agent::{Agent, AgentError};

use super::Agent as AgentDocument;

#[test]
fn agent_document_round_trip_preserves_missed_heartbeat_state(
) -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new().build_registered()?;
    let mut document = AgentDocument::from(&agent);

    document.last_missed_heartbeat = Some("2026-08-26T12:00:00Z".into());
    document.consecutive_missed_heartbeats = 3;

    let reconstituted = Agent::try_from(document)?;

    assert_eq!(
        reconstituted
            .last_missed_heartbeat()
            .map(|timestamp| String::from(timestamp.clone())),
        Some("2026-08-26T12:00:00+00:00".into())
    );
    assert_eq!(reconstituted.consecutive_missed_heartbeats(), 3);

    Ok(())
}

#[test]
fn agent_document_rejects_invalid_missed_heartbeat_timestamp(
) -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new().build_registered()?;
    let mut document = AgentDocument::from(&agent);

    document.last_missed_heartbeat = Some("not-a-timestamp".into());

    let result = Agent::try_from(document);

    assert!(matches!(result, Err(AgentError::DataIntegrityError(_))));
    Ok(())
}
