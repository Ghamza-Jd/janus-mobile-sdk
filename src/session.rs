use crate::callback::RawJaSessionCallback;
use crate::context::RawJaContext;
use crate::handle::RawJaHandle;
use crate::RawEchotestHandle;
use jarust::japlugin::Attach;
use jarust::jasession::JaSession;
use jarust_plugins::echotest::EchoTest;
use std::sync::Arc;

pub struct RawJaSession {
    session: JaSession,
}

impl RawJaSession {
    pub fn new(session: JaSession) -> Self {
        Self { session }
    }

    pub fn attach(
        &self,
        ctx: Arc<RawJaContext>,
        plugin_id: String,
        cb: Box<dyn RawJaSessionCallback>,
    ) {
        let session = self.session.clone();
        ctx.rt.spawn(async move {
            match session.attach(&plugin_id).await {
                Ok((handle, receiver)) => {
                    cb.on_attach_success(Arc::new(RawJaHandle::new(handle, receiver)))
                }
                Err(_) => cb.on_attach_failure(),
            }
        });
    }

    pub fn attach_echotest(&self, ctx: Arc<RawJaContext>, cb: Box<dyn RawJaSessionCallback>) {
        let session = self.session.clone();
        ctx.rt.spawn(async move {
            match session.attach_echo_test().await {
                Ok((handle, receiver)) => cb
                    .on_attach_echotest_success(Arc::new(RawEchotestHandle::new(handle, receiver))),
                Err(_) => cb.on_attach_echotest_failure(),
            }
        });
    }
}
