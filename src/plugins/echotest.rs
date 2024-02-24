use crate::context::RawJaContext;
use jarust::japrotocol::JaResponse;
use jarust_plugins::echotest::handle::EchoTestHandle;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

pub struct RawEchotestHandle {
    handle: EchoTestHandle,
    receiver: Mutex<Option<mpsc::Receiver<JaResponse>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl RawEchotestHandle {
    pub fn new(handle: EchoTestHandle, receiver: mpsc::Receiver<JaResponse>) -> Self {
        Self {
            handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }

    pub fn start(&self, ctx: Arc<RawJaContext>) {}
}

impl Drop for RawEchotestHandle {
    fn drop(&mut self) {
        if let Ok(Some(abort_handle)) = self.abort_handle.lock().map(|mut x| x.take()) {
            abort_handle.abort();
        }
    }
}
