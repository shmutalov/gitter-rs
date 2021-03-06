use std::borrow::Cow;
use std::collections::HashMap;
use reqwest::{Client, IntoUrl};
use reqwest::header::{Accept, Authorization, Bearer, ContentType, Headers};
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

const API_BASE_URL: &str = "https://api.gitter.im/v1";

#[derive(Debug)]
pub enum ApiError {
    EmptyResponse,
    BadResponse(String),
    BadRequest(String),
    RoomNotFound,
    UserNotFound,
    Unknown(String),
}

type ApiResult<T> = Result<T, ApiError>;

impl<'a> Gitter<'a> {
    /// New initializes the Gitter API client
    pub fn new<S>(token: S) -> ApiResult<Gitter<'a>>
    where
        S: Into<Cow<'a, str>>,
    {
        match Client::builder().timeout(Duration::from_secs(40)).build() {
            Ok(client) => Ok(Gitter {
                token: token.into(),
                api_base_url: API_BASE_URL.into(),
                client,
            }),
            Err(e) => Err(ApiError::Unknown(e.to_string())),
        }
    }

    /// Returns the current user
    pub fn get_user(&self) -> ApiResult<User> {
        let full_url = format!("{}/user", self.api_base_url);
        match self.get::<&str, Vec<User>>(&full_url) {
            Ok(users) => {
                if !users.is_empty() {
                    Ok(users[0].clone())
                } else {
                    Err(ApiError::UserNotFound)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Returns a list of Rooms the user is part of
    pub fn get_user_rooms<S>(&self, user_id: S) -> ApiResult<Vec<Room>>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/rooms", self.api_base_url, user_id.as_ref());
        self.get(&full_url)
    }

    /// Receive user's unread items and mentions in the room
    pub fn get_unread_items<U, R>(&self, user_id: U, room_id: R) -> ApiResult<UnreadItems>
    where
        U: AsRef<str>,
        R: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/rooms/{}/unreadItems", self.api_base_url, user_id.as_ref(), room_id.as_ref());
        self.get(&full_url)
    }

    /// Mark user's given room messages as read
    pub fn mark_messages_as_read<U, R>(
        &self,
        user_id: U,
        room_id: R,
        message_ids: &[String],
    ) -> ApiResult<()>
    where
        U: AsRef<str>,
        R: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/rooms/{}/unreadItems", self.api_base_url, user_id.as_ref(), room_id.as_ref());
        let unread_items = UnreadItems::from_msg_ids(message_ids);
        self.post(&full_url, &unread_items)
    }

    /// Returns a list of rooms the current user is in
    pub fn get_rooms(&self) -> ApiResult<Vec<Room>> {
        let full_url = format!("{}/rooms", self.api_base_url);
        self.get(&full_url)
    }

    /// List of the user's GitHub Organizations and their respective Room if available.
    pub fn get_user_organizations<S>(&self, user_id: S) -> ApiResult<Vec<Organization>>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/orgs", self.api_base_url, user_id.as_ref());
        self.get(&full_url)
    }

