#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JanusGatewayError {
    #[error("Failed to connect to server, reason: ${reason}")]
    ConnectionFailure { reason: String },
}
