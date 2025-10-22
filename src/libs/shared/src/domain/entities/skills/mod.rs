mod agntcy_skill_to_skill;

#[doc = "A structured view of distinct abilities, defining the capabilities within the Open Agentic Schema Framework."]
pub enum Skill {
    #[doc = "Natural Language Processing (NLP) tasks are the application of computational techniques to the analysis and synthesis of natural language and speech."]
    NaturalLanguageProcessing,
    #[doc = "Creating narratives, stories, or fictional content with creativity and coherence."]
    Storytelling,
    #[doc = "Verifying facts and claims given a reference text."]
    FactAndClaimVerification,
    #[doc = "Natural Language Generation (NLG) describes the ability to generate human-like text from structured data or other inputs."]
    NaturalLanguageGeneration,
    #[doc = "Reducing or eliminating biased language and ensuring fair and unbiased output."]
    BiasMitigation,
    #[doc = "Generating a piece of text given a description or a first sentence to complete."]
    StoryGeneration,
    #[doc = "Capability to analyze and determine the semantic similarity between sentences, supporting tasks like search, matching, and content comparison."]
    SentenceSimilarity,
    #[doc = "Capabilities for classifying and categorizing text into predefined categories or labels."]
    TextClassification,
    #[doc = "Capability to identify and retrieve relevant documents or text passages based on specific criteria or queries from a larger collection of texts."]
    DocumentAndPassageRetrieval,
    #[doc = "Classifying the relation between two texts, like a contradiction, entailment, and others."]
    NaturalLanguageInference,
    #[doc = "Capabilities for extracting and representing textual features as vectors for downstream tasks."]
    ModuleExtraction,
    #[doc = "Natural Language Understanding (NLU) focuses on the ability to interpret and comprehend human language, including understanding context, semantics, and identifying key entities within text."]
    NaturalLanguageUnderstanding,
    #[doc = "Assisting with solving problems by generating potential solutions or strategies."]
    ProblemSolving,
    #[doc = "Tailoring responses based on user preferences, history, or context."]
    UserAdaptation,
    #[doc = "Automatically generating relevant and meaningful questions from a given text or context."]
    QuestionGeneration,
    #[doc = "Understanding the context and nuances of text input to provide relevant responses."]
    ContextualComprehension,
    #[doc = "Converting text from one language to another while maintaining meaning and context."]
    Translation,
    #[doc = "Rewriting text to express the same ideas using different words and structures while maintaining the original meaning."]
    TextParaphrasing,
    #[doc = "Capabilities for performing logical analysis, inference, and problem-solving tasks."]
    AnalyticalAndLogicalReasoning,
    #[doc = "Capability to identify and extract factual information from text documents or knowledge bases, including entities, relationships, and key data points."]
    FactExtraction,
    #[doc = "Capabilities for handling multiple languages, including translation and multilingual text processing."]
    LanguageTranslationAndMultilingualSupport,
    #[doc = "Grasping the meaning and intent behind words and phrases."]
    SemanticUnderstanding,
    #[doc = "Continuing a given text prompt in a coherent and contextually appropriate manner to generate fluent and contextually relevant content."]
    TextCompletion,
    #[doc = "Rewriting text to match the style of a given reference text while preserving the original content."]
    TextStyleTransfer,
    #[doc = "Tagging each part of a sentence as nouns, adjectives, verbs, and so on."]
    PartOfSpeechTagging,
    #[doc = "Representing parts of text with vectors to be used as input to other tasks."]
    ModelModuleExtraction,
    #[doc = "Capabilities for ensuring ethical, unbiased, and safe content generation and interaction."]
    EthicalAndSafeInteraction,
    #[doc = "Task to recognize names as entity, for example, people, locations, buildings, and so on."]
    NamedEntityRecognition,
    #[doc = "Avoiding the generation of harmful, inappropriate, or sensitive content."]
    ContentModeration,
    #[doc = "Producing conversational responses that are contextually relevant and engaging within a dialogue context."]
    DialogueGeneration,
    #[doc = "Capabilities for retrieving relevant information from various sources and synthesizing it into coherent, contextually appropriate responses. This includes searching, extracting, combining, and presenting information in a meaningful way."]
    InformationRetrievalAndSynthesis,
    #[doc = "Identifying and categorizing key entities within the text, such as names, dates, or locations."]
    EntityRecognition,
    #[doc = "Capabilities for classifying individual tokens or words within text."]
    TokenClassification,
    #[doc = "Capabilities for generating various forms of creative content, including narratives, poetry, and other creative writing forms."]
    CreativeContentGeneration,
    #[doc = "Capability to perform efficient and accurate searches within large textual databases based on various criteria, including keywords, semantic meaning, or complex queries."]
    InformationRetrievalSynthesisSearch,
    #[doc = "System capability to understand questions and provide accurate, relevant answers by analyzing available information sources."]
    QuestionAnswering,
    #[doc = "Modifying the tone or style of generated text to suit specific audiences or purposes."]
    ToneAndStyleAdjustment,
    #[doc = "Capability to aggregate and combine information from multiple sources, creating comprehensive and coherent responses while maintaining context and relevance."]
    KnowledgeSynthesis,
    #[doc = "Condensing longer texts into concise summaries while preserving essential information and maintaining coherence."]
    TextSummarization,
    #[doc = "Classify the sentiment of a text, that is, a positive movie review."]
    SentimentAnalysis,
    #[doc = "Recognizing and processing text in multiple languages."]
    MultilingualUnderstanding,
    #[doc = "Composing poems, prose, or other forms of creative literature."]
    PoetryAndCreativeWriting,
    #[doc = "Classifying a text as belong to one of several topics, which can be used to tag a text."]
    TopicLabellingAndTagging,
    #[doc = "Capabilities for adapting and personalizing content based on user context and preferences."]
    PersonalisationAndAdaptation,
    #[doc = "Making logical inferences based on provided information."]
    InferenceAndDeduction,
    #[doc = "Images / Computer Vision tasks are the application of computational techniques to the analysis and synthesis of images."]
    ImagesComputerVision,
    #[doc = "Predicting the distance or depth of objects within a scene from a single image or multiple images."]
    DepthEstimation,
    #[doc = "Assigning labels or categories to images based on their visual content."]
    ImageClassification,
    #[doc = "Identifying and isolating key characteristics or patterns from an image to aid in tasks like classification or recognition."]
    ImageModuleExtraction,
    #[doc = "Creating new images from learned patterns or data using machine learning models."]
    ImageGeneration,
    #[doc = "Assigning labels or categories to images based on their visual content."]
    ImageSegmentation,
    #[doc = "The process of converting a 2D image into a 3D representation or model, often by inferring depth and spatial relationships."]
    ImageTo3d,
    #[doc = "Transforming one image into another using a learned mapping, often for tasks like style transfer, colorization, or image enhancement."]
    ImageToImage,
    #[doc = "Identifying and locating specific points of interest within an image or object."]
    KeypointDetection,
    #[doc = "Producing segmented regions in an image to highlight specific areas or objects, typically represented as separate layers or overlays."]
    MaskGeneration,
    #[doc = "Identifying and locating specific objects within an image or video, often by drawing bounding boxes around them."]
    ObjectDetection,
    #[doc = "Assigning labels or categories to entire videos or segments based on their visual and audio content."]
    VideoClassification,
    #[doc = "Audio tasks are the application of computational techniques to the analysis and synthesis of audio data."]
    Audio,
    #[doc = "Assigning labels or classes to audio content based on its characteristics."]
    AudioClassification,
    #[doc = "Transforming audio through various manipulations including cutting, filtering, and mixing."]
    AudioToAudio,
    #[doc = "Tabular / Text tasks are the application of computational techniques to the analysis and synthesis of tabular data and text."]
    TabularText,
    #[doc = "Classifying data based on attributes using classical machine learning approaches."]
    TabularClassification,
    #[doc = "Predicting numerical values based on tabular attributes and features."]
    TabularRegression,
    #[doc = "Analytical skills encompass a range of capabilities that involve logical reasoning, problem-solving, and the ability to process and interpret complex data."]
    AnalyticalSkills,
    #[doc = "Rewriting and optimizing existing code through refactoring techniques."]
    CodeRefactoringAndOptimization,
    #[doc = "Automatically filling in code templates with appropriate content."]
    CodeTemplateFilling,
    #[doc = "Generating natural language documentation for code segments."]
    CodeToDocstrings,
    #[doc = "Capabilities for code generation, documentation, and optimization."]
    CodingSkills,
    #[doc = "Translating natural language instructions into executable code."]
    TextToCode,
    #[doc = "Solving geometric problems and spatial reasoning tasks."]
    Geometry,
    #[doc = "Solving mathematical exercises presented in natural language format."]
    MathWordProblems,
    #[doc = "Capabilities for solving mathematical problems and proving theorems."]
    MathematicalReasoning,
    #[doc = "Executing pure mathematical operations, such as arithmetic calculations."]
    PureMathematicalOperations,
    #[doc = "Proving mathematical theorems using computational methods."]
    AutomatedTheoremProving,
    #[doc = "Retrieval Augmented Generation tasks are the application of computational techniques to the analysis and synthesis of data from multiple modalities."]
    RetrievalAugmentedGeneration,
    #[doc = "Document or database question answering is the process of retrieving and using information from a document or database to answer a specific question."]
    DocumentOrDatabaseQuestionAnswering,
    #[doc = "Generation of any is augmenting the creation of text, images, audio, or other media by incorporating retrieved information to improve or guide the generation process."]
    GenerationOfAny,
    #[doc = "Document retrieval is the process of retrieving relevant documents from a collection based on a specific query, typically through indexing and search techniques."]
    DocumentRetrieval,
    #[doc = "Depth estimations the task of predicting the distance or depth of objects within a scene from a single image or multiple images."]
    Indexing,
    #[doc = "Retrieval of information is the process of fetching relevant data or documents from a large dataset or database based on a specific query or input."]
    RetrievalOfInformation,
    #[doc = "Search is the process of exploring a dataset or index to find relevant information or results based on a given query."]
    RetrievalOfInformationSearch,
    #[doc = "Multi-modal tasks are the application of computational techniques to the analysis and synthesis of data from multiple modalities."]
    MultiModal,
    #[doc = "Converting between any supported modalities (text, image, audio, video, or 3D)."]
    AnyToAnyTransformation,
    #[doc = "Capabilities for processing audio, including speech synthesis and recognition."]
    AudioProcessing,
    #[doc = "Converting spoken language into written text."]
    AutomaticSpeechRecognition,
    #[doc = "Converting text into natural-sounding speech audio."]
    TextToSpeech,
    #[doc = "Capabilities for processing and generating images from various inputs and generating textual descriptions of visual content."]
    ImageProcessing,
    #[doc = "Generating textual descriptions or captions for images."]
    ImageToText,
    #[doc = "Generating 3D objects or scenes based on textual descriptions."]
    TextTo3d,
    #[doc = "Generating images based on textual descriptions or instructions."]
    TextToImage,
    #[doc = "Generating video content based on textual descriptions or instructions."]
    TextToVideo,
    #[doc = "Answering questions about images using natural language."]
    VisualQuestionAnswering,

}

