use crate::context_provider::{AutoContextProvider, ContextProvider};
use crate::time::now_timestamp_ms;
use block_insight_cross::protos::messages::client::ClientMessage;
use block_insight_cross::protos::messages::client::client_message::Payload;
use block_insight_cross::protos::messages::heartbeat::HeartBeat;
use dioxus::core::Task;
use dioxus::hooks::{UnboundedReceiver, use_coroutine, use_signal};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
pub use ewebsock::WsEvent;
use ewebsock::{Options, WsMessage, WsSender, ws_connect};
use futures_util::StreamExt;
use prost::Message;
use std::ops::ControlFlow;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("websocket还未连接")]
    NotConnected,
}

pub struct WebSocket {}
impl WebSocket {
    pub fn use_web_socket_provider() {
        HeartbeatTask::use_context_provider();
        use_coroutine(
            move |mut receiver: UnboundedReceiver<WebSocketCmd>| async move {
                let mut inner_state = InnerState {
                    state: WebSocketState::Init,
                    ws_sender: None,
                };

                while let Some(cmd) = receiver.next().await {
                    inner_state.handle_cmd(cmd);
                }
            },
        );
    }

    pub fn connected() -> bool {
        let HeartbeatTask { task } = HeartbeatTask::use_context();
        task.peek_unchecked().is_some()
    }

    /// 连接服务器，如果已经连接，会先关闭旧连接
    pub fn connect(url: String, cb: EventHandler) {
        let cmd_sender = Self::use_web_socket();
        cmd_sender.send(WebSocketCmd::Connect(url, cb));
        // 启动心跳
        let HeartbeatTask { mut task } = HeartbeatTask::use_context();
        let heartbeat_task = spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;

                let heartbeat = HeartBeat {
                    timestamp: now_timestamp_ms() as u64,
                };
                cmd_sender.send(WebSocketCmd::Send(Payload::HeartBeat(heartbeat)));
            }
        });
        // 移除旧的
        if let Some(old) = task.take() {
            old.cancel();
        }
        task.set(Some(heartbeat_task));
    }

    pub fn close() {
        let cmd_sender = Self::use_web_socket();
        cmd_sender.send(WebSocketCmd::Close);
        let mut heartbeat_task = use_context::<HeartbeatTask>();
        let task = heartbeat_task.task.take();
        task.map(|task| task.cancel());
    }

    pub fn send(msg: Payload) -> Result<(), WebSocketError> {
        if !Self::connected() {
            return Err(WebSocketError::NotConnected);
        }

        let cmd_sender = Self::use_web_socket();
        cmd_sender.send(WebSocketCmd::Send(msg));

        Ok(())
    }

    fn use_web_socket() -> Coroutine<WebSocketCmd> {
        use_coroutine_handle::<WebSocketCmd>()
    }
}

enum WebSocketCmd {
    Connect(String, EventHandler),
    Send(Payload),
    Close,
}

#[derive(Default)]
enum WebSocketState {
    #[default]
    Init,
    Connected(String),
}

struct InnerState {
    state: WebSocketState,
    ws_sender: Option<WsSender>,
}

impl InnerState {
    fn connect(&mut self, url: String, handler: EventHandler) {
        match &self.state {
            WebSocketState::Init => match ws_connect(url.clone(), Options::default(), handler) {
                Ok(sender) => {
                    self.ws_sender = Some(sender);
                    self.state = WebSocketState::Connected(url);
                }
                Err(e) => {
                    error!("连接websocket出错，url: {url}, 原因: {e:?}");
                }
            },
            WebSocketState::Connected(old) => {
                error!("正在尝试重复连接websocket, 原url: {old}, 新url: {url}");
            }
        }
    }

    fn send(&mut self, message: Payload) {
        match &self.state {
            WebSocketState::Init => {
                error!("尝试在一个未连接的websocket上发送消息");
            }
            WebSocketState::Connected(_) => {
                if let Some(sender) = &mut self.ws_sender {
                    let message = ClientMessage {
                        payload: Some(message),
                    };
                    let data = message.encode_to_vec();
                    sender.send(WsMessage::Binary(data));
                } else {
                    error!("websocket状态异常：状态为[Connected]，但未设置[WsSender]");
                }
            }
        }
    }

    fn close(&mut self) {
        match &self.state {
            WebSocketState::Init => {
                error!("尝试关闭一个未连接的websocket");
            }
            WebSocketState::Connected(_) => {
                let ws_sender = self.ws_sender.take();
                match ws_sender {
                    None => {
                        self.state = WebSocketState::Init;
                        error!("尝试关闭websocket,但是[WsSender]不存在");
                    }
                    Some(mut ws_sender) => {
                        ws_sender.close();
                        self.state = WebSocketState::Init;
                    }
                }
            }
        }
    }

    fn handle_cmd(&mut self, cmd: WebSocketCmd) {
        match cmd {
            WebSocketCmd::Connect(url, cb) => {
                self.connect(url, cb);
            }
            WebSocketCmd::Send(msg) => {
                self.send(msg);
            }
            WebSocketCmd::Close => {
                self.close();
            }
        }
    }
}

pub type EventHandler = Box<dyn Send + Fn(WsEvent) -> ControlFlow<()>>;

#[derive(Default, Clone, Copy)]
struct HeartbeatTask {
    task: Signal<Option<Task>>,
}

impl ContextProvider for HeartbeatTask {
    fn instance() -> Self {
        let task = use_signal(|| None);
        HeartbeatTask { task }
    }
}

// impl HeartbeatTask {
//     pub fn use_context() -> Self {
//         use_context()
//     }
// }
