use std::time::Duration;

use crate::error::JanusGatewayError;
use crate::handle::Handle;
use jarust::japlugin::AttachHandleParams;
use jarust::jasession::JaSession;
use jarust::prelude::Attach;

#[derive(uniffi::Object)]
pub struct Session {
    inner: JaSession,
}

impl Session {
    pub fn new(session: JaSession) -> Self {
        Self { inner: session }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Session {
    pub async fn attach(
        &self,
        plugin_id: &str,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<Handle> {
        let (handle, receiver) = match self
            .inner
            .attach(AttachHandleParams {
                plugin_id: plugin_id.to_string(),
                timeout,
            })
            .await
        {
            Ok(handle) => handle,
            Err(why) => {
                return Err(JanusGatewayError::HandleCreationFailure {
                    plugin: plugin_id.to_string(),
                    reason: why.to_string(),
                })
            }
        };
        Ok(Handle::new(handle, receiver))
    }
}
