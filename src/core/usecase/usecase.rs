use crate::core::AppError;
use async_trait::async_trait;

// Generic UseCase trait with Input and Output type parameters
#[async_trait]
pub trait UseCase<Input, Output> {
    // Main execution method that all use cases must implement
    async fn execute(&self, input: Input) -> Result<Output, AppError>;
}

// // Optional: A trait for use cases that don't need input
// #[async_trait]
// pub trait NoInputUseCase<Output> {
//     async fn execute(&self) -> Result<Output, AppError>;
// }

// Optional: A trait for use cases that don't produce output (commands)
#[async_trait]
pub trait CommandUseCase<Input> {
    async fn execute(&self, input: Input) -> Result<(), AppError>;
}
