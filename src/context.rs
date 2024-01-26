use crate::error::RawJaError;

pub struct RawJaContext {
    pub(crate) rt: tokio::runtime::Runtime,
}

impl RawJaContext {
    pub fn new(num_workers: Option<u8>, name: Option<String>) -> Result<Self, RawJaError> {
        let num_workers = num_workers.unwrap_or(1);
        let num_workers = if num_workers == 0 { 1 } else { num_workers };
        let name = name.unwrap_or(String::from("jarust-runtime-worker"));

        let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_workers.into())
            .thread_name(&name)
            .enable_all()
            .build()
        else {
            return Err(RawJaError::RuntimeCreationFailure);
        };

        Ok(Self { rt })
    }
}
