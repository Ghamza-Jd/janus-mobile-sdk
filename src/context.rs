use crate::error::RawJaError;

pub struct RawJaContext {
    pub(crate) rt: tokio::runtime::Runtime,
}

impl RawJaContext {
    pub fn new() -> Result<Self, RawJaError> {
        let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("jarust-runtime-worker")
            .enable_all()
            .build()
        else {
            return Err(RawJaError::RuntimeCreationFailure);
        };

        Ok(Self { rt })
    }
}
