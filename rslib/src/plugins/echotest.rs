use crate::error::JanusGatewayError;
use crate::japrotocol::Jsep;
use jarust_plugins::echo_test::events::EchoTestEvent;
use jarust_plugins::echo_test::events::PluginEvent;
use jarust_plugins::echo_test::handle::EchoTestHandle as JaEchoTestHandle;
use jarust_plugins::echo_test::msg_options::StartOptions;
use jarust_transport::japrotocol::EstablishmentProtocol;
use std::fmt::Debug;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct EchotestHandle {
    inner: JaEchoTestHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<PluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl EchotestHandle {
    pub fn new(handle: JaEchoTestHandle, receiver: mpsc::UnboundedReceiver<PluginEvent>) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl EchotestHandle {
    pub async fn start(
        &self,
        audio: Option<bool>,
        video: Option<bool>,
        bitrate: Option<u32>,
    ) -> crate::JanusGatewayResult<()> {
        if let Err(why) = self
            .inner
            .start(StartOptions {
                audio: audio.unwrap_or_default(),
                video: video.unwrap_or_default(),
                bitrate,
            })
            .await
        {
            return Err(JanusGatewayError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn start_with_jsep(
        &self,
        audio: Option<bool>,
        video: Option<bool>,
        bitrate: Option<u32>,
        jsep: Jsep,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<()> {
        if let Err(why) = self
            .inner
            .start_with_establishment(
                StartOptions {
                    audio: audio.unwrap_or_default(),
                    video: video.unwrap_or_default(),
                    bitrate,
                },
                EstablishmentProtocol::JSEP(jsep.into()),
                timeout,
            )
            .await
        {
            return Err(JanusGatewayError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn start_event_loop(&self, cb: Box<dyn EchotestHandleCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };

        let join_handle = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                match event {
                    PluginEvent::EchoTestEvent(EchoTestEvent::Result { echotest, result }) => {
                        cb.on_result(echotest, result);
                    }
                    PluginEvent::EchoTestEvent(EchoTestEvent::ResultWithEstablishment {
                        echotest,
                        result,
                        establishment_protocol,
                    }) => match establishment_protocol {
                        EstablishmentProtocol::JSEP(jsep) => {
                            cb.on_result_with_jsep(echotest, result, jsep.into());
                        }
                        EstablishmentProtocol::RTP(_) => {}
                    },
                    PluginEvent::GenericEvent(_) => {}
                }
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }
}

impl Drop for EchotestHandle {
    fn drop(&mut self) {
        if let Ok(Some(abort_handle)) = self.abort_handle.lock().map(|mut x| x.take()) {
            abort_handle.abort();
        }
    }
}

#[uniffi::export(callback_interface)]
pub trait EchotestHandleCallback: Send + Sync + Debug {
    fn on_result(&self, echotest: String, result: String);
    fn on_result_with_jsep(&self, echotest: String, result: String, jsep: Jsep);
}
