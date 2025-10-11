{{ feature_attr }}
#[doc = "A comprehensive table outlining module sets of application and knowledge areas within the framework."]
pub enum Module {
    #[doc = "Runtime module set."]
    Runtime,
    #[doc = "Describes common LLM interaction prompts to use the agent."]
    LlmPrompt,
    #[doc = "Describes LLM support and its configuration for a given agent."]
    LlmModel,
    #[doc = "Describes A2A card details for communication and usage with A2A protocol."]
    A2a,
    #[doc = "Agent manifest"]
    Manifest,
    #[doc = "Describes MCP servers required to run and interact with the agent."]
    Mcp,
    #[doc = "Identity module set."]
    Identity,
    #[doc = "Observability module set."]
    Observability,
    #[doc = "A module describing how the agent can be observed."]
    Observability,
    #[doc = "Evaluation module set."]
    Evaluation,
    #[doc = "Assessing actions and outcomes to determine their effectiveness, guiding future decision-making and enhancing personal agency."]
    Evaluation,

}

{{ identify_trait }}

{{ identify_impl }}

{{ feature_attr }}
impl From<Module> for &str {
    fn from(value: Module) -> &'static str {
        match value {
            Module::Runtime => "runtime",
            Module::LlmPrompt => "runtime/prompt",
            Module::LlmModel => "runtime/model",
            Module::A2a => "runtime/a2a",
            Module::Manifest => "runtime/manifest",
            Module::Mcp => "runtime/mcp",
            Module::Identity => "identity",
            Module::Observability => "observability",
            Module::Observability => "observability/base_module/observability",
            Module::Evaluation => "evaluation",
            Module::Evaluation => "evaluation/base_module/evaluation",

        }
    }
}

{{ feature_attr }}
impl From<Module> for u32 {
    fn from(value: Module) -> u32 {
        match value {
            Module::Runtime => 3,
            Module::LlmPrompt => 304,
            Module::LlmModel => 303,
            Module::A2a => 305,
            Module::Manifest => 301,
            Module::Mcp => 302,
            Module::Identity => 4,
            Module::Observability => 1,
            Module::Observability => 101,
            Module::Evaluation => 2,
            Module::Evaluation => 201,

        }
    }
}