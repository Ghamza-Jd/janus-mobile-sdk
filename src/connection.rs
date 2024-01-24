use crate::callback::RawJaConnectionCallback;
use crate::context::RawJaContext;
use crate::session::RawJaSession;
use jarust::jaconnection::JaConnection;
use std::sync::Arc;

pub struct RawJaConnection {
    connection: JaConnection,
}

impl RawJaConnection {
    pub fn new(connection: JaConnection) -> Self {
        Self { connection }
    }

    pub fn create(
        &self,
        ctx: Arc<RawJaContext>,
        ka_interval: u32,
        cb: Box<dyn RawJaConnectionCallback>,
    ) {
        let mut conn = self.connection.clone();
        ctx.rt.spawn(async move {
            match conn.create(ka_interval).await {
                Ok(_) => cb.on_session_creation_success(Arc::new(RawJaSession)),
                Err(_) => cb.on_session_creation_failure(),
            }
        });
    }
}
