use bayeux::message::*;
use bayeux::VERSION;
use serde_json::Value;
use std::collections::HashMap;

/// Bayeux messages builder
pub struct MessageBuilder;

impl MessageBuilder {
    /// Create HANDSHAKE message
    pub fn create_handshake(
        connection_types: Vec<ConnectionType>,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: "/meta/handshake".into(),
            version: Some(VERSION.into()),
            supported_connection_types: Some(connection_types),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }

    /// Create CONNECT message
    pub fn create_connect(
        client_id: String,
        connection_type: ConnectionType,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: "/meta/connect".into(),
            client_id: Some(client_id),
            connection_type: Some(connection_type),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }

    /// Create DISCONNECT message
    pub fn create_disconnect(
        client_id: String,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: "/meta/disconnect".into(),
            client_id: Some(client_id),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }

    /// Create SUBSCRIBE message
    pub fn create_subscribe(
        client_id: String,
        subscription: &Vec<String>,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: "/meta/subscribe".into(),
            client_id: Some(client_id),
            subscriptions: Some(subscription.clone()),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }

    /// Create UNSUBSCRIBE message
    pub fn create_unsubscribe(
        client_id: String,
        subscription: &Vec<String>,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: "/meta/unsubscribe".into(),
            client_id: Some(client_id),
            subscriptions: Some(subscription.clone()),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }

    /// Create PUBLISH message
    pub fn create_publish(
        client_id: String,
        channel: String,
        data: Value,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: channel,
            client_id: Some(client_id),
            data: Some(data),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }

    /// Create DELIVERY message
    pub fn create_delivery(
        channel: String,
        data: Value,
        extensions: &Option<HashMap<String, Value>>,
    ) -> Message {
        Message {
            channel: channel,
            data: Some(data),
            extensions: extensions.clone(),
            ..Default::default()
        }
    }
}