#[cfg(feature = "skill")]
#[doc = "Distinct abilities"]
pub enum Skill {
    #[doc = "Capabilities for adapting and personalizing content based on user context and preferences."]
    Personalization,
    #[doc = "Automatically filling in code templates with appropriate content."]
    CodeTemplates,
    #[doc = "Reviewing code, configurations, or dependency manifests to surface potential security weaknesses and misconfigurations."]
    VulnerabilityAnalysis,
    #[doc = "Facilitating negotiation, conflict handling, and consensus-building between agents."]
    NegotiationResolution,
    #[doc = "Identifying and isolating key characteristics or patterns from an image to aid in tasks like classification or recognition."]
    ImageFeatureExtraction,
    #[doc = "Proposing plausible explanations or solution pathways for incomplete or uncertain scenarios."]
    HypothesisGeneration,
    #[doc = "Capabilities for extracting and representing textual features as vectors for downstream tasks."]
    FeatureExtraction,
    #[doc = "Capabilities for retrieving relevant information from various sources and synthesizing it into coherent, contextually appropriate responses. This includes searching, extracting, combining, and presenting information in a meaningful way."]
    InformationRetrievalSynthesis,
    #[doc = "Rewriting text to express the same ideas using different words and structures while maintaining the original meaning."]
    Paraphrasing,
    #[doc = "Managing real-time collaboration and state synchronization among agents."]
    AgentCoordination,
    #[doc = "Producing segmented regions in an image to highlight specific areas or objects, typically represented as separate layers or overlays."]
    MaskGeneration,
    #[doc = "Scanning artifacts (code, logs, documents) to identify exposed credentials, tokens, or other sensitive secrets."]
    SecretLeakDetection,
    #[doc = "Translating natural language instructions into executable code."]
    TextToCode,
    #[doc = "Converting text from one language to another while maintaining meaning and context."]
    Translation,
    #[doc = "Defining or explaining steps to allocate and configure compute, storage, and networking resources."]
    InfrastructureProvisioning,
    #[doc = "Capability to perform efficient and accurate searches within large textual databases based on various criteria, including keywords, semantic meaning, or complex queries."]
    InformationRetrievalSynthesisSearch,
    #[doc = "Classify the sentiment of a text, that is, a positive movie review."]
    SentimentAnalysis,
    #[doc = "Generating a piece of text given a description or a first sentence to complete."]
    StoryGeneration,
    #[doc = "Generating 3D objects or scenes based on textual descriptions."]
    TextTo3d,
    #[doc = "Designing or describing automated sequences integrating multiple tools or services."]
    WorkflowAutomation,
    #[doc = "Retrieval of information is the process of fetching relevant data or documents from a large dataset or database based on a specific query or input."]
    RetrievalOfInformation,
    #[doc = "Maintaining coherent reasoning chains over extended sequences of steps or time."]
    LongHorizonReasoning,
    #[doc = "Selecting and ordering tool invocations to accomplish a specified goal efficiently."]
    ToolUsePlanning,
    #[doc = "Transforming one image into another using a learned mapping, often for tasks like style transfer, colorization, or image enhancement."]
    ImageToImage,
    #[doc = "Condensing system event or transaction logs into human-readable compliance or oversight summaries."]
    AuditTrailSummarization,
    #[doc = "Classifying the relation between two texts, like a contradiction, entailment, and others."]
    NaturalLanguageInference,
    #[doc = "Search is the process of exploring a dataset or index to find relevant information or results based on a given query."]
    RetrievalOfInformationSearch,
    #[doc = "Designing or explaining multi-step sequences that extract, transform, and load datasets."]
    DataTransformationPipeline,
    #[doc = "Assisting with solving problems by generating potential solutions or strategies."]
    ProblemSolving,
    #[doc = "Understanding the context and nuances of text input to provide relevant responses."]
    ContextualComprehension,
    #[doc = "Capability to analyze and determine the semantic similarity between sentences, supporting tasks like search, matching, and content comparison."]
    SentenceSimilarity,
    #[doc = "The base skill is a generic skill defining a set of attributes for skill classes."]
    BaseSkill,
    #[doc = "Natural Language Understanding (NLU) focuses on the ability to interpret and comprehend human language, including understanding context, semantics, and identifying key entities within text."]
    NaturalLanguageUnderstanding,
    #[doc = "Classifying data based on attributes using classical machine learning approaches."]
    TabularClassification,
    #[doc = "Capabilities for classifying and categorizing text into predefined categories or labels."]
    TextClassification,
    #[doc = "Creating new images from learned patterns or data using machine learning models."]
    ImageGeneration,
    #[doc = "Constructing informative transformed variables to improve downstream model performance."]
    FeatureEngineering,
    #[doc = "Linking custom scripts or functions with external tools to extend capabilities."]
    ScriptIntegration,
    #[doc = "Deriving structural metadata (fields, types, relationships) from raw or semi-structured data."]
    SchemaInference,
    #[doc = "Depth estimations the task of predicting the distance or depth of objects within a scene from a single image or multiple images."]
    Indexing,
    #[doc = "Capabilities for handling multiple languages, including translation and multilingual text processing."]
    LanguageTranslation,
    #[doc = "Identifying indicators of malicious activity, suspicious patterns, or emerging threats across logs and data sources."]
    ThreatDetection,
    #[doc = "Capabilities for processing audio, including speech synthesis and recognition."]
    AudioProcessing,
    #[doc = "Tagging each part of a sentence as nouns, adjectives, verbs, and so on."]
    PosTagging,
    #[doc = "Classifying a text as belong to one of several topics, which can be used to tag a text."]
    TopicLabeling,
    #[doc = "Capabilities for code generation, documentation, and optimization."]
    CodingSkills,
    #[doc = "Converting between any supported modalities (text, image, audio, video, or 3D)."]
    AnyToAny,
    #[doc = "Generating images based on textual descriptions or instructions."]
    TextToImage,
    #[doc = "Categorizing potential operational or data-related risks by impact and likelihood for prioritization."]
    RiskClassification,
    #[doc = "Evaluating datasets for completeness, validity, consistency, and timeliness."]
    DataQualityAssessment,
    #[doc = "Producing conversational responses that are contextually relevant and engaging within a dialogue context."]
    DialogueGeneration,
    #[doc = "Rewriting text to match the style of a given reference text while preserving the original content."]
    TextStyleTransfer,
    #[doc = "Interpreting and explaining API specifications, endpoints, parameters, and expected payloads."]
    ApiSchemaUnderstanding,
    #[doc = "Creating narratives, stories, or fictional content with creativity and coherence."]
    Storytelling,
    #[doc = "Configuring and interpreting telemetry signals, thresholds, and alerts for operational health."]
    MonitoringAlerting,
    #[doc = "Composing poems, prose, or other forms of creative literature."]
    PoetryWriting,
    #[doc = "Capabilities for solving mathematical problems and proving theorems."]
    MathematicalReasoning,
    #[doc = "Identifying and locating specific objects within an image or video, often by drawing bounding boxes around them."]
    ObjectDetection,
    #[doc = "Continuing a given text prompt in a coherent and contextually appropriate manner to generate fluent and contextually relevant content."]
    TextCompletion,
    #[doc = "Generating textual descriptions or captions for images."]
    ImageToText,
    #[doc = "Converting spoken language into written text."]
    SpeechRecognition,
    #[doc = "Identifying and categorizing key entities within the text, such as names, dates, or locations."]
    EntityRecognition,
    #[doc = "Task to recognize names as entity, for example, people, locations, buildings, and so on."]
    NamedEntityRecognition,
    #[doc = "Executing pure mathematical operations, such as arithmetic calculations."]
    PureMathOperations,
    #[doc = "Capabilities for classifying individual tokens or words within text."]
    TokenClassification,
    #[doc = "Capability to identify and extract factual information from text documents or knowledge bases, including entities, relationships, and key data points."]
    FactExtraction,
    #[doc = "Assigning labels or classes to audio content based on its characteristics."]
    AudioClassification,
    #[doc = "Document retrieval is the process of retrieving relevant documents from a collection based on a specific query, typically through indexing and search techniques."]
    DocumentRetrieval,
    #[doc = "Coordinating plans across multiple agents, resolving dependencies and optimizing sequencing."]
    MultiAgentPlanning,
    #[doc = "Assigning labels or categories to entire videos or segments based on their visual and audio content."]
    VideoClassification,
    #[doc = "Creating targeted test inputs or scenarios to probe system behavior and edge cases."]
    TestCaseGeneration,
    #[doc = "Evaluating processes or outputs against defined standards (e.g., GDPR, HIPAA) and identifying gaps."]
    ComplianceAssessment,
    #[doc = "Assigning labels or categories to images based on their visual content."]
    ImageClassification,
    #[doc = "Document or database question answering is the process of retrieving and using information from a document or database to answer a specific question."]
    DocumentOrDatabaseQuestionAnswering,
    #[doc = "Coordinating multi-stage application or model deployments, rollbacks, and version transitions."]
    DeploymentOrchestration,
    #[doc = "Detecting and correcting errors, inconsistencies, and missing values to improve dataset quality."]
    DataCleaning,
    #[doc = "Capabilities for ensuring ethical, unbiased, and safe content generation and interaction."]
    EthicalInteraction,
    #[doc = "Breaking complex objectives into structured, atomic subtasks."]
    TaskDecomposition,
    #[doc = "Capability to identify and retrieve relevant documents or text passages based on specific criteria or queries from a larger collection of texts."]
    DocumentPassageRetrieval,
    #[doc = "Verifying facts and claims given a reference text."]
    FactVerification,
    #[doc = "Evaluating data handling or user flows to surface potential privacy risks and recommend mitigations."]
    PrivacyRiskAssessment,
    #[doc = "Running standardized benchmarks or evaluation suites and summarizing results."]
    BenchmarkExecution,
    #[doc = "Tracking, promoting, and documenting different iterations of models and their artifacts."]
    ModelVersioning,
    #[doc = "Modifying the tone or style of generated text to suit specific audiences or purposes."]
    StyleAdjustment,
    #[doc = "Natural Language Generation (NLG) describes the ability to generate human-like text from structured data or other inputs."]
    NaturalLanguageGeneration,
    #[doc = "Translating organizational or regulatory policies into structured, enforceable rules or checklists."]
    PolicyMapping,
    #[doc = "Tracking latency, throughput, resource utilization, and service reliability over time."]
    PerformanceMonitoring,
    #[doc = "Assigning labels or categories to images based on their visual content."]
    ImageSegmentation,
    #[doc = "Capabilities for generating various forms of creative content, including narratives, poetry, and other creative writing forms."]
    CreativeContent,
    #[doc = "Making logical inferences based on provided information."]
    InferenceDeduction,
    #[doc = "Generating video content based on textual descriptions or instructions."]
    TextToVideo,
    #[doc = "System capability to understand questions and provide accurate, relevant answers by analyzing available information sources."]
    QuestionAnswering,
    #[doc = "Recognizing and processing text in multiple languages."]
    MultilingualUnderstanding,
    #[doc = "Representing parts of text with vectors to be used as input to other tasks."]
    ModelFeatureExtraction,
    #[doc = "Generating natural language documentation for code segments."]
    CodeToDocstrings,
    #[doc = "Predicting the distance or depth of objects within a scene from a single image or multiple images."]
    DepthEstimation,
    #[doc = "Answering questions about images using natural language."]
    VisualQa,
    #[doc = "Capabilities for processing and generating images from various inputs and generating textual descriptions of visual content."]
    ImageProcessing,
    #[doc = "Proving mathematical theorems using computational methods."]
    TheoremProving,
    #[doc = "Allocating responsibilities to agents based on capabilities and task requirements."]
    RoleAssignment,
    #[doc = "Capabilities for performing logical analysis, inference, and problem-solving tasks."]
    AnalyticalReasoning,
    #[doc = "Avoiding the generation of harmful, inappropriate, or sensitive content."]
    ContentModeration,
    #[doc = "The process of converting a 2D image into a 3D representation or model, often by inferring depth and spatial relationships."]
    ImageTo3d,
    #[doc = "Transforming audio through various manipulations including cutting, filtering, and mixing."]
    AudioToAudio,
    #[doc = "Predicting numerical values based on tabular attributes and features."]
    TabularRegression,
    #[doc = "Formulating high-level multi-phase strategies aligned with long-term objectives."]
    StrategicPlanning,
    #[doc = "Reducing or eliminating biased language and ensuring fair and unbiased output."]
    BiasMitigation,
    #[doc = "Identifying and locating specific points of interest within an image or object."]
    KeypointDetection,
    #[doc = "Designing or modifying continuous integration and delivery workflows and pipelines."]
    CiCdConfiguration,
    #[doc = "Assessing outputs for accuracy, relevance, coherence, safety, and style adherence."]
    QualityEvaluation,
    #[doc = "Automatically generating relevant and meaningful questions from a given text or context."]
    QuestionGeneration,
    #[doc = "Solving mathematical exercises presented in natural language format."]
    MathWordProblems,
    #[doc = "Capability to aggregate and combine information from multiple sources, creating comprehensive and coherent responses while maintaining context and relevance."]
    KnowledgeSynthesis,
    #[doc = "Converting text into natural-sounding speech audio."]
    TextToSpeech,
    #[doc = "Condensing longer texts into concise summaries while preserving essential information and maintaining coherence."]
    Summarization,
    #[doc = "Identifying unusual patterns, drifts, or deviations in data or model outputs."]
    AnomalyDetection,
    #[doc = "Tailoring responses based on user preferences, history, or context."]
    UserAdaptation,
    #[doc = "Generation of any is augmenting the creation of text, images, audio, or other media by incorporating retrieved information to improve or guide the generation process."]
    GenerationOfAny,
    #[doc = "Grasping the meaning and intent behind words and phrases."]
    SemanticUnderstanding,
    #[doc = "Rewriting and optimizing existing code through refactoring techniques."]
    CodeOptimization,
    #[doc = "Solving geometric problems and spatial reasoning tasks."]
    Geometry,
    #[doc = "Organizing intermediate reasoning steps into clear, justifiable sequences."]
    ChainOfThoughtStructuring,

}

