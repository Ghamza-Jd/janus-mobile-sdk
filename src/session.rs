use jarust::jasession::JaSession;

pub struct RawJaSession {
    session: JaSession,
}

impl RawJaSession {
    pub fn new(session: JaSession) -> Self {
        Self { session }
    }
}
