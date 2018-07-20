use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// Type of transport the client requires for communication.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionType {
    LongPolling,
    CallbackPolling,
    Iframe,
    Flash,
    Websocket,
}

/// The reconnect advice field is a string that indicates how
/// the client should act in the case of a failure to connect.
#[derive(Serialize, Deserialize, Debug)]
pub enum ReconnectType {
    /// a client MAY attempt to reconnect with a /meta/connect message after the interval
    /// (as defined by interval advice field or client-default backoff),
    /// and with the same credentials.
    Retry,

    /// the server has terminated any prior connection status and the client MUST reconnect
    /// with a /meta/handshake message. A client MUST NOT automatically retry
    /// when a reconnect advice handshake has been received.
    Handshake,

    /// indicates a hard failure for the connect attempt.
    /// A client MUST respect reconnect advice none and
    /// MUST NOT automatically retry or handshake.
    None,
}

/// The advice message field provides a way for servers to inform clients
/// of their preferred mode of client operation so that in conjunction with
/// server-enforced limits, Bayeux implementations can prevent resource
/// exhaustion and inelegant failure modes.
///
/// Furthermore, the advice message field provides a way for clients to inform
/// servers of their preferred mode of operation so that they can better inform
/// client-side applications of state changes (for example, connection state changes)
/// that are relevant for applications.
///
/// The advice field is a JSON encoded object containing general and transport
/// specific values that indicate modes of operation, timeouts and other potential
/// transport specific parameters. Advice fields may occur either in
/// the top level of an advice object or within a transport specific section of the advice object.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Advice {
    /// The reconnect advice field is a string that indicates how the
    /// client should act in the case of a failure to connect.
    pub reconnect: Option<ReconnectType>,

    /// An integer representing the period of time, in milliseconds,
    /// for the server to delay responses to the /meta/connect channel.
    ///
    /// This value is merely informative for clients.
    /// Bayeux servers SHOULD honor timeout advices sent by clients.
    pub timeout: Option<u32>,

    /// An integer representing the minimum period of time, in milliseconds,
    /// for a client to delay subsequent requests to the /meta/connect channel.
    /// A negative period indicates that the message should not be retried.
    ///
    /// A client MUST implement interval support, but a client MAY exceed
    /// the interval provided by the server. A client SHOULD implement a backoff
    /// strategy to increase the interval if requests to the server
    /// fail without new advice being received from the server.
    pub interval: Option<i32>,

    /// This is a boolean field, which if true indicates that the server
    /// has detected multiple Bayeux client instances running within the same web client.
    pub multiple_clients: Option<bool>,

    /// This is an array of strings field, which if present indicates
    /// a list of host names or IP addresses that MAY be used as
    /// alternate servers with which the client may connect.
    /// If a client receives advice to re-handshake and the current
    /// server is not included in a supplied hosts list,
    /// then the client SHOULD try the hosts in order until a successful
    /// connection is establish. Advice received during handshakes
    /// with hosts in the list supersedes any previously received advice.
    pub hosts: Option<Vec<String>>,

    // TODO: Add a specification?
    pub callback_polling: Option<TransportAdvice>,
    pub long_polling: Option<TransportAdvice>,
    pub websocket: Option<TransportAdvice>,
    pub flash: Option<TransportAdvice>,
    pub iframe: Option<TransportAdvice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransportAdvice {
    /// The reconnect advice field is a string that indicates how the
    /// client should act in the case of a failure to connect.
    pub reconnect: Option<ReconnectType>,

    /// An integer representing the period of time, in milliseconds,
    /// for the server to delay responses to the /meta/connect channel.
    ///
    /// This value is merely informative for clients.
    /// Bayeux servers SHOULD honor timeout advices sent by clients.
    pub timeout: Option<u32>,

    /// An integer representing the minimum period of time, in milliseconds,
    /// for a client to delay subsequent requests to the /meta/connect channel.
    /// A negative period indicates that the message should not be retried.
    ///
    /// A client MUST implement interval support, but a client MAY exceed
    /// the interval provided by the server. A client SHOULD implement a backoff
    /// strategy to increase the interval if requests to the server
    /// fail without new advice being received from the server.
    pub interval: Option<i32>,
}

