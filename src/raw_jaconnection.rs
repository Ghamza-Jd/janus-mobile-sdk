use crate::raw_jacallback::RawJaCallback;
use crate::raw_jaconfig::RawJaConfig;
use crate::raw_jaerror::RawJaError;
use jarust::connect as jarust_connect;
use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;

pub struct RawJaConnection {
    rt: tokio::runtime::Runtime,
}

impl RawJaConnection {
    pub fn new() -> Result<Self, RawJaError> {
        let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("jarust-scheduler")
            .enable_all()
            .build()
        else {
            return Err(RawJaError::RuntimeCreationFailure);
        };

        Ok(Self { rt })
    }

    pub fn connect(&self, config: RawJaConfig, cb: Box<dyn RawJaCallback>) {
        let root_namespace = config.root_namespace.unwrap_or(String::from("janus"));
        let config = JaConfig::new(
            &config.uri,
            config.apisecret,
            TransportType::Wss,
            &root_namespace,
        );
        self.rt.spawn(async move {
            match jarust_connect(config).await {
                Ok(_) => cb.on_connection_success(),
                Err(_) => cb.on_connection_failure(),
            }
        });
    }
}
