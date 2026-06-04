#[cfg(feature = "module")]
#[doc = "Module sets of application and knowledge areas"]
pub enum Module {
    #[doc = "Describes common LLM interaction prompts to use the agent."]
    Prompt,
    #[doc = "The base module is a generic module defining a set of attributes for module classes."]
    BaseModule,
    #[doc = "Describes LLM support and its configuration for a given agent."]
    Model,
    #[doc = "Modules for basic LLM functionality."]
    Llm,
    #[doc = "A module describing how the agent can be observed."]
    Observability,
    #[doc = "Describes A2A card details for communication and usage with A2A protocol."]
    A2a,
    #[doc = "Agent Connect Protocol manifest"]
    Acp,
    #[doc = "Open Agent Spec is a specification language to define the structure of agentic systems, from simple tool-use agents to structured workflows and multi-agent systems. This module describes Open Agent Spec details to inform about the agent structure."]
    Agentspec,
    #[doc = "Describes MCP servers required to run and interact with the agent."]
    Mcp,

}

#[cfg(all(feature = "module", feature = "identify"))]
pub trait Identify {
    fn uid(&self) -> u32;
    fn name(&self) -> String;
}

#[cfg(all(feature = "module", feature = "identify"))]
impl Identify for Module {
    fn uid(&self) -> u32 {
        match self {
            Module::Prompt => 10202,
            Module::BaseModule => 0,
            Module::Model => 10201,
            Module::Llm => 102,
            Module::Observability => 101,
            Module::A2a => 203,
            Module::Acp => 201,
            Module::Agentspec => 204,
            Module::Mcp => 202,

        }
    }
    fn name(&self) -> String {
        match self {
            Module::Prompt => "prompt",
            Module::BaseModule => "base_module",
            Module::Model => "model",
            Module::Llm => "llm",
            Module::Observability => "observability",
            Module::A2a => "a2a",
            Module::Acp => "acp",
            Module::Agentspec => "agentspec",
            Module::Mcp => "mcp",

        }.to_string()
    }
}

#[cfg(feature = "module")]
impl From<Module> for String {
    fn from(value: Module) -> String {
        match value {
            Module::Prompt => "prompt",
            Module::BaseModule => "base_module",
            Module::Model => "model",
            Module::Llm => "llm",
            Module::Observability => "observability",
            Module::A2a => "a2a",
            Module::Acp => "acp",
            Module::Agentspec => "agentspec",
            Module::Mcp => "mcp",

        }.to_string()
    }
}

#[cfg(feature = "module")]
impl From<Module> for u32 {
    fn from(value: Module) -> u32 {
        match value {
            Module::Prompt => 10202,
            Module::BaseModule => 0,
            Module::Model => 10201,
            Module::Llm => 102,
            Module::Observability => 101,
            Module::A2a => 203,
            Module::Acp => 201,
            Module::Agentspec => 204,
            Module::Mcp => 202,

        }
    }
}