/// Bayeux message
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// The channel message field MUST be included in every
    /// Bayeux message to specify the source or destination of the
    /// message. In a request, the channel specifies the destination of the message,
    /// and in a response it specifies the source of the message.
    pub channel: String,

    /// The version message field MUST be included in
    /// messages to/from the /meta/handshake channel to
    /// indicate the protocol version expected by the client/server.
    pub version: Option<String>,

    /// The minimumVersion message field MAY be included in
    /// messages to/from the /meta/handshake channel to indicate
    /// the oldest protocol version that can be handled by the client/server.
    pub minimum_version: Option<String>,

    /// The supportedConnectionTypes field is included in
    /// messages to/from the /meta/handshake channel to allow
    /// clients and servers to reveal the transports that are supported.
    /// The value is an array of strings,
    /// with each string representing a transport name. Defined connection types include:
    pub supported_connection_types: Option<Vec<ConnectionType>>,

    /// The clientId message field uniquely identifies a client to the Bayeux server.
    /// The clientId message field MUST be included in every message sent to
    /// the server except for messages sent to the /meta/handshake channel and
    /// MAY be omitted in a publish message. The clientId message field MAY be
    /// returned in message responses except for failed handshake requests and
    /// for publish message responses that were sent without clientId.
    /// However, care must be taken to not leak the clientId to other
    /// clients when broadcasting messages, because that would allow
    /// any other client to impersonate the client whose clientId was leaked.
    pub client_id: Option<String>,

    /// The advice message field provides a way for servers to inform clients
    /// of their preferred mode of client operation so that in conjunction with
    /// server-enforced limits, Bayeux implementations can prevent resource
    /// exhaustion and inelegant failure modes.
    ///
    /// Furthermore, the advice message field provides a way for clients to inform
    /// servers of their preferred mode of operation so that they can better inform
    /// client-side applications of state changes (for example, connection state changes)
    /// that are relevant for applications.
    ///
    /// The advice field is a JSON encoded object containing general and transport
    /// specific values that indicate modes of operation, timeouts and other potential
    /// transport specific parameters. Advice fields may occur either in
    /// the top level of an advice object or within a transport specific section of the advice object.
    pub advice: Option<Advice>,

    /// The connectionType message field specifies the type of transport
    /// the client requires for communication. The connectionType message
    /// field MUST be included in request messages to the /meta/connect channel.
    pub connection_type: Option<ConnectionType>,

    /// An id message field MAY be included in any Bayeux message with an alpha numeric value
    ///
    /// Generation of IDs is implementation specific and may be provided by the application.
    /// Messages published to /meta/** and /service/** SHOULD have id fields that are unique within the connection.
    ///
    /// Messages sent in response to messages delivered to /meta/**
    /// channels MUST use the same message id as the request message.
    ///
    /// Messages sent in response to messages delivered to /service/**
    /// channels SHOULD use the same message id as the request message
    /// or an id derived from the request message id.
    pub id: Option<String>,

    /// The timestamp message field SHOULD be specified in
    /// the following ISO 8601 profile (all times SHOULD be sent in GMT time)
    pub timestamp: Option<DateTime<Utc>>,

    /// The data message field is an arbitrary JSON encoded object that contains event information.
    /// The data message field MUST be included in publish messages,
    /// and a Bayeux server MUST include the data message field in an event delivery message.
    pub data: Option<Value>,

    /// The boolean successful message field is used to indicate success
    /// or failure and MUST be included in responses to
    /// the /meta/handshake, /meta/connect, /meta/subscribe,
    /// /meta/unsubscribe, /meta/disconnect, and publish channels.
    pub successful: Option<bool>,

    /// auth_successful field MAY be included to support prototype client
    /// implementations that required the authSuccessful field
    pub auth_successful: Option<bool>,

    /// The subscription message field specifies the channels
    /// the client wishes to subscribe to or unsubscribe from.
    /// The subscription message field MUST be included in requests
    /// and responses to/from the /meta/subscribe or /meta/unsubscribe channels.
    #[serde(rename = "subscription")]
    pub subscriptions: Option<Vec<String>>,

    /// The error message field is OPTIONAL in any Bayeux response.
    /// The error message field MAY indicate the type of error
    /// that occurred when a request returns with a false successful message.
    pub error: Option<String>,

    /// An ext message field MAY be included in any Bayeux message.
    /// Its value SHOULD be a JSON encoded object with top level names
    /// distinguished by implementation names (for example "com.acme.ext.auth").
    ///
    /// The contents of ext message field may be arbitrary values that
    /// allow extensions to be negotiated and implemented
    /// between server and client implementations.
    #[serde(rename = "ext")]
    pub extensions: Option<HashMap<String, Value>>,
}