impl From<Skill> for &str {
    fn from(value: Skill) -> &'static str {
        match value {
            Skill::NaturalLanguageProcessing => "natural_language_processing",
            Skill::Storytelling => "natural_language_processing/creative_content/storytelling",
            Skill::FactAndClaimVerification => "natural_language_processing/analytical_reasoning/fact_verification",
            Skill::NaturalLanguageGeneration => "natural_language_processing/natural_language_generation",
            Skill::BiasMitigation => "natural_language_processing/ethical_interaction/bias_mitigation",
            Skill::StoryGeneration => "natural_language_processing/natural_language_generation/story_generation",
            Skill::SentenceSimilarity => "natural_language_processing/information_retrieval_synthesis/sentence_similarity",
            Skill::TextClassification => "natural_language_processing/text_classification",
            Skill::DocumentAndPassageRetrieval => "natural_language_processing/information_retrieval_synthesis/document_passage_retrieval",
            Skill::NaturalLanguageInference => "natural_language_processing/text_classification/natural_language_inference",
            Skill::ModuleExtraction => "natural_language_processing/feature_extraction",
            Skill::NaturalLanguageUnderstanding => "natural_language_processing/natural_language_understanding",
            Skill::ProblemSolving => "natural_language_processing/analytical_reasoning/problem_solving",
            Skill::UserAdaptation => "natural_language_processing/personalization/user_adaptation",
            Skill::QuestionGeneration => "natural_language_processing/natural_language_generation/question_generation",
            Skill::ContextualComprehension => "natural_language_processing/natural_language_understanding/contextual_comprehension",
            Skill::Translation => "natural_language_processing/language_translation/translation",
            Skill::TextParaphrasing => "natural_language_processing/natural_language_generation/paraphrasing",
            Skill::AnalyticalAndLogicalReasoning => "natural_language_processing/analytical_reasoning",
            Skill::FactExtraction => "natural_language_processing/information_retrieval_synthesis/fact_extraction",
            Skill::LanguageTranslationAndMultilingualSupport => "natural_language_processing/language_translation",
            Skill::SemanticUnderstanding => "natural_language_processing/natural_language_understanding/semantic_understanding",
            Skill::TextCompletion => "natural_language_processing/natural_language_generation/text_completion",
            Skill::TextStyleTransfer => "natural_language_processing/natural_language_generation/text_style_transfer",
            Skill::PartOfSpeechTagging => "natural_language_processing/token_classification/pos_tagging",
            Skill::ModelModuleExtraction => "natural_language_processing/feature_extraction/model_feature_extraction",
            Skill::EthicalAndSafeInteraction => "natural_language_processing/ethical_interaction",
            Skill::NamedEntityRecognition => "natural_language_processing/token_classification/named_entity_recognition",
            Skill::ContentModeration => "natural_language_processing/ethical_interaction/content_moderation",
            Skill::DialogueGeneration => "natural_language_processing/natural_language_generation/dialogue_generation",
            Skill::InformationRetrievalAndSynthesis => "natural_language_processing/information_retrieval_synthesis",
            Skill::EntityRecognition => "natural_language_processing/natural_language_understanding/entity_recognition",
            Skill::TokenClassification => "natural_language_processing/token_classification",
            Skill::CreativeContentGeneration => "natural_language_processing/creative_content",
            Skill::InformationRetrievalSynthesisSearch => "natural_language_processing/information_retrieval_synthesis/information_retrieval_synthesis_search",
            Skill::QuestionAnswering => "natural_language_processing/information_retrieval_synthesis/question_answering",
            Skill::ToneAndStyleAdjustment => "natural_language_processing/personalization/style_adjustment",
            Skill::KnowledgeSynthesis => "natural_language_processing/information_retrieval_synthesis/knowledge_synthesis",
            Skill::TextSummarization => "natural_language_processing/natural_language_generation/summarization",
            Skill::SentimentAnalysis => "natural_language_processing/text_classification/sentiment_analysis",
            Skill::MultilingualUnderstanding => "natural_language_processing/language_translation/multilingual_understanding",
            Skill::PoetryAndCreativeWriting => "natural_language_processing/creative_content/poetry_writing",
            Skill::TopicLabellingAndTagging => "natural_language_processing/text_classification/topic_labeling",
            Skill::PersonalisationAndAdaptation => "natural_language_processing/personalization",
            Skill::InferenceAndDeduction => "natural_language_processing/analytical_reasoning/inference_deduction",
            Skill::ImagesComputerVision => "images_computer_vision",
            Skill::DepthEstimation => "images_computer_vision/depth_estimation",
            Skill::ImageClassification => "images_computer_vision/image_classification",
            Skill::ImageModuleExtraction => "images_computer_vision/image_feature_extraction",
            Skill::ImageGeneration => "images_computer_vision/image_generation",
            Skill::ImageSegmentation => "images_computer_vision/image_segmentation",
            Skill::ImageTo3d => "images_computer_vision/image_to_3d",
            Skill::ImageToImage => "images_computer_vision/image_to_image",
            Skill::KeypointDetection => "images_computer_vision/keypoint_detection",
            Skill::MaskGeneration => "images_computer_vision/mask_generation",
            Skill::ObjectDetection => "images_computer_vision/object_detection",
            Skill::VideoClassification => "images_computer_vision/video_classification",
            Skill::Audio => "audio",
            Skill::AudioClassification => "audio/audio_classification",
            Skill::AudioToAudio => "audio/audio_to_audio",
            Skill::TabularText => "tabular_text",
            Skill::TabularClassification => "tabular_text/tabular_classification",
            Skill::TabularRegression => "tabular_text/tabular_regression",
            Skill::AnalyticalSkills => "analytical_skills",
            Skill::CodeRefactoringAndOptimization => "analytical_skills/coding_skills/code_optimization",
            Skill::CodeTemplateFilling => "analytical_skills/coding_skills/code_templates",
            Skill::CodeToDocstrings => "analytical_skills/coding_skills/code_to_docstrings",
            Skill::CodingSkills => "analytical_skills/coding_skills",
            Skill::TextToCode => "analytical_skills/coding_skills/text_to_code",
            Skill::Geometry => "analytical_skills/mathematical_reasoning/geometry",
            Skill::MathWordProblems => "analytical_skills/mathematical_reasoning/math_word_problems",
            Skill::MathematicalReasoning => "analytical_skills/mathematical_reasoning",
            Skill::PureMathematicalOperations => "analytical_skills/mathematical_reasoning/pure_math_operations",
            Skill::AutomatedTheoremProving => "analytical_skills/mathematical_reasoning/theorem_proving",
            Skill::RetrievalAugmentedGeneration => "retrieval_augmented_generation",
            Skill::DocumentOrDatabaseQuestionAnswering => "retrieval_augmented_generation/document_or_database_question_answering",
            Skill::GenerationOfAny => "retrieval_augmented_generation/generation_of_any",
            Skill::DocumentRetrieval => "retrieval_augmented_generation/retrieval_of_information/document_retrieval",
            Skill::Indexing => "retrieval_augmented_generation/retrieval_of_information/indexing",
            Skill::RetrievalOfInformation => "retrieval_augmented_generation/retrieval_of_information",
            Skill::RetrievalOfInformationSearch => "retrieval_augmented_generation/retrieval_of_information/retrieval_of_information_search",
            Skill::MultiModal => "multi_modal",
            Skill::AnyToAnyTransformation => "multi_modal/any_to_any",
            Skill::AudioProcessing => "multi_modal/audio_processing",
            Skill::AutomaticSpeechRecognition => "multi_modal/audio_processing/speech_recognition",
            Skill::TextToSpeech => "multi_modal/audio_processing/text_to_speech",
            Skill::ImageProcessing => "multi_modal/image_processing",
            Skill::ImageToText => "multi_modal/image_processing/image_to_text",
            Skill::TextTo3d => "multi_modal/image_processing/text_to_3d",
            Skill::TextToImage => "multi_modal/image_processing/text_to_image",
            Skill::TextToVideo => "multi_modal/image_processing/text_to_video",
            Skill::VisualQuestionAnswering => "multi_modal/image_processing/visual_qa",

        }
    }
}

