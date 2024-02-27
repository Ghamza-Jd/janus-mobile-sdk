use crate::context::RawJaContext;
use jarust::japrotocol::Jsep;
use jarust::japrotocol::JsepType;
use jarust_plugins::echotest::events::EchoTestPluginEvent;
use jarust_plugins::echotest::handle::EchoTestHandle;
use jarust_plugins::echotest::messages::EchoTestStartMsg;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

pub struct RawEchotestHandle {
    handle: EchoTestHandle,
    receiver: Mutex<Option<mpsc::Receiver<EchoTestPluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl RawEchotestHandle {
    pub fn new(handle: EchoTestHandle, receiver: mpsc::Receiver<EchoTestPluginEvent>) -> Self {
        Self {
            handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }

    pub fn start(&self, ctx: Arc<RawJaContext>, msg: RawEchotestStartMsg) {
        let handle = self.handle.clone();
        ctx.rt.spawn(async move {
            _ = handle.start(msg.into(), Duration::from_secs(5)).await;
        });
    }

    pub fn assign_handler(&self, ctx: Arc<RawJaContext>, cb: Box<dyn RawEchotestEventsCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };
        let join_handle = ctx.rt.spawn(async move {
            while let Some(item) = receiver.recv().await {
                match item {
                    EchoTestPluginEvent::Result { echotest, result } => {
                        cb.on_result(echotest, result)
                    }
                }
            }
        });
        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }
}

impl Drop for RawEchotestHandle {
    fn drop(&mut self) {
        if let Ok(Some(abort_handle)) = self.abort_handle.lock().map(|mut x| x.take()) {
            abort_handle.abort();
        }
    }
}

pub struct RawEchotestStartMsg {
    pub audio: bool,
    pub video: bool,
    pub bitrate: Option<u32>,
    pub jsep: Option<RawJsep>,
}

pub struct RawJsep {
    pub sdp: String,
    pub jsep_type: RawJsepType,
}

pub enum RawJsepType {
    Offer,
    Answer,
}

impl Into<EchoTestStartMsg> for RawEchotestStartMsg {
    fn into(self) -> EchoTestStartMsg {
        let jsep = match self.jsep {
            Some(raw_jsep) => Some(Jsep {
                sdp: raw_jsep.sdp,
                jsep_type: match raw_jsep.jsep_type {
                    RawJsepType::Offer => JsepType::Offer,
                    RawJsepType::Answer => JsepType::Answer,
                },
            }),
            None => None,
        };
        EchoTestStartMsg {
            audio: self.audio,
            video: self.video,
            jsep,
            bitrate: self.bitrate,
        }
    }
}

pub trait RawEchotestEventsCallback: Send + Sync + Debug {
    fn on_result(&self, echotest: String, result: String);
}
