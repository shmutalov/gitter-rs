use std::borrow::Cow;
use std::collections::HashMap;
use reqwest::{Client, IntoUrl};
use reqwest::header::{Headers, Accept, ContentType, Authorization, Bearer};
use serde::{Deserialize, Serialize};
use serde_urlencoded;
use std::time::Duration;

use models::*;

#[derive(Debug)]
pub struct Gitter<'a> {
    token: Cow<'a, str>,
    api_base_url: Cow<'a, str>,
    client: Client,
}

const API_BASE_URL: &str = "https://api.gitter.im/v1/";

#[derive(Debug)]
pub enum ApiError {
    EmptyResponse,
    BadResponse,
    BadRequest(String),
    RoomNotFound,
    UserNotFound,
    Unknown(String),
}

type ApiResult<T> = Result<T, ApiError>;

impl<'a> Gitter<'a> {
    // New initializes the Gitter API client
    pub fn new<S>(token: S) -> Gitter<'a>
        where S: Into<Cow<'a, str>>
    {
        let mut client = Client::new().unwrap();
        client.timeout(Duration::from_secs(40));

        let gitter = Gitter {
            token: token.into(),
            api_base_url: API_BASE_URL.into(),
            client: client,
        };

        gitter
    }

    // Returns the current user
    pub fn get_user(&self) -> ApiResult<User> {
        let full_url = self.api_base_url.to_string() + "user";
        match self.get::<&str, Vec<User>>(&full_url) {
            Ok(users) => {
                if users.len() > 0 {
                    return Ok(users[0].clone());
                } else {
                    return Err(ApiError::UserNotFound);
                }
            }
            Err(e) => Err(e),
        }
    }

