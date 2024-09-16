use crate::config::Config;
use crate::error::JanusGatewayError;
use crate::session::Session;
use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::jaconnection::CreateConnectionParams;
use jarust::jaconnection::JaConnection;
use jarust::TransactionGenerationStrategy;
use std::time::Duration;

#[derive(uniffi::Object)]
pub struct Connection {
    inner: JaConnection,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn raw_janus_connect(config: Config) -> crate::JanusGatewayResult<Connection> {
    let mut builder = JaConfig::builder()
        .url(&config.url)
        .capacity(config.capacity.into());

    if let Some(apisecret) = config.apisecret {
        builder = builder.apisecret(&apisecret);
    }

    if let Some(namespace) = config.namespace {
        builder = builder.namespace(&namespace);
    }

    let config = builder.build();

    let connection = match jarust::connect(
        config,
        TransportType::Ws,
        TransactionGenerationStrategy::Random,
    )
    .await
    {
        Ok(connection) => connection,
        Err(why) => {
            return Err(JanusGatewayError::ConnectionFailure {
                reason: why.to_string(),
            })
        }
    };

    Ok(Connection { inner: connection })
}

#[uniffi::export(async_runtime = "tokio")]
impl Connection {
    pub async fn create_session(
        &self,
        ka_interval: u32,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<Session> {
        let mut connection = self.inner.clone();
        let session = match connection
            .create(CreateConnectionParams {
                ka_interval,
                timeout,
            })
            .await
        {
            Ok(session) => session,
            Err(why) => {
                return Err(JanusGatewayError::SessionCreationFailure {
                    reason: why.to_string(),
                })
            }
        };
        Ok(Session::new(session))
    }
}
