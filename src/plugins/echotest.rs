use crate::context::RawJaContext;
use jarust::japrotocol::JaResponse;
use jarust::japrotocol::Jsep;
use jarust::japrotocol::JsepType;
use jarust_plugins::echotest::handle::EchoTestHandle;
use jarust_plugins::echotest::messages::EchoTestStartMsg;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

pub struct RawEchotestHandle {
    handle: EchoTestHandle,
    receiver: Mutex<Option<mpsc::Receiver<JaResponse>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl RawEchotestHandle {
    pub fn new(handle: EchoTestHandle, receiver: mpsc::Receiver<JaResponse>) -> Self {
        Self {
            handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }

    pub fn start(&self, ctx: Arc<RawJaContext>, msg: RawEchotestStartMsg) {
        let handle = self.handle.clone();
        ctx.rt.spawn(async move {});
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
