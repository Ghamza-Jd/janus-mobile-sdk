use crate::{jacallback::JaCallback, jaconfig::JaConfig};

pub struct JaConnection;

impl JaConnection {
    pub fn new() -> Self {
        Self
    }

    pub fn connect(
        &self,
        config: JaConfig,
        success_cb: Box<dyn JaCallback>,
        failure_cb: Box<dyn JaCallback>,
    ) {
    }
}