#[cfg(all(feature = "skill", feature = "identify"))]
pub trait Identify {
    fn uid(&self) -> u32;
    fn name(&self) -> String;
}

#[cfg(all(feature = "skill", feature = "identify"))]
impl Identify for Skill {
    fn uid(&self) -> u32 {
        match self {
            Skill::Personalization => 106,
            Skill::CodeTemplates => 50203,
            Skill::VulnerabilityAnalysis => 802,
            Skill::NegotiationResolution => 1005,
            Skill::ImageFeatureExtraction => 208,
            Skill::HypothesisGeneration => 1504,
            Skill::FeatureExtraction => 110,
            Skill::InformationRetrievalSynthesis => 103,
            Skill::Paraphrasing => 10203,
            Skill::AgentCoordination => 1004,
            Skill::MaskGeneration => 209,
            Skill::SecretLeakDetection => 803,
            Skill::TextToCode => 50201,
            Skill::Translation => 10501,
            Skill::InfrastructureProvisioning => 1201,
            Skill::InformationRetrievalSynthesisSearch => 10306,
            Skill::SentimentAnalysis => 10902,
            Skill::StoryGeneration => 10207,
            Skill::TextTo3d => 70104,
            Skill::WorkflowAutomation => 1402,
            Skill::RetrievalOfInformation => 601,
            Skill::LongHorizonReasoning => 1502,
            Skill::ToolUsePlanning => 1403,
            Skill::ImageToImage => 210,
            Skill::AuditTrailSummarization => 1303,
            Skill::NaturalLanguageInference => 10903,
            Skill::RetrievalOfInformationSearch => 60102,
            Skill::DataTransformationPipeline => 904,
            Skill::ProblemSolving => 10702,
            Skill::ContextualComprehension => 10101,
            Skill::SentenceSimilarity => 10304,
            Skill::BaseSkill => 0,
            Skill::NaturalLanguageUnderstanding => 101,
            Skill::TabularClassification => 401,
            Skill::TextClassification => 109,
            Skill::ImageGeneration => 206,
            Skill::FeatureEngineering => 903,
            Skill::ScriptIntegration => 1404,
            Skill::SchemaInference => 902,
            Skill::Indexing => 60101,
            Skill::LanguageTranslation => 105,
            Skill::ThreatDetection => 801,
            Skill::AudioProcessing => 702,
            Skill::PosTagging => 11102,
            Skill::TopicLabeling => 10901,
            Skill::CodingSkills => 502,
            Skill::AnyToAny => 703,
            Skill::TextToImage => 70102,
            Skill::RiskClassification => 1304,
            Skill::DataQualityAssessment => 905,
            Skill::DialogueGeneration => 10204,
            Skill::TextStyleTransfer => 10206,
            Skill::ApiSchemaUnderstanding => 1401,
            Skill::Storytelling => 10401,
            Skill::MonitoringAlerting => 1205,
            Skill::PoetryWriting => 10402,
            Skill::MathematicalReasoning => 501,
            Skill::ObjectDetection => 204,
            Skill::TextCompletion => 10201,
            Skill::ImageToText => 70101,
            Skill::SpeechRecognition => 70202,
            Skill::EntityRecognition => 10103,
            Skill::NamedEntityRecognition => 11101,
            Skill::PureMathOperations => 50101,
            Skill::TokenClassification => 111,
            Skill::FactExtraction => 10301,
            Skill::AudioClassification => 301,
            Skill::DocumentRetrieval => 60103,
            Skill::MultiAgentPlanning => 1003,
            Skill::VideoClassification => 202,
            Skill::TestCaseGeneration => 1102,
            Skill::ComplianceAssessment => 1302,
            Skill::ImageClassification => 203,
            Skill::DocumentOrDatabaseQuestionAnswering => 602,
            Skill::DeploymentOrchestration => 1202,
            Skill::DataCleaning => 901,
            Skill::EthicalInteraction => 108,
            Skill::TaskDecomposition => 1001,
            Skill::DocumentPassageRetrieval => 10305,
            Skill::FactVerification => 10703,
            Skill::PrivacyRiskAssessment => 804,
            Skill::BenchmarkExecution => 1101,
            Skill::ModelVersioning => 1204,
            Skill::StyleAdjustment => 10602,
            Skill::NaturalLanguageGeneration => 102,
            Skill::PolicyMapping => 1301,
            Skill::PerformanceMonitoring => 1105,
            Skill::ImageSegmentation => 201,
            Skill::CreativeContent => 104,
            Skill::InferenceDeduction => 10701,
            Skill::TextToVideo => 70103,
            Skill::QuestionAnswering => 10302,
            Skill::MultilingualUnderstanding => 10502,
            Skill::ModelFeatureExtraction => 11001,
            Skill::CodeToDocstrings => 50202,
            Skill::DepthEstimation => 207,
            Skill::VisualQa => 70105,
            Skill::ImageProcessing => 701,
            Skill::TheoremProving => 50104,
            Skill::RoleAssignment => 1002,
            Skill::AnalyticalReasoning => 107,
            Skill::ContentModeration => 10802,
            Skill::ImageTo3d => 211,
            Skill::AudioToAudio => 302,
            Skill::TabularRegression => 402,
            Skill::StrategicPlanning => 1501,
            Skill::BiasMitigation => 10801,
            Skill::KeypointDetection => 205,
            Skill::CiCdConfiguration => 1203,
            Skill::QualityEvaluation => 1103,
            Skill::QuestionGeneration => 10205,
            Skill::MathWordProblems => 50102,
            Skill::KnowledgeSynthesis => 10303,
            Skill::TextToSpeech => 70201,
            Skill::Summarization => 10202,
            Skill::AnomalyDetection => 1104,
            Skill::UserAdaptation => 10601,
            Skill::GenerationOfAny => 603,
            Skill::SemanticUnderstanding => 10102,
            Skill::CodeOptimization => 50204,
            Skill::Geometry => 50103,
            Skill::ChainOfThoughtStructuring => 1503,

        }
    }
    fn name(&self) -> String {
        match self {
            Skill::Personalization => "personalization",
            Skill::CodeTemplates => "code_templates",
            Skill::VulnerabilityAnalysis => "vulnerability_analysis",
            Skill::NegotiationResolution => "negotiation_resolution",
            Skill::ImageFeatureExtraction => "image_feature_extraction",
            Skill::HypothesisGeneration => "hypothesis_generation",
            Skill::FeatureExtraction => "feature_extraction",
            Skill::InformationRetrievalSynthesis => "information_retrieval_synthesis",
            Skill::Paraphrasing => "paraphrasing",
            Skill::AgentCoordination => "agent_coordination",
            Skill::MaskGeneration => "mask_generation",
            Skill::SecretLeakDetection => "secret_leak_detection",
            Skill::TextToCode => "text_to_code",
            Skill::Translation => "translation",
            Skill::InfrastructureProvisioning => "infrastructure_provisioning",
            Skill::InformationRetrievalSynthesisSearch => "information_retrieval_synthesis_search",
            Skill::SentimentAnalysis => "sentiment_analysis",
            Skill::StoryGeneration => "story_generation",
            Skill::TextTo3d => "text_to_3d",
            Skill::WorkflowAutomation => "workflow_automation",
            Skill::RetrievalOfInformation => "retrieval_of_information",
            Skill::LongHorizonReasoning => "long_horizon_reasoning",
            Skill::ToolUsePlanning => "tool_use_planning",
            Skill::ImageToImage => "image_to_image",
            Skill::AuditTrailSummarization => "audit_trail_summarization",
            Skill::NaturalLanguageInference => "natural_language_inference",
            Skill::RetrievalOfInformationSearch => "retrieval_of_information_search",
            Skill::DataTransformationPipeline => "data_transformation_pipeline",
            Skill::ProblemSolving => "problem_solving",
            Skill::ContextualComprehension => "contextual_comprehension",
            Skill::SentenceSimilarity => "sentence_similarity",
            Skill::BaseSkill => "base_skill",
            Skill::NaturalLanguageUnderstanding => "natural_language_understanding",
            Skill::TabularClassification => "tabular_classification",
            Skill::TextClassification => "text_classification",
            Skill::ImageGeneration => "image_generation",
            Skill::FeatureEngineering => "feature_engineering",
            Skill::ScriptIntegration => "script_integration",
            Skill::SchemaInference => "schema_inference",
            Skill::Indexing => "indexing",
            Skill::LanguageTranslation => "language_translation",
            Skill::ThreatDetection => "threat_detection",
            Skill::AudioProcessing => "audio_processing",
            Skill::PosTagging => "pos_tagging",
            Skill::TopicLabeling => "topic_labeling",
            Skill::CodingSkills => "coding_skills",
            Skill::AnyToAny => "any_to_any",
            Skill::TextToImage => "text_to_image",
            Skill::RiskClassification => "risk_classification",
            Skill::DataQualityAssessment => "data_quality_assessment",
            Skill::DialogueGeneration => "dialogue_generation",
            Skill::TextStyleTransfer => "text_style_transfer",
            Skill::ApiSchemaUnderstanding => "api_schema_understanding",
            Skill::Storytelling => "storytelling",
            Skill::MonitoringAlerting => "monitoring_alerting",
            Skill::PoetryWriting => "poetry_writing",
            Skill::MathematicalReasoning => "mathematical_reasoning",
            Skill::ObjectDetection => "object_detection",
            Skill::TextCompletion => "text_completion",
            Skill::ImageToText => "image_to_text",
            Skill::SpeechRecognition => "speech_recognition",
            Skill::EntityRecognition => "entity_recognition",
            Skill::NamedEntityRecognition => "named_entity_recognition",
            Skill::PureMathOperations => "pure_math_operations",
            Skill::TokenClassification => "token_classification",
            Skill::FactExtraction => "fact_extraction",
            Skill::AudioClassification => "audio_classification",
            Skill::DocumentRetrieval => "document_retrieval",
            Skill::MultiAgentPlanning => "multi_agent_planning",
            Skill::VideoClassification => "video_classification",
            Skill::TestCaseGeneration => "test_case_generation",
            Skill::ComplianceAssessment => "compliance_assessment",
            Skill::ImageClassification => "image_classification",
            Skill::DocumentOrDatabaseQuestionAnswering => "document_or_database_question_answering",
            Skill::DeploymentOrchestration => "deployment_orchestration",
            Skill::DataCleaning => "data_cleaning",
            Skill::EthicalInteraction => "ethical_interaction",
            Skill::TaskDecomposition => "task_decomposition",
            Skill::DocumentPassageRetrieval => "document_passage_retrieval",
            Skill::FactVerification => "fact_verification",
            Skill::PrivacyRiskAssessment => "privacy_risk_assessment",
            Skill::BenchmarkExecution => "benchmark_execution",
            Skill::ModelVersioning => "model_versioning",
            Skill::StyleAdjustment => "style_adjustment",
            Skill::NaturalLanguageGeneration => "natural_language_generation",
            Skill::PolicyMapping => "policy_mapping",
            Skill::PerformanceMonitoring => "performance_monitoring",
            Skill::ImageSegmentation => "image_segmentation",
            Skill::CreativeContent => "creative_content",
            Skill::InferenceDeduction => "inference_deduction",
            Skill::TextToVideo => "text_to_video",
            Skill::QuestionAnswering => "question_answering",
            Skill::MultilingualUnderstanding => "multilingual_understanding",
            Skill::ModelFeatureExtraction => "model_feature_extraction",
            Skill::CodeToDocstrings => "code_to_docstrings",
            Skill::DepthEstimation => "depth_estimation",
            Skill::VisualQa => "visual_qa",
            Skill::ImageProcessing => "image_processing",
            Skill::TheoremProving => "theorem_proving",
            Skill::RoleAssignment => "role_assignment",
            Skill::AnalyticalReasoning => "analytical_reasoning",
            Skill::ContentModeration => "content_moderation",
            Skill::ImageTo3d => "image_to_3d",
            Skill::AudioToAudio => "audio_to_audio",
            Skill::TabularRegression => "tabular_regression",
            Skill::StrategicPlanning => "strategic_planning",
            Skill::BiasMitigation => "bias_mitigation",
            Skill::KeypointDetection => "keypoint_detection",
            Skill::CiCdConfiguration => "ci_cd_configuration",
            Skill::QualityEvaluation => "quality_evaluation",
            Skill::QuestionGeneration => "question_generation",
            Skill::MathWordProblems => "math_word_problems",
            Skill::KnowledgeSynthesis => "knowledge_synthesis",
            Skill::TextToSpeech => "text_to_speech",
            Skill::Summarization => "summarization",
            Skill::AnomalyDetection => "anomaly_detection",
            Skill::UserAdaptation => "user_adaptation",
            Skill::GenerationOfAny => "generation_of_any",
            Skill::SemanticUnderstanding => "semantic_understanding",
            Skill::CodeOptimization => "code_optimization",
            Skill::Geometry => "geometry",
            Skill::ChainOfThoughtStructuring => "chain_of_thought_structuring",

        }.to_string()
    }
}

