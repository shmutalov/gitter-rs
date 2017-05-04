use std::borrow::Cow;
use hyper::client::Client; 
use hyper_tls::HttpsConnector;

use models::*;

#[derive(Debug)]
struct Gitter<'a> {
    token: Cow<'a, str>,
    api_base_url: Cow<'a, str>,
    client: Client<HttpsConnector>,
}

const API_BASE_URL: &str = "https://api.gitter.im/v1/";

impl<'a> Gitter<'a> {
    // New initializes the Gitter API client
    pub fn new<S>(token: S, client: Client<HttpsConnector>) -> Gitter<'a> 
        where S: Into<Cow<'a, str>>
    {
        let gitter = Gitter{
            token: token.into(),
            api_base_url: API_BASE_URL.into(),
            client: client,
            };

        gitter
    }

    // Returns the current user
    pub fn get_user(&self) -> Result<User, ()> {
        unimplemented!()
    }

    
    // Returns a list of Rooms the user is part of
    pub fn get_user_rooms<S>(&self, user_id: S) -> Result<Vec<Room>, ()>
        where S: Into<String> 
    {
        unimplemented!()
    }

    // Returns a list of rooms the current user is in
    pub fn get_rooms(&self) -> Result<Vec<Room>, ()> {
        unimplemented!()
    }

    // Returns the users in the room with the passed id
    pub fn get_users_in_room<S>(&self, room_id: S) -> Result<Vec<User>, ()> 
        where S: Into<String>
    {
        unimplemented!()
    }

    // Returns a room with the passed id
    pub fn get_room<S>(&self, room_id: S) -> Result<Room, ()> 
        where S: Into<String>
    {
        unimplemented!()
    }

    // Returns a list of messages in a room.
    // Pagination is optional. You can pass nil or specific pagination params.
    pub fn get_messages<S>(&self, room_id: S, params: Option<Pagination>) -> Result<Vec<Message>, ()>
        where S: Into<String>
    {
        unimplemented!()
    }

    // Returns a message in a room.
    pub fn get_message<S>(&self, room_id: S, message_id: S) -> Result<Message, ()> 
        where S: Into<String>
    {
        unimplemented!()
    }

    // Sends a message to a room
    pub fn send_message<S>(&self, room_id: S, text: S) -> Result<(), ()>
        where S: Into<String>
    {
        unimplemented!()
    }

    // Joins a room
    pub fn join_room<S>(&self, room_id: S, user_id: S) -> Result<Room, ()> 
        where S: Into<String>
    {
        unimplemented!()
    }

    // Removes a user from the room
    pub fn leave_room<S>(&self, room_id: S, user_id: S) -> Result<(), ()>
        where S: Into<String>
    {
        unimplemented!()
    }

    // Queries the Rooms resources of gitter API
    pub fn search_rooms<S>(&self, room: S) -> Result<Vec<Room>, ()> 
        where S: Into<String>
    {
        unimplemented!()
    }

    // Returns the room ID of a given URI
    pub fn get_room_id<S>(&self, uri: S) -> Result<String, ()>
        where S: Into<String>
    {
        unimplemented!()
    }


}

// Pagination params
pub struct Pagination<'a> {
    // Skip n messages
    skip: i32,

    // Get messages before beforeId
    before_id: Cow<'a, str>,

    // Get messages after afterId
    after_id: Cow<'a, str>,

    // Maximum number of messages to return
    limit: i32,

    // Search query
    query: Cow<'a, str>,
}