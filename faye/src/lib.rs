#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate tungstenite;
extern crate url;

pub mod bayeux;
use bayeux::*;

pub mod message_handler;
pub use message_handler::MessageHandler;

use std::collections::HashMap;
use tungstenite::client::AutoStream;
use tungstenite::{Message as WsMsg, WebSocket};
use url::Url;

#[derive(Debug)]
pub enum FayeError {
    ConnectionError(String),
    WebsocketError(tungstenite::Error),
    CloseError,
    WriteError,
    ReadError,
    InvalidResponse,
    DeserializeError,
    HandshakeError(String),
    ConnectError(String),
    SubscribeError(String),
    UnsubscribeError(String),
}

type FayeResult<T> = Result<T, FayeError>;

pub struct FayeClient<H>
where
    H: MessageHandler,
{
    state: SessionState,
    handler: H,
    extensions: Option<HashMap<String, serde_json::Value>>,
}

impl<H> FayeClient<H>
where
    H: MessageHandler,
{
    pub fn new(
        handler: H,
        extensions: Option<HashMap<String, serde_json::Value>>,
    ) -> FayeClient<H> {
        FayeClient {
            state: SessionState::new(),
            handler: handler,
            extensions: extensions,
        }
    }

    /// Connect to Bayeux server
    pub fn connect(&mut self, url: Url) -> FayeResult<()> {
        if self.is_opened() {
            if let Some(ref mut socket) = self.state.socket {
                socket.close(None).map_err(|_| FayeError::CloseError)?;
            }
        }

        self.state.connecting = true;

        let (_socket, _) = tungstenite::connect(url).map_err(|e| FayeError::WebsocketError(e))?;

        self.state.connecting = false;
        self.state.connected = true;
        self.state.socket = Some(_socket);

        self.bayeux_handshake()?;
        self.bayeux_connect()
    }

    /// Disconnect from Bayeux server
    pub fn disconnect(&mut self) -> FayeResult<()> {
        if !self.is_connected() {
            return Ok(());
        }

        self.bayeux_disconnect()
    }

    /// Subscribe to channels
    pub fn subscribe(&mut self, subscriptions: &Vec<String>) -> FayeResult<()> {
        if !self.is_connected() {
            return Err(FayeError::ConnectionError(
                "Cannot subscribe, because connection was not established".into(),
            ));
        }

        self.bayeux_subscribe(subscriptions)
    }

    /// Unsubscribe from channels
    pub fn unsubscribe(&mut self, subscriptions: &Vec<String>) -> FayeResult<()> {
        if !self.is_connected() {
            return Err(FayeError::ConnectionError(
                "Cannot unsubscribe, because connection was not established".into(),
            ));
        }

        self.bayeux_unsubscribe(subscriptions)
    }

    /// Returns true if connection was established, or false
    pub fn is_connected(&self) -> bool {
        self.state.connected && self.state.handshake
    }

    /// Returns true on establishing the connection or connection already established,
    /// other way returns false
    pub fn is_opened(&self) -> bool {
        self.state.socket.is_some() && (self.state.connected || self.state.connecting)
    }

    /// Make a handshake with Bayeux server
    fn bayeux_handshake(&mut self) -> FayeResult<()> {
        let connection_types = vec![ConnectionType::Websocket];
        let handshake_msg = MessageBuilder::create_handshake(connection_types, &self.extensions);
        let msg = self.send_message(handshake_msg)?;

        if !msg.successful.unwrap_or(false) && !Message::is_handshake(&msg) {
            return Err(FayeError::HandshakeError(msg.get_error()));
        }

        // extract the client_id
        self.state.client_id = Some(
            msg.client_id
                .as_ref()
                .ok_or_else(|| FayeError::HandshakeError("Server didn't send Client ID".into()))?
                .clone(),
        );
        self.state.handshake = true;

        // TODO: Handle server Advice
        println!("Advice: {:?}", msg.advice);

        self.handler.process_message(&msg);

        Ok(())
    }

    /// Send a CONNECT message to Bayeux server
    fn bayeux_connect(&mut self) -> FayeResult<()> {
        let client_id: String = self.state.client_id.as_ref().unwrap().clone();
        let connect_msg =
            MessageBuilder::create_connect(client_id, ConnectionType::Websocket, &self.extensions);
        let msg = self.send_message(connect_msg)?;

        if !msg.successful.unwrap_or(false) && !Message::is_connect(&msg) {
            return Err(FayeError::ConnectError(msg.get_error()));
        }

        self.state.connecting = false;
        self.state.connected = true;

        self.handler.process_message(&msg);

        Ok(())
    }

    /// Send a DISCONNECT message to Bayeux server (but no wait for response)
    fn bayeux_disconnect(&mut self) -> FayeResult<()> {
        let client_id: String = self.state.client_id.as_ref().unwrap().clone();
        let disconnect_msg = MessageBuilder::create_disconnect(client_id, &self.extensions);

        if let Some(ref mut socket) = self.state.socket {
            // send message
            socket
                .write_message(WsMsg::Text(disconnect_msg.to_json()))
                .map_err(|_| FayeError::WriteError)?;
            socket.close(None).map_err(|_| FayeError::CloseError)?;
        }

        self.state.connected = false;
        self.state.handshake = false;
        self.state.socket = None;

        Ok(())
    }

    /// Send a SUBSCIBE message to Bayeux server with given subscriptions
    fn bayeux_subscribe(&mut self, subscriptions: &Vec<String>) -> FayeResult<()> {
        let client_id: String = self.state.client_id.as_ref().unwrap().clone();
        let subscribe_msg =
            MessageBuilder::create_subscribe(client_id, subscriptions, &self.extensions);
        let msg = self.send_message(subscribe_msg)?;

        if !msg.successful.unwrap_or(false) && !Message::is_subscribe(&msg) {
            return Err(FayeError::SubscribeError(msg.get_error()));
        }

        self.handler.process_message(&msg);

        Ok(())
    }

    /// Send a UNSUBSCIBE message to Bayeux server with given subscriptions
    fn bayeux_unsubscribe(&mut self, subscriptions: &Vec<String>) -> FayeResult<()> {
        let client_id: String = self.state.client_id.as_ref().unwrap().clone();
        let unsubscribe_msg =
            MessageBuilder::create_subscribe(client_id, subscriptions, &self.extensions);
        let msg = self.send_message(unsubscribe_msg)?;

        if !msg.successful.unwrap_or(false) && !Message::is_unsubscribe(&msg) {
            return Err(FayeError::UnsubscribeError(msg.get_error()));
        }

        self.handler.process_message(&msg);

        Ok(())
    }

    /// Send the message and read response
    fn send_message(&mut self, message: Message) -> FayeResult<Message> {
        if let Some(ref mut socket) = self.state.socket {
            // send message
            socket
                .write_message(WsMsg::Text(message.to_json()))
                .map_err(|_| FayeError::WriteError)?;

            // read the message response
            let response_text = socket
                .read_message()
                .and_then(|msg| msg.into_text())
                .map_err(|_| FayeError::InvalidResponse)?;
            let m: Message =
                serde_json::from_str(&response_text).map_err(|_| FayeError::DeserializeError)?;

            return Ok(m);
        }

        Err(FayeError::ConnectionError(
            "Cannot send a message, because connection was not established".into(),
        ))
    }
}

#[derive(Default)]
pub struct SessionState {
    pub socket: Option<WebSocket<AutoStream>>,
    pub connected: bool,
    pub connecting: bool,
    pub handshake: bool,
    pub client_id: Option<String>,
}

impl SessionState {
    pub fn new() -> SessionState {
        Default::default()
    }
}
