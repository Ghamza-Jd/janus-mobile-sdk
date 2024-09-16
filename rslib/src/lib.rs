pub mod config;
pub mod connection;
pub mod error;
pub mod logger;
pub mod session;

pub type JanusGatewayResult<T> = core::result::Result<T, error::JanusGatewayError>;

uniffi::setup_scaffolding!();
