use crate::callback::RawJaEventsCallback;
use crate::context::RawJaContext;
use jarust::jahandle::JaHandle;
use jarust::japrotocol::JaResponse;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

pub struct RawJaHandle {
    handle: JaHandle,
    receiver: Mutex<Option<mpsc::Receiver<JaResponse>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl RawJaHandle {
    pub fn new(handle: JaHandle, receiver: mpsc::Receiver<JaResponse>) -> Self {
        Self {
            handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }

    pub fn assign_handler(&self, ctx: Arc<RawJaContext>, cb: Box<dyn RawJaEventsCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };
        let join_handle = ctx.rt.spawn(async move {
            while let Some(item) = receiver.recv().await {
                if let Ok(item) = serde_json::to_string(&item) {
                    cb.on_event(item);
                }
            }
        });
        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }

    pub fn message(&self, ctx: Arc<RawJaContext>, message: String) {
        let body = serde_json::from_str(&message).unwrap();
        let handle = self.handle.clone();
        ctx.rt.spawn(async move {
            _ = handle.message(body).await;
        });
    }
}

impl Drop for RawJaHandle {
    fn drop(&mut self) {
        if let Ok(Some(abort_handle)) = self.abort_handle.lock().map(|mut x| x.take()) {
            abort_handle.abort();
        }
    }
}
