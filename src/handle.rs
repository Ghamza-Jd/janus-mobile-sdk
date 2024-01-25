use jarust::jahandle::JaHandle;

pub struct RawJaHandle {
    handle: JaHandle,
}

impl RawJaHandle {
    pub fn new(handle: JaHandle) -> Self {
        Self { handle }
    }
}
