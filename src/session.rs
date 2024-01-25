use crate::callback::RawJaSessionCallback;
use crate::context::RawJaContext;
use crate::handle::RawJaHandle;
use jarust::japlugin::Attach;
use jarust::jasession::JaSession;
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
                Ok((handle, _)) => cb.on_attach_success(Arc::new(RawJaHandle::new(handle))),
                Err(_) => cb.on_attach_failure(),
            }
        });
    }
}
