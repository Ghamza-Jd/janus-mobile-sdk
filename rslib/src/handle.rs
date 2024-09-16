use jarust::prelude::JaHandle;
use jarust::prelude::JaResponse;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct Handle {
    inner: JaHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<JaResponse>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl Handle {
    pub fn new(handle: JaHandle, receiver: mpsc::UnboundedReceiver<JaResponse>) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }
}
