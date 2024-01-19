#[derive(Debug, thiserror::Error)]
pub enum JaError {
    #[error("Failed to create runtime")]
    RuntimeCreationFailure,
}
