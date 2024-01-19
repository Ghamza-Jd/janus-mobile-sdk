use crate::jacallback::JaCallback;
use crate::jaconfig::JaConfig;
use crate::jaerror::JaError;
use jarust::connect as jarust_connect;
use jarust::jaconfig::JaConfig as JarustConfig;
use jarust::jaconfig::TransportType;

pub struct JaConnection {
    rt: tokio::runtime::Runtime,
}

impl JaConnection {
    pub fn new() -> Result<Self, JaError> {
        let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("jarust-scheduler")
            .enable_all()
            .build()
        else {
            return Err(JaError::RuntimeCreationFailure);
        };

        Ok(Self { rt })
    }

    pub fn connect(&self, config: JaConfig, cb: Box<dyn JaCallback>) {
        let root_namespace = config.root_namespace.unwrap_or(String::from("janus"));
        let config = JarustConfig::new(
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
