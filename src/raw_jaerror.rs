#[derive(Debug, thiserror::Error)]
pub enum RawJaError {
    #[error("Failed to create runtime")]
    RuntimeCreationFailure,
}
