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

use tungstenite::client::AutoStream;
use tungstenite::{connect, WebSocket};
use url::Url;

#[derive(Debug)]
pub enum FayeError {
    ConnectionError,
    CloseError,
}

type FayeResult<T> = Result<T, FayeError>;

pub struct FayeClient<H>
where
    H: MessageHandler,
{
    state: Option<SessionState>,
    handler: Box<H>,
}

impl<H> FayeClient<H>
where
    H: MessageHandler,
{
    pub fn new(handler: H) -> FayeClient<H> {
        FayeClient {
            state: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect<U>(&mut self, url: Url) -> FayeResult<()> {
        if self.is_opened() {
            if let Some(ref mut state) = self.state {
                if let Some(ref mut socket) = state.socket {
                    socket.close(None).map_err(|_| FayeError::CloseError)?;
                }
            }
        }

        self.state = Some(SessionState {
            connecting: true,
            ..Default::default()
        });

        let (_socket, _) = connect(url).map_err(|_| FayeError::ConnectionError)?;
        if let Some(ref mut state) = self.state {
            state.connecting = false;
            state.connected = true;
            state.socket = Some(_socket);
        }

        self.handshake()?;
        self.handler.on_connected();

        Ok(())
    }

    /// Returns true if connection was established, or false
    pub fn is_connected(&self) -> bool {
        if let Some(ref state) = self.state {
            return state.connected && state.handshake;
        }

        false
    }

    /// Returns true on establishing the connection or connection already established,
    /// other way returns false
    pub fn is_opened(&self) -> bool {
        if let Some(ref state) = self.state {
            if let Some(ref _socket) = state.socket {
                return state.connected || state.connecting;
            }
        }

        false
    }

    fn handshake(&mut self) -> FayeResult<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct SessionState {
    pub socket: Option<WebSocket<AutoStream>>,
    pub connected: bool,
    pub connecting: bool,
    pub handshake: bool,
}

impl SessionState {
    pub fn new() -> SessionState {
        Default::default()
    }
}
