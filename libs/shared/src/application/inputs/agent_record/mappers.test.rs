use crate::application::inputs::agent_record::{
    CreateAgentRecordInput, LivenessProbeConfigurationInput, MessageBindingInput, ProtocolInput,
};
use crate::presentation::http::v1::requests::create_agent_record::body::{
    Capabilities, CreateAgentRecordBody, MessageBinding, RestHttpAgentInterface,
    RestHttpLivenessProbe, RpcAgentInterface, StdioAgentInterface, Visibility,
};

#[test]
fn maps_concrete_request_interfaces_to_polymorphic_application_inputs() {
    let request = CreateAgentRecordBody {
        name: "assistant".into(),
        description: "A helpful agent".into(),
        rest_http_interfaces: vec![RestHttpAgentInterface {
            name: "rest".into(),
            description: Some("REST interface".into()),
            message_binding: Some(MessageBinding::HttpJson),
            liveness_probe_config: Some(RestHttpLivenessProbe {
                route: "/healthcheck".into(),
                interval_seconds: 30,
                timeout_seconds: 10,
                missed_heartbeat_threshold: 3,
                initial_delay_seconds: 60,
            }),
        }],
        rpc_interfaces: vec![RpcAgentInterface {
            name: "rpc".into(),
            description: None,
            message_binding: Some(MessageBinding::JsonRpc2_0),
        }],
        stdio_interfaces: vec![StdioAgentInterface {
            name: "stdio".into(),
            description: None,
            message_binding: None,
        }],
        capabilities: Capabilities {
            streaming: false,
            push_notifications: false,
        },
        provider: None,
        version: "1.0.0".into(),
        artifact_locators: vec![],
        default_input_modes: vec!["application/json".into()],
        default_output_modes: vec!["application/json".into()],
        skills: vec![],
        tags: vec![],
        icon_url: None,
        documentation_url: None,
        visibility: Visibility::Private,
    };

    let input = match CreateAgentRecordInput::try_from(request) {
        Ok(input) => input,
        Err(error) => panic!("Expected valid I/O mode mapping: {error}"),
    };

    assert_eq!(input.interfaces.len(), 3);
    assert!(matches!(
        input.interfaces[0].protocol,
        ProtocolInput::RestHttp
    ));
    assert!(matches!(
        input.interfaces[0].message_binding,
        Some(MessageBindingInput::HttpJson)
    ));
    assert!(matches!(
        input.interfaces[0].liveness_probe_config,
        Some(LivenessProbeConfigurationInput::RestHttp {
            ref route,
            interval_seconds: 30,
            timeout_seconds: 10,
            missed_heartbeat_threshold: 3,
            initial_delay_seconds: 60,
        }) if route == "/healthcheck"
    ));
    assert!(matches!(input.interfaces[1].protocol, ProtocolInput::Rpc));
    assert!(matches!(
        input.interfaces[1].message_binding,
        Some(MessageBindingInput::JsonRpc2_0)
    ));
    assert!(input.interfaces[1].liveness_probe_config.is_none());
    assert!(matches!(input.interfaces[2].protocol, ProtocolInput::Stdio));
    assert!(input.interfaces[2].liveness_probe_config.is_none());
    assert_eq!(input.default_input_modes[0].as_str(), "application/json");
    assert_eq!(input.default_output_modes[0].as_str(), "application/json");
}