#[cfg(feature = "skill")]
impl From<Skill> for String {
    fn from(value: Skill) -> String {
        match value {
            Skill::Personalization => "personalization",
            Skill::CodeTemplates => "code_templates",
            Skill::VulnerabilityAnalysis => "vulnerability_analysis",
            Skill::NegotiationResolution => "negotiation_resolution",
            Skill::ImageFeatureExtraction => "image_feature_extraction",
            Skill::HypothesisGeneration => "hypothesis_generation",
            Skill::FeatureExtraction => "feature_extraction",
            Skill::InformationRetrievalSynthesis => "information_retrieval_synthesis",
            Skill::Paraphrasing => "paraphrasing",
            Skill::AgentCoordination => "agent_coordination",
            Skill::MaskGeneration => "mask_generation",
            Skill::SecretLeakDetection => "secret_leak_detection",
            Skill::TextToCode => "text_to_code",
            Skill::Translation => "translation",
            Skill::InfrastructureProvisioning => "infrastructure_provisioning",
            Skill::InformationRetrievalSynthesisSearch => "information_retrieval_synthesis_search",
            Skill::SentimentAnalysis => "sentiment_analysis",
            Skill::StoryGeneration => "story_generation",
            Skill::TextTo3d => "text_to_3d",
            Skill::WorkflowAutomation => "workflow_automation",
            Skill::RetrievalOfInformation => "retrieval_of_information",
            Skill::LongHorizonReasoning => "long_horizon_reasoning",
            Skill::ToolUsePlanning => "tool_use_planning",
            Skill::ImageToImage => "image_to_image",
            Skill::AuditTrailSummarization => "audit_trail_summarization",
            Skill::NaturalLanguageInference => "natural_language_inference",
            Skill::RetrievalOfInformationSearch => "retrieval_of_information_search",
            Skill::DataTransformationPipeline => "data_transformation_pipeline",
            Skill::ProblemSolving => "problem_solving",
            Skill::ContextualComprehension => "contextual_comprehension",
            Skill::SentenceSimilarity => "sentence_similarity",
            Skill::BaseSkill => "base_skill",
            Skill::NaturalLanguageUnderstanding => "natural_language_understanding",
            Skill::TabularClassification => "tabular_classification",
            Skill::TextClassification => "text_classification",
            Skill::ImageGeneration => "image_generation",
            Skill::FeatureEngineering => "feature_engineering",
            Skill::ScriptIntegration => "script_integration",
            Skill::SchemaInference => "schema_inference",
            Skill::Indexing => "indexing",
            Skill::LanguageTranslation => "language_translation",
            Skill::ThreatDetection => "threat_detection",
            Skill::AudioProcessing => "audio_processing",
            Skill::PosTagging => "pos_tagging",
            Skill::TopicLabeling => "topic_labeling",
            Skill::CodingSkills => "coding_skills",
            Skill::AnyToAny => "any_to_any",
            Skill::TextToImage => "text_to_image",
            Skill::RiskClassification => "risk_classification",
            Skill::DataQualityAssessment => "data_quality_assessment",
            Skill::DialogueGeneration => "dialogue_generation",
            Skill::TextStyleTransfer => "text_style_transfer",
            Skill::ApiSchemaUnderstanding => "api_schema_understanding",
            Skill::Storytelling => "storytelling",
            Skill::MonitoringAlerting => "monitoring_alerting",
            Skill::PoetryWriting => "poetry_writing",
            Skill::MathematicalReasoning => "mathematical_reasoning",
            Skill::ObjectDetection => "object_detection",
            Skill::TextCompletion => "text_completion",
            Skill::ImageToText => "image_to_text",
            Skill::SpeechRecognition => "speech_recognition",
            Skill::EntityRecognition => "entity_recognition",
            Skill::NamedEntityRecognition => "named_entity_recognition",
            Skill::PureMathOperations => "pure_math_operations",
            Skill::TokenClassification => "token_classification",
            Skill::FactExtraction => "fact_extraction",
            Skill::AudioClassification => "audio_classification",
            Skill::DocumentRetrieval => "document_retrieval",
            Skill::MultiAgentPlanning => "multi_agent_planning",
            Skill::VideoClassification => "video_classification",
            Skill::TestCaseGeneration => "test_case_generation",
            Skill::ComplianceAssessment => "compliance_assessment",
            Skill::ImageClassification => "image_classification",
            Skill::DocumentOrDatabaseQuestionAnswering => "document_or_database_question_answering",
            Skill::DeploymentOrchestration => "deployment_orchestration",
            Skill::DataCleaning => "data_cleaning",
            Skill::EthicalInteraction => "ethical_interaction",
            Skill::TaskDecomposition => "task_decomposition",
            Skill::DocumentPassageRetrieval => "document_passage_retrieval",
            Skill::FactVerification => "fact_verification",
            Skill::PrivacyRiskAssessment => "privacy_risk_assessment",
            Skill::BenchmarkExecution => "benchmark_execution",
            Skill::ModelVersioning => "model_versioning",
            Skill::StyleAdjustment => "style_adjustment",
            Skill::NaturalLanguageGeneration => "natural_language_generation",
            Skill::PolicyMapping => "policy_mapping",
            Skill::PerformanceMonitoring => "performance_monitoring",
            Skill::ImageSegmentation => "image_segmentation",
            Skill::CreativeContent => "creative_content",
            Skill::InferenceDeduction => "inference_deduction",
            Skill::TextToVideo => "text_to_video",
            Skill::QuestionAnswering => "question_answering",
            Skill::MultilingualUnderstanding => "multilingual_understanding",
            Skill::ModelFeatureExtraction => "model_feature_extraction",
            Skill::CodeToDocstrings => "code_to_docstrings",
            Skill::DepthEstimation => "depth_estimation",
            Skill::VisualQa => "visual_qa",
            Skill::ImageProcessing => "image_processing",
            Skill::TheoremProving => "theorem_proving",
            Skill::RoleAssignment => "role_assignment",
            Skill::AnalyticalReasoning => "analytical_reasoning",
            Skill::ContentModeration => "content_moderation",
            Skill::ImageTo3d => "image_to_3d",
            Skill::AudioToAudio => "audio_to_audio",
            Skill::TabularRegression => "tabular_regression",
            Skill::StrategicPlanning => "strategic_planning",
            Skill::BiasMitigation => "bias_mitigation",
            Skill::KeypointDetection => "keypoint_detection",
            Skill::CiCdConfiguration => "ci_cd_configuration",
            Skill::QualityEvaluation => "quality_evaluation",
            Skill::QuestionGeneration => "question_generation",
            Skill::MathWordProblems => "math_word_problems",
            Skill::KnowledgeSynthesis => "knowledge_synthesis",
            Skill::TextToSpeech => "text_to_speech",
            Skill::Summarization => "summarization",
            Skill::AnomalyDetection => "anomaly_detection",
            Skill::UserAdaptation => "user_adaptation",
            Skill::GenerationOfAny => "generation_of_any",
            Skill::SemanticUnderstanding => "semantic_understanding",
            Skill::CodeOptimization => "code_optimization",
            Skill::Geometry => "geometry",
            Skill::ChainOfThoughtStructuring => "chain_of_thought_structuring",

        }.to_string()
    }
}