    // Returns a list of Rooms the user is part of
    pub fn get_user_rooms<S>(&self, user_id: S) -> ApiResult<Vec<Room>>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "user/" + &user_id.into() + "/rooms";
        self.get(&full_url)
    }

    // Returns a list of rooms the current user is in
    pub fn get_rooms(&self) -> ApiResult<Vec<Room>> {
        let full_url = self.api_base_url.to_string() + "rooms";
        self.get(&full_url)
    }

    // Returns the users in the room with the passed id
    pub fn get_users_in_room<S>(&self, room_id: S) -> ApiResult<Vec<User>>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into() + "/users";
        self.get(&full_url)
    }

    // Returns a room with the passed id
    pub fn get_room<S>(&self, room_id: S) -> ApiResult<Room>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into();
        self.get(&full_url)
    }

    // Returns a list of messages in a room.
    // Pagination is optional. You can pass nil or specific pagination params.
    pub fn get_messages<S>(&self, room_id: S, params: Option<Pagination>) -> ApiResult<Vec<Message>>
        where S: Into<String>
    {
        let mut full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into() +
                           "/chatMessages";

        if let Some(p) = params {
            full_url.push_str("?");
            full_url.push_str(&p.encode());
        }

        self.get(&full_url)
    }

    // Returns a message in a room.
    pub fn get_message<S>(&self, room_id: S, message_id: S) -> ApiResult<Message>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into() +
                       "/chatMessages/" + &message_id.into();

        self.get(&full_url)
    }

    // Sends a message to a room
    pub fn send_message<S>(&self, room_id: S, text: S) -> ApiResult<()>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into() + "/chatMessages";
        let msg = OutMessage { text: text.into() };

        self.post(&full_url, &msg)
    }

    // Joins a room
    pub fn join_room<S>(&self, room_id: S, user_id: S) -> ApiResult<Room>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "user/" + &user_id.into() + "/rooms";
        let room = JoinRoom::from_id(room_id);

        self.post(&full_url, &room)
    }

    /// Join a room (uri method)
    pub fn join_room_by_uri<S>(&self, uri: S) -> ApiResult<Room>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "/rooms";
        let room = JoinRoom::from_uri(uri.into());

        self.post(&full_url, &room)
    }

    /// Update a room topic
    pub fn update_room_topic<S>(&self, room_id: S, topic: S) -> ApiResult<Room>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "/rooms" + &room_id.into();
        let room = UpdateRoom::from_topic(topic.into());

        self.post(&full_url, &room)
    }

    /// Update a room noindex (indexing in search engines)
    pub fn update_room_noindex<S>(&self, room_id: S, noindex: bool) -> ApiResult<Room>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "/rooms" + &room_id.into();
        let room = UpdateRoom::from_noindex(noindex.into());

        self.post(&full_url, &room)
    }

    /// Update a room topic
    pub fn update_room_tags<S>(&self, room_id: S, tags: S) -> ApiResult<Room>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "/rooms" + &room_id.into();
        let room = UpdateRoom::from_tags(tags.into());

        self.post(&full_url, &room)
    }

    // Removes a user from the room
    pub fn leave_room<S>(&self, room_id: S, user_id: S) -> ApiResult<()>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into() + "/users/" +
                       &user_id.into();

        self.delete(&full_url)
    }

    // Delete a room
    pub fn delete_room<S>(&self, room_id: S) -> ApiResult<()>
        where S: Into<String>
    {
        let full_url = self.api_base_url.to_string() + "rooms/" + &room_id.into();

        self.delete(&full_url)
    }

    // Queries the Rooms resources of gitter API
    pub fn search_rooms<S>(&self, room: S) -> ApiResult<SearchResult>
        where S: Into<String>
    {
        let query = &[("q", &room.into())];
        let full_url = self.api_base_url.to_string() + "rooms?" +
                       &serde_urlencoded::to_string(query).unwrap();

        self.get(&full_url)
    }

    // Returns the room ID of a given URI
    pub fn get_room_id<S>(&self, uri: S) -> ApiResult<String>
        where S: AsRef<str>
    {
        let uri = uri.as_ref();
        self.search_rooms(uri.to_string())?
            .rooms.iter()
            .find(|r| r.uri.as_ref().map_or(false, |u| u == uri))
            .map(|r| r.id.to_string())
            .ok_or(ApiError::RoomNotFound)
    }

    // create default headers
    fn default_headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(ContentType::json());
        headers.set(Accept::json());
        headers.set(Authorization(Bearer { token: self.token.to_string() }));

        headers
    }

    // Returns raw data in bytes from specified url
    fn get<S, T>(&self, url: S) -> ApiResult<T>
        where S: IntoUrl,
              T: Deserialize
    {
        match self.client.get(url).headers(self.default_headers()).send() {
            Ok(mut response) => response.json::<T>().map_err(|e| ApiError::Unknown(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }

    // Sends raw body data to specified url and returns response raw data
    fn post<S, B, T>(&self, url: S, body: B) -> ApiResult<T>
        where S: IntoUrl,
              B: Serialize,
              T: Deserialize
    {
        match self.client.post(url).json(&body).send() {
            Ok(mut response) => response.json::<T>().map_err(|e| ApiError::Unknown(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }

    // Deletes resource by specified url
    fn delete<S, T>(&self, url: S) -> ApiResult<T>
        where S: IntoUrl,
              T: Deserialize
    {
        match self.client.delete(url).send() {
            Ok(mut response) => response.json::<T>().map_err(|e| ApiError::Unknown(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }
}

// Pagination params
pub struct Pagination<'a> {
    // Skip n messages
    pub skip: i32,

    // Get messages before beforeId
    pub before_id: Option<Cow<'a, str>>,

    // Get messages after afterId
    pub after_id: Option<Cow<'a, str>>,

    // Maximum number of messages to return
    pub limit: i32,

    // Search query
    #[allow(dead_code)]
    pub query: Option<Cow<'a, str>>,
}

impl<'a> Pagination<'a> {
    pub fn encode(self) -> String {
        let mut values = HashMap::new();

        if let Some(after_id) = self.after_id {
            values.insert("afterId", after_id.to_string());
        }

        if let Some(before_id) = self.before_id {
            values.insert("beforeId", before_id.to_string());
        }

        if self.skip > 0 {
            values.insert("skip", self.skip.to_string());
        }

        if self.limit > 0 {
            values.insert("limit", self.limit.to_string());
        }

        serde_urlencoded::to_string(&values).unwrap()
    }
}
