use crate::domain::entities::agent::test_fixtures::AgentBuilder;
use crate::presentation::http::v1::responses::agents::Agent as AgentResponse;
use crate::shared_kernel::value_objects::TimeStamp;

#[test]
fn agent_response_preserves_missed_heartbeat_state() -> Result<(), Box<dyn std::error::Error>> {
    let last_missed_heartbeat = TimeStamp::parse_string("2026-08-26T12:00:00Z")?;
    let agent = AgentBuilder::new()
        .with_last_missed_heartbeat(last_missed_heartbeat)
        .with_consecutive_missed_heartbeats(3)
        .build_reconstituted()?;

    let response = AgentResponse::from(agent);

    assert_eq!(
        response.last_missed_heartbeat.as_deref(),
        Some("2026-08-26T12:00:00+00:00")
    );
    assert_eq!(response.consecutive_missed_heartbeats, 3);
    assert!(response.endpoints.is_empty());

    Ok(())
}
