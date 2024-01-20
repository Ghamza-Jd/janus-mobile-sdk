use crate::connection::RawJaConnection;
use std::fmt::Debug;
use std::sync::Arc;

pub trait RawJaCallback: Send + Sync + Debug {
    fn on_connection_success(&self, connection: Arc<RawJaConnection>);
    fn on_connection_failure(&self);
}
