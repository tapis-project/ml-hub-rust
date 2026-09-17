pub mod deployment;
pub mod reconciliation;

#[async_trait::async_trait]
pub trait Workflow<TInput, TResult, TError> {
    async fn run(&self, input: TInput) -> Result<TResult, TError>;
}
