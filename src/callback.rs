use crate::connection::RawJaConnection;
use crate::handle::RawJaHandle;
use crate::plugins::echotest::RawEchotestHandle;
use crate::session::RawJaSession;
use std::fmt::Debug;
use std::sync::Arc;

pub trait RawJaConnectionCallback: Send + Sync + Debug {
    fn on_connection_success(&self, connection: Arc<RawJaConnection>);
    fn on_connection_failure(&self);

    fn on_session_creation_success(&self, session: Arc<RawJaSession>);
    fn on_session_creation_failure(&self);
}

pub trait RawJaSessionCallback: Send + Sync + Debug {
    fn on_attach_success(&self, handle: Arc<RawJaHandle>);
    fn on_attach_failure(&self);

    fn on_attach_echotest_success(&self, handle: Arc<RawEchotestHandle>);
    fn on_attach_echotest_failure(&self);
}

pub trait RawJaEventsCallback: Send + Sync + Debug {
    fn on_event(&self, event: String);
}
