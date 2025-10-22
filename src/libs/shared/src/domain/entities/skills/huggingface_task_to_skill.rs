
use huggingface_tasks::task;
use crate::domain::entities::skills as entities;

impl From<task::Task> for entities::Skill {
    fn from(value: task::Task) -> Self {
        match value {
            task::Task::VideoClassification => entities::Skill::VideoClassification,
            task::Task::Translation => entities::Skill::Translation,
            task::Task::TextToImage => entities::Skill::TextToImage,
            task::Task::TextTo3d => entities::Skill::TextTo3d,
            task::Task::TextClassification => entities::Skill::TextClassification,
            task::Task::TabularClassification => entities::Skill::TabularClassification,
            task::Task::SentenceSimilarity => entities::Skill::SentenceSimilarity,
            task::Task::QuestionAnswering => entities::Skill::QuestionAnswering,
            task::Task::ObjectDetection => entities::Skill::ObjectDetection,
            task::Task::KeypointDetection => entities::Skill::KeypointDetection,
            task::Task::ImageSegmentation => entities::Skill::ImageSegmentation,
            task::Task::ImageClassification => entities::Skill::ImageClassification,
            task::Task::DepthEstimation => entities::Skill::DepthEstimation,
            task::Task::AutomaticSpeechRecognition => entities::Skill::AutomaticSpeechRecognition,
            task::Task::AudioToAudio => entities::Skill::AudioToAudio,
            task::Task::AudioClassification => entities::Skill::AudioClassification,
            task::Task::ImageTo3d => entities::Skill::ImageTo3d,
            task::Task::ImageToImage => entities::Skill::ImageToImage,
            task::Task::ImageToText => entities::Skill::ImageToText,
            task::Task::MaskGeneration => entities::Skill::MaskGeneration,
            task::Task::TabularRegression => entities::Skill::TabularRegression,
            task::Task::TextToSpeech => entities::Skill::TextToSpeech,
            task::Task::TextToVideo => entities::Skill::TextToVideo,
            task::Task::TokenClassification => entities::Skill::TokenClassification,
            task::Task::VisualQuestionAnswering => entities::Skill::VisualQuestionAnswering,
            task::Task::AnyToAny => entities::Skill::AnyToAnyTransformation,
            // NOTE: Need a better conversion
            task::Task::AudioTextToText => entities::Skill::Audio,
            task::Task::DocumentQuestionAnswering => entities::Skill::DocumentOrDatabaseQuestionAnswering,
            // NOTE: Need a better conversion
            task::Task::VisualDocumentRetrieval => entities::Skill::InformationRetrievalAndSynthesis,
            
            task::Task::FeatureExtraction => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::FillMask => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ImageFeatureExtraction => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ImageTextToText => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ImageToVideo => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ReinforcementLearning => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::Summarization => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::TableQuestionAnswering => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::TextGeneration => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::TextRanking => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::UnconditionalImageGeneration => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::VideoTextToText => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::VideoToVideo => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ZeroShotClassification => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ZeroShotImageClassification => entities::Skill::AnalyticalAndLogicalReasoning,
            task::Task::ZeroShotObjectDetection => entities::Skill::AnalyticalAndLogicalReasoning,
        }
    }
}