#[cfg(feature = "skill")]
impl From<Skill> for u32 {
    fn from(value: Skill) -> u32 {
        match value {
            Skill::Personalization => 106,
            Skill::CodeTemplates => 50203,
            Skill::VulnerabilityAnalysis => 802,
            Skill::NegotiationResolution => 1005,
            Skill::ImageFeatureExtraction => 208,
            Skill::HypothesisGeneration => 1504,
            Skill::FeatureExtraction => 110,
            Skill::InformationRetrievalSynthesis => 103,
            Skill::Paraphrasing => 10203,
            Skill::AgentCoordination => 1004,
            Skill::MaskGeneration => 209,
            Skill::SecretLeakDetection => 803,
            Skill::TextToCode => 50201,
            Skill::Translation => 10501,
            Skill::InfrastructureProvisioning => 1201,
            Skill::InformationRetrievalSynthesisSearch => 10306,
            Skill::SentimentAnalysis => 10902,
            Skill::StoryGeneration => 10207,
            Skill::TextTo3d => 70104,
            Skill::WorkflowAutomation => 1402,
            Skill::RetrievalOfInformation => 601,
            Skill::LongHorizonReasoning => 1502,
            Skill::ToolUsePlanning => 1403,
            Skill::ImageToImage => 210,
            Skill::AuditTrailSummarization => 1303,
            Skill::NaturalLanguageInference => 10903,
            Skill::RetrievalOfInformationSearch => 60102,
            Skill::DataTransformationPipeline => 904,
            Skill::ProblemSolving => 10702,
            Skill::ContextualComprehension => 10101,
            Skill::SentenceSimilarity => 10304,
            Skill::BaseSkill => 0,
            Skill::NaturalLanguageUnderstanding => 101,
            Skill::TabularClassification => 401,
            Skill::TextClassification => 109,
            Skill::ImageGeneration => 206,
            Skill::FeatureEngineering => 903,
            Skill::ScriptIntegration => 1404,
            Skill::SchemaInference => 902,
            Skill::Indexing => 60101,
            Skill::LanguageTranslation => 105,
            Skill::ThreatDetection => 801,
            Skill::AudioProcessing => 702,
            Skill::PosTagging => 11102,
            Skill::TopicLabeling => 10901,
            Skill::CodingSkills => 502,
            Skill::AnyToAny => 703,
            Skill::TextToImage => 70102,
            Skill::RiskClassification => 1304,
            Skill::DataQualityAssessment => 905,
            Skill::DialogueGeneration => 10204,
            Skill::TextStyleTransfer => 10206,
            Skill::ApiSchemaUnderstanding => 1401,
            Skill::Storytelling => 10401,
            Skill::MonitoringAlerting => 1205,
            Skill::PoetryWriting => 10402,
            Skill::MathematicalReasoning => 501,
            Skill::ObjectDetection => 204,
            Skill::TextCompletion => 10201,
            Skill::ImageToText => 70101,
            Skill::SpeechRecognition => 70202,
            Skill::EntityRecognition => 10103,
            Skill::NamedEntityRecognition => 11101,
            Skill::PureMathOperations => 50101,
            Skill::TokenClassification => 111,
            Skill::FactExtraction => 10301,
            Skill::AudioClassification => 301,
            Skill::DocumentRetrieval => 60103,
            Skill::MultiAgentPlanning => 1003,
            Skill::VideoClassification => 202,
            Skill::TestCaseGeneration => 1102,
            Skill::ComplianceAssessment => 1302,
            Skill::ImageClassification => 203,
            Skill::DocumentOrDatabaseQuestionAnswering => 602,
            Skill::DeploymentOrchestration => 1202,
            Skill::DataCleaning => 901,
            Skill::EthicalInteraction => 108,
            Skill::TaskDecomposition => 1001,
            Skill::DocumentPassageRetrieval => 10305,
            Skill::FactVerification => 10703,
            Skill::PrivacyRiskAssessment => 804,
            Skill::BenchmarkExecution => 1101,
            Skill::ModelVersioning => 1204,
            Skill::StyleAdjustment => 10602,
            Skill::NaturalLanguageGeneration => 102,
            Skill::PolicyMapping => 1301,
            Skill::PerformanceMonitoring => 1105,
            Skill::ImageSegmentation => 201,
            Skill::CreativeContent => 104,
            Skill::InferenceDeduction => 10701,
            Skill::TextToVideo => 70103,
            Skill::QuestionAnswering => 10302,
            Skill::MultilingualUnderstanding => 10502,
            Skill::ModelFeatureExtraction => 11001,
            Skill::CodeToDocstrings => 50202,
            Skill::DepthEstimation => 207,
            Skill::VisualQa => 70105,
            Skill::ImageProcessing => 701,
            Skill::TheoremProving => 50104,
            Skill::RoleAssignment => 1002,
            Skill::AnalyticalReasoning => 107,
            Skill::ContentModeration => 10802,
            Skill::ImageTo3d => 211,
            Skill::AudioToAudio => 302,
            Skill::TabularRegression => 402,
            Skill::StrategicPlanning => 1501,
            Skill::BiasMitigation => 10801,
            Skill::KeypointDetection => 205,
            Skill::CiCdConfiguration => 1203,
            Skill::QualityEvaluation => 1103,
            Skill::QuestionGeneration => 10205,
            Skill::MathWordProblems => 50102,
            Skill::KnowledgeSynthesis => 10303,
            Skill::TextToSpeech => 70201,
            Skill::Summarization => 10202,
            Skill::AnomalyDetection => 1104,
            Skill::UserAdaptation => 10601,
            Skill::GenerationOfAny => 603,
            Skill::SemanticUnderstanding => 10102,
            Skill::CodeOptimization => 50204,
            Skill::Geometry => 50103,
            Skill::ChainOfThoughtStructuring => 1503,

        }
    }
}