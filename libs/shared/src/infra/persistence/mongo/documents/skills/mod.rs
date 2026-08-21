pub mod document_to_entity;
pub mod entity_to_document;
pub mod input_to_document;

use serde::{Serialize, Deserialize};

#[doc = "A structured view of distinct abilities, defining the capabilities within the Open Agentic Schema Framework."]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
    #[doc = "Capability to perform efficient and accurate searches within large textual databases based on various criteria, including tags, semantic meaning, or complex queries."]
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