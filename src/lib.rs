mod callback;
mod config;
mod connection;
mod context;
mod error;
mod logger;
mod session;

use crate::callback::RawJaConnectionCallback;
use crate::config::RawJaConfig;
use crate::connection::RawJaConnection;
use crate::context::RawJaContext;
use crate::error::RawJaError;
use crate::logger::raw_jarust_init_logger;
use crate::session::RawJaSession;
use std::sync::Arc;

pub fn raw_jarust_connect(
    ctx: Arc<RawJaContext>,
    config: RawJaConfig,
    cb: Box<dyn RawJaConnectionCallback>,
) {
    let root_namespace = config.root_namespace.unwrap_or(String::from("janus"));
    let config = jarust::jaconfig::JaConfig::new(&config.uri, config.apisecret, &root_namespace);
    ctx.rt.spawn(async move {
        match jarust::connect(config, jarust::jaconfig::TransportType::Wss).await {
            Ok(conn) => cb.on_connection_success(Arc::new(RawJaConnection::new(conn))),
            Err(_) => cb.on_connection_failure(),
        }
    });
}

uniffi::include_scaffolding!("jarust");
