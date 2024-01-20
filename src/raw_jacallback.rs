use std::fmt::Debug;

pub trait RawJaCallback: Send + Sync + Debug {
    fn on_connection_success(&self);
    fn on_connection_failure(&self);
}
