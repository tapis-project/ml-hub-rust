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
                timeout_seconds: 10,
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
        skills: vec![],
        icon_url: None,
        documentation_url: None,
        visibility: Visibility::Private,
    };

    let input = CreateAgentRecordInput::from(request);

    assert_eq!(input.interfaces.len(), 3);
    assert!(matches!(input.interfaces[0].protocol, ProtocolInput::RestHttp));
    assert!(matches!(
        input.interfaces[0].message_binding,
        Some(MessageBindingInput::HttpJson)
    ));
    assert!(matches!(
        input.interfaces[0].liveness_probe_config,
        Some(LivenessProbeConfigurationInput::RestHttp {
            ref route,
            timeout_seconds: 10,
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
}
