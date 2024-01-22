use crate::callback::RawJaConnectionCallback;
use crate::session::RawJaSession;
use std::sync::Arc;

pub struct RawJaConnection;

impl RawJaConnection {
    pub fn create(&self, ka_interval: u32, cb: Box<dyn RawJaConnectionCallback>) {
        cb.on_session_creation_success(Arc::new(RawJaSession))
    }
}