    /// List of the user's GitHub Repositories and their respective Room if available.
    pub fn get_user_repositories<S>(&self, user_id: S) -> ApiResult<Vec<Repository>>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/repos", self.api_base_url, user_id.as_ref());
        self.get(&full_url)
    }

    /// List of Gitter channels nested under the current user.
    pub fn get_user_channels<S>(&self, user_id: S) -> ApiResult<Vec<Channel>>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/channels", self.api_base_url, user_id.as_ref());
        self.get(&full_url)
    }

    /// Returns the users in the room with the passed id
    pub fn get_users_in_room<S>(&self, room_id: S) -> ApiResult<Vec<User>>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}/users", self.api_base_url, room_id.as_ref());
        self.get(&full_url)
    }

    /// Returns a room with the passed id
    pub fn get_room<S>(&self, room_id: S) -> ApiResult<Room>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}", self.api_base_url, room_id.as_ref());
        self.get(&full_url)
    }

    /// Returns a list of messages in a room.
    /// Pagination is optional. You can pass nil or specific pagination params.
    pub fn get_messages<S>(&self, room_id: S, params: Option<Pagination>) -> ApiResult<Vec<Message>>
    where
        S: AsRef<str>,
    {
        let mut full_url = format!("{}/rooms/{}/chatMessages", self.api_base_url, room_id.as_ref());

        if let Some(p) = params {
            full_url.push_str("?");
            full_url.push_str(&p.encode());
        }

        self.get(&full_url)
    }

    /// Returns a message in a room.
    pub fn get_message<R, M>(&self, room_id: R, message_id: M) -> ApiResult<Message>
    where
        R: AsRef<str>,
        M: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}/chatMessages/{}", self.api_base_url, room_id.as_ref(), message_id.as_ref());
        self.get(&full_url)
    }

    /// Sends a message to a room
    pub fn send_message<R, T>(&self, room_id: R, text: T) -> ApiResult<Message>
    where
        R: AsRef<str>,
        T: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}/chatMessages", self.api_base_url, room_id.as_ref());
        let msg = OutMessage {
            text: text.as_ref()
        };

        self.post(&full_url, &msg)
    }

    /// Update a message
    pub fn update_message<R, M, T>(&self, room_id: R, msg_id: M, text: T) -> ApiResult<()>
    where
        R: AsRef<str>,
        M: AsRef<str>,
        T: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}/chatMessages/{}", self.api_base_url, room_id.as_ref(), msg_id.as_ref());
        let msg = OutMessage {
            text: text.as_ref()
        };

        self.put(&full_url, &msg)
    }

    /// Joins a room
    pub fn join_room<R, U>(&self, room_id: R, user_id: U) -> ApiResult<Room>
    where
        R: AsRef<str>,
        U: AsRef<str>,
    {
        let full_url = format!("{}/user/{}/rooms", self.api_base_url, user_id.as_ref());
        let room = JoinRoom::from_id(room_id);

        self.post(&full_url, &room)
    }

    /// Join a room (uri method)
    pub fn join_room_by_uri<S>(&self, uri: S) -> ApiResult<Room>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/rooms", self.api_base_url);
        let room = JoinRoom::from_uri(uri);

        self.post(&full_url, &room)
    }

    /// Update a room topic
    pub fn update_room_topic<R, T>(&self, room_id: R, topic: T) -> ApiResult<Room>
    where
        R: AsRef<str>,
        T: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}", self.api_base_url, room_id.as_ref());
        let room = UpdateRoom::from_topic(topic);

        self.post(&full_url, &room)
    }

    /// Update a room noindex (indexing in search engines)
    pub fn update_room_noindex<S>(&self, room_id: S, noindex: bool) -> ApiResult<Room>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}", self.api_base_url, room_id.as_ref());
        let room = UpdateRoom::from_noindex(noindex);

        self.post(&full_url, &room)
    }

    /// Update a room topic
    pub fn update_room_tags<R, T>(&self, room_id: R, tags: T) -> ApiResult<Room>
    where
        R: AsRef<str>,
        T: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}", self.api_base_url, room_id.as_ref());
        let room = UpdateRoom::from_tags(tags);

        self.post(&full_url, &room)
    }

    /// Removes a user from the room
    pub fn leave_room<R, U>(&self, room_id: R, user_id: U) -> ApiResult<()>
    where
        R: AsRef<str>,
        U: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}/users/{}", self.api_base_url, room_id.as_ref(), user_id.as_ref());

        self.delete(&full_url)
    }

    /// Delete a room
    pub fn delete_room<S>(&self, room_id: S) -> ApiResult<()>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/rooms/{}", self.api_base_url, room_id.as_ref());

        self.delete(&full_url)
    }

    /// Queries the Rooms resources of gitter API
    pub fn search_rooms<S>(&self, room: S) -> ApiResult<SearchResult>
    where
        S: AsRef<str>,
    {
        let query = &[("q", room.as_ref())];
        let full_url = format!("{}/rooms?{}", self.api_base_url, &serde_urlencoded::to_string(query).unwrap());

        self.get(&full_url)
    }

    /// Returns the room ID of a given URI
    pub fn get_room_id<S>(&self, uri: S) -> ApiResult<String>
    where
        S: AsRef<str>,
    {
        let uri = uri.as_ref();
        self.search_rooms(uri.to_string())?
            .rooms
            .iter()
            .find(|r| r.uri.as_ref().map_or(false, |u| u == uri))
            .map(|r| r.id.to_string())
            .ok_or(ApiError::RoomNotFound)
    }

    /// Returns a list of groups the current user is in
    pub fn get_groups(&self) -> ApiResult<Vec<Group>> {
        let full_url = format!("{}/groups", self.api_base_url);

        self.get(&full_url)
    }

    /// List of rooms nested under the specified group.
    pub fn get_group_rooms<S>(&self, group_id: S) -> ApiResult<Vec<Room>>
    where
        S: AsRef<str>,
    {
        let full_url = format!("{}/groups/{}/rooms", self.api_base_url, group_id.as_ref());

        self.get(&full_url)
    }

    /// create default headers
    fn default_headers(&self) -> Headers {
        let mut headers = Headers::new();

        headers.set(ContentType::json());
        headers.set(Accept::json());
        headers.set(Authorization(Bearer {
            token: self.token.to_string(),
        }));

        headers
    }

    /// Returns raw data in bytes from specified url
    fn get<S, T>(&self, url: S) -> ApiResult<T>
    where
        S: IntoUrl,
        for<'de> T: Deserialize<'de>,
    {
        match self.client.get(url).headers(self.default_headers()).send() {
            Ok(mut response) => response
                .json::<T>()
                .map_err(|e| ApiError::BadResponse(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }

    /// Posts raw body data to specified url and returns response raw data
    fn post<S, B, T>(&self, url: S, body: B) -> ApiResult<T>
    where
        S: IntoUrl,
        B: Serialize,
        for<'de> T: Deserialize<'de>,
    {
        match self.client
            .post(url)
            .headers(self.default_headers())
            .json(&body)
            .send()
        {
            Ok(mut response) => response
                .json::<T>()
                .map_err(|e| ApiError::BadResponse(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }

    /// Puts raw body data to specified url and returns response raw data
    fn put<S, B, T>(&self, url: S, body: B) -> ApiResult<T>
    where
        S: IntoUrl,
        B: Serialize,
        for<'de> T: Deserialize<'de>,
    {
        match self.client
            .put(url)
            .headers(self.default_headers())
            .json(&body)
            .send()
        {
            Ok(mut response) => response
                .json::<T>()
                .map_err(|e| ApiError::BadResponse(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }

    /// Deletes resource by specified url
    fn delete<S, T>(&self, url: S) -> ApiResult<T>
    where
        S: IntoUrl,
        for<'de> T: Deserialize<'de>,
    {
        match self.client
            .delete(url)
            .headers(self.default_headers())
            .send()
        {
            Ok(mut response) => response
                .json::<T>()
                .map_err(|e| ApiError::BadResponse(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }
}

/// Pagination params
pub struct Pagination<'a> {
    /// Skip n messages
    pub skip: i32,

    /// Get messages before beforeId
    pub before_id: Option<Cow<'a, str>>,

    /// Get messages after afterId
    pub after_id: Option<Cow<'a, str>>,

    /// Maximum number of messages to return
    pub limit: i32,

    /// Search query
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
