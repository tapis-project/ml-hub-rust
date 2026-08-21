use crate::domain::entities::deployment as entities;
use crate::presentation::http::v1::requests::deployment::ParallelismStrategy;



impl From<ParallelismStrategy> for entities::ParallelismStrategy {
    fn from(value: ParallelismStrategy) -> Self {
        match value {
            ParallelismStrategy::ContextParallelism => entities::ParallelismStrategy::ContextParallelism,
            ParallelismStrategy::ExpertParallelism => entities::ParallelismStrategy::ExpertParallelism,
            ParallelismStrategy::PipelineParallelism => entities::ParallelismStrategy::PipelineParallelism,
            ParallelismStrategy::SequenceParallelism => entities::ParallelismStrategy::SequenceParallelism,
            ParallelismStrategy::TensorParallelism => entities::ParallelismStrategy::TensorParallelism
        }
    }
}