impl From<Skill> for u32 {
    fn from(value: Skill) -> u32 {
        match value {
            Skill::NaturalLanguageProcessing => 1,
            Skill::Storytelling => 10401,
            Skill::FactAndClaimVerification => 10703,
            Skill::NaturalLanguageGeneration => 102,
            Skill::BiasMitigation => 10801,
            Skill::StoryGeneration => 10207,
            Skill::SentenceSimilarity => 10304,
            Skill::TextClassification => 109,
            Skill::DocumentAndPassageRetrieval => 10305,
            Skill::NaturalLanguageInference => 10903,
            Skill::ModuleExtraction => 110,
            Skill::NaturalLanguageUnderstanding => 101,
            Skill::ProblemSolving => 10702,
            Skill::UserAdaptation => 10601,
            Skill::QuestionGeneration => 10205,
            Skill::ContextualComprehension => 10101,
            Skill::Translation => 10501,
            Skill::TextParaphrasing => 10203,
            Skill::AnalyticalAndLogicalReasoning => 107,
            Skill::FactExtraction => 10301,
            Skill::LanguageTranslationAndMultilingualSupport => 105,
            Skill::SemanticUnderstanding => 10102,
            Skill::TextCompletion => 10201,
            Skill::TextStyleTransfer => 10206,
            Skill::PartOfSpeechTagging => 11102,
            Skill::ModelModuleExtraction => 11001,
            Skill::EthicalAndSafeInteraction => 108,
            Skill::NamedEntityRecognition => 11101,
            Skill::ContentModeration => 10802,
            Skill::DialogueGeneration => 10204,
            Skill::InformationRetrievalAndSynthesis => 103,
            Skill::EntityRecognition => 10103,
            Skill::TokenClassification => 111,
            Skill::CreativeContentGeneration => 104,
            Skill::InformationRetrievalSynthesisSearch => 10306,
            Skill::QuestionAnswering => 10302,
            Skill::ToneAndStyleAdjustment => 10602,
            Skill::KnowledgeSynthesis => 10303,
            Skill::TextSummarization => 10202,
            Skill::SentimentAnalysis => 10902,
            Skill::MultilingualUnderstanding => 10502,
            Skill::PoetryAndCreativeWriting => 10402,
            Skill::TopicLabellingAndTagging => 10901,
            Skill::PersonalisationAndAdaptation => 106,
            Skill::InferenceAndDeduction => 10701,
            Skill::ImagesComputerVision => 2,
            Skill::DepthEstimation => 207,
            Skill::ImageClassification => 203,
            Skill::ImageModuleExtraction => 208,
            Skill::ImageGeneration => 206,
            Skill::ImageSegmentation => 201,
            Skill::ImageTo3d => 211,
            Skill::ImageToImage => 210,
            Skill::KeypointDetection => 205,
            Skill::MaskGeneration => 209,
            Skill::ObjectDetection => 204,
            Skill::VideoClassification => 202,
            Skill::Audio => 3,
            Skill::AudioClassification => 301,
            Skill::AudioToAudio => 302,
            Skill::TabularText => 4,
            Skill::TabularClassification => 401,
            Skill::TabularRegression => 402,
            Skill::AnalyticalSkills => 5,
            Skill::CodeRefactoringAndOptimization => 50204,
            Skill::CodeTemplateFilling => 50203,
            Skill::CodeToDocstrings => 50202,
            Skill::CodingSkills => 502,
            Skill::TextToCode => 50201,
            Skill::Geometry => 50103,
            Skill::MathWordProblems => 50102,
            Skill::MathematicalReasoning => 501,
            Skill::PureMathematicalOperations => 50101,
            Skill::AutomatedTheoremProving => 50104,
            Skill::RetrievalAugmentedGeneration => 6,
            Skill::DocumentOrDatabaseQuestionAnswering => 602,
            Skill::GenerationOfAny => 603,
            Skill::DocumentRetrieval => 60103,
            Skill::Indexing => 60101,
            Skill::RetrievalOfInformation => 601,
            Skill::RetrievalOfInformationSearch => 60102,
            Skill::MultiModal => 7,
            Skill::AnyToAnyTransformation => 703,
            Skill::AudioProcessing => 702,
            Skill::AutomaticSpeechRecognition => 70202,
            Skill::TextToSpeech => 70201,
            Skill::ImageProcessing => 701,
            Skill::ImageToText => 70101,
            Skill::TextTo3d => 70104,
            Skill::TextToImage => 70102,
            Skill::TextToVideo => 70103,
            Skill::VisualQuestionAnswering => 70105,

        }
    }
}