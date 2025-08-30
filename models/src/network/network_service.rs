use std::ops::ControlFlow;

use block_insight_cross::protos::messages::{
    client::client_message::Payload,
    server::{ServerMessage, server_message},
};
use dioxus::{
    core::{Task, spawn, spawn_forever},
    logger::tracing::{error, info},
};
use prost::{DecodeError, Message};
use utils::ws_cross::{WebSocketError, WsEvent};

use crate::{
    auth::auth_service::AuthService,
    constants::WS_BASE_URL,
    service::service_provider::{Service, ServiceProvider},
};

enum ParsedWsEvent {
    Opened,
    ServerMessage(ServerMessage),
    Text(String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    WebSocketError(String),
    Unknown(String),
    Closed,
}

impl TryFrom<WsEvent> for ParsedWsEvent {
    type Error = DecodeError;

    fn try_from(value: WsEvent) -> Result<Self, Self::Error> {
        match value {
            WsEvent::Opened => Ok(ParsedWsEvent::Opened),
            WsEvent::Message(ws_message) => match ws_message {
                ewebsock::WsMessage::Binary(items) => {
                    let server_message = ServerMessage::decode(items.as_slice());
                    match server_message {
                        Ok(server_message) => Ok(ParsedWsEvent::ServerMessage(server_message)),
                        Err(e) => {
                            error!("解析服务器数据时出错:{e:?}");
                            Err(e)
                        }
                    }
                }
                ewebsock::WsMessage::Text(text) => Ok(ParsedWsEvent::Text(text)),
                ewebsock::WsMessage::Unknown(unkonw) => Ok(ParsedWsEvent::Unknown(unkonw)),
                ewebsock::WsMessage::Ping(items) => Ok(ParsedWsEvent::Ping(items)),
                ewebsock::WsMessage::Pong(items) => Ok(ParsedWsEvent::Pong(items)),
            },
            WsEvent::Error(e) => Ok(ParsedWsEvent::WebSocketError(e)),
            WsEvent::Closed => Ok(ParsedWsEvent::Closed),
        }
    }
}

#[derive(Clone)]
pub struct NetworkService {
    task: Option<Task>,
}

impl Service for NetworkService {
    fn instance() -> Self {
        NetworkService { task: None }
    }
}

impl NetworkService {
    pub fn connect(&mut self) {
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<ParsedWsEvent>();
        let cb = move |event| {
            let parsed = ParsedWsEvent::try_from(event);
            match parsed {
                Ok(event) => match sender.send(event) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("收到服务器数据，解析成功，但是发送给ui线程时出错: {e:?}");
                    }
                },
                Err(e) => {
                    error!("解析服务器数据出错：{e:?}");
                }
            }

            ControlFlow::Continue(())
        };

        let mut context = ServerMessageHandlingContext::collect();
        let task = spawn_forever(async move {
            while let Some(event) = receiver.recv().await {
                Self::handle_ws_event(&mut context, event);
                todo!("handle event")
            }
        });
        let task = if let Some(task) = task {
            task
        } else {
            error!("启动数据处理进程失败");
            return;
        };

        let old_task = self.task.replace(task);
        if let Some(old_task) = old_task {
            old_task.cancel();
        }

        let cb = Box::new(cb);
        utils::ws_cross::WebSocket::connect(WS_BASE_URL.to_string(), cb);
    }

    pub fn send(&self, payload: Payload) -> Result<(), WebSocketError> {
        utils::ws_cross::WebSocket::send(payload)
    }
}

struct ServerMessageHandlingContext {
    auth_service: AuthService,
}

impl ServerMessageHandlingContext {
    pub fn collect() -> Self {
        let auth_service = AuthService::use_service();
        ServerMessageHandlingContext { auth_service }
    }
}

impl NetworkService {
    fn handle_ws_event(ctx: &mut ServerMessageHandlingContext, event: ParsedWsEvent) {
        match event {
            ParsedWsEvent::Opened => {
                //暂时没有逻辑可做
                info!("websocket连接打开了");
            }
            ParsedWsEvent::ServerMessage(server_message) => {
                Self::handle_server_message(ctx, server_message);
            }
            ParsedWsEvent::Text(text) => {
                info!("收到服务器文本消息:{text}");
            }
            ParsedWsEvent::Ping(_) => {
                // todo: handle ping
                info!("收到服务器的Ping消息");
            }
            ParsedWsEvent::Pong(_) => {
                // todo: handle pong
                info!("收到服务器的Pong消息");
            }
            ParsedWsEvent::WebSocketError(e) => {
                error!("websocket出现了错误: {e}");
            }
            ParsedWsEvent::Unknown(unkonwn) => {
                info!("收到服务器未知消息:{unkonwn}");
            }
            ParsedWsEvent::Closed => {
                info!("websocket连接关闭了")
            }
        }
    }

    fn handle_server_message(
        context: &mut ServerMessageHandlingContext,
        server_message: ServerMessage,
    ) {
        if server_message.code != 0 {
            error!(
                "服务器回复了错误, code: {}, msg: {:?}",
                server_message.code,
                server_message.error_message()
            );
            return;
        }
        match server_message.payload {
            Some(payload) => match payload {
                server_message::Payload::HeartBeat(heart_beat) => {
                    info!("收到心跳，暂时不处理: {}", heart_beat.timestamp);
                }
                server_message::Payload::LoginResp(login_resp) => {
                    let auth_service = AuthService::use_service();
                }
                server_message::Payload::LogoutResp(logout_resp) => todo!(),
                server_message::Payload::FetchUserResp(fetch_user_resp) => todo!(),
            },
            None => {
                error!("服务器回复了消息，但内容为空");
            }
        }
        todo!("handle server message");
    }
}
