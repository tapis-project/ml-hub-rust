pub mod reconciliation;
pub mod deployment;

#[async_trait::async_trait]
pub trait Workflow<TInput, TResult, TError> {
    async fn run(&self, input: TInput) -> Result<TResult, TError>;
}