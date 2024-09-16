use crate::config::Config;
use crate::error::JanusGatewayError;
use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::jaconnection::JaConnection;
use jarust::TransactionGenerationStrategy;

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
