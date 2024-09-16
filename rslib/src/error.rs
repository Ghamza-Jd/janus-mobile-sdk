#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JanusGatewayError {
    #[error("Failed to connect to server, reason: ${reason}")]
    ConnectionFailure { reason: String },
    #[error("Failed to create a session with server: reason ${reason}")]
    SessionCreationFailure { reason: String },
    #[error("Failed to attach ${plugin} handle, reason ${reason}")]
    HandleCreationFailure { plugin: String, reason: String },
    #[error("Could not serialize ${body}")]
    Serialize { body: String },
    #[error("Failed to send ${reason}")]
    SendFailure { reason: String },
}
