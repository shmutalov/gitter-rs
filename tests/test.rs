extern crate serde;
extern crate serde_json;
extern crate gitter;

use gitter::*;

fn get_gitter_api<'a>() -> Gitter<'a> {
    let token = std::env::var("GITTER_BOT_TOKEN").unwrap();
    Gitter::new(token)
}

#[test]
fn deserialize_user() {
    let user_json_str = "[{
  \"id\": \"53307734c3599d1de448e192\",
  \"username\": \"malditogeek\",
  \"displayName\": \"Mauro Pompilio\",
  \"url\": \"/malditogeek\",
  \"avatarUrlSmall\": \"https://avatars.githubusercontent.com/u/14751?\",
  \"avatarUrlMedium\": \"https://avatars.githubusercontent.com/u/14751?\"
}]";

    let user = serde_json::from_str::<Vec<User>>(user_json_str).unwrap();

    assert_eq!("53307734c3599d1de448e192", user[0].id);
}

#[test]
fn serialize_out_message() {
    let out_msg = OutMessage{text: "test `message` from @shmutalov".to_string()};
    let out_msg_expected = "{\"text\":\"test `message` from @shmutalov\"}";
    let out_msg_json = serde_json::to_string(&out_msg).unwrap();

    assert_eq!(&out_msg_expected, &out_msg_json);
}

#[test]
fn api_init() {
    Gitter::new("asbcasadasd");
}

#[test]
fn api_get_user() {
    let api = get_gitter_api();
    let user = api.get_user();

    assert!(user.is_ok());
}

#[test]
fn api_get_user_rooms() {
    let api = get_gitter_api();

    let user = api.get_user().unwrap();
    let rooms = api.get_user_rooms(&user.id);

    assert!(rooms.is_ok());
}

#[test]
fn api_get_rooms() {
    let api = get_gitter_api();
    let user = api.get_user().unwrap();

    let rooms = api.get_rooms();

    assert!(rooms.is_ok());

    let user_rooms = api.get_user_rooms(&user.id);

    assert!(user_rooms.is_ok());

    assert_eq!(rooms.unwrap().len(), user_rooms.unwrap().len());
}

#[test]
fn api_get_users_in_room() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let users = api.get_users_in_room(&rooms[0].id);

    assert!(users.is_ok());
}

#[test]
fn api_get_room() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let room = api.get_room(&rooms[0].id);

    assert!(room.is_ok());
}

#[test]
fn api_get_messages_without_pagination() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let messages = api.get_messages(&rooms[0].id, None);

    assert!(messages.is_ok());
}

#[test]
fn api_get_messages_with_pagination() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let pagination = Pagination{
        skip: 1,
        limit: 50,
        after_id: None,
        before_id: None,
        query: None
        };
    let messages = api.get_messages(&rooms[0].id, Some(pagination));

    assert!(messages.is_ok());
}

#[test]
fn api_get_message() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let messages = api.get_messages(&rooms[0].id, None).unwrap();
    
    let message = api.get_message(&rooms[0].id, &messages[0].id);

    assert!(message.is_ok());
}

#[test]
fn api_get_room_id() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let room = rooms.into_iter().find(|x| x.uri.is_some()).unwrap();
    let room_id = api.get_room_id(room.uri.unwrap_or("".to_owned()));

    assert!(room_id.is_ok());
}

#[test]
fn api_search_rooms() {
    let api = get_gitter_api();

    let rooms = api.get_rooms().unwrap();
    let room = rooms.into_iter().find(|x| x.uri.is_some()).unwrap();
    let search_result = api.search_rooms(&room.name);

    assert!(search_result.is_ok());
    assert!(!search_result.unwrap().rooms.is_empty());
}

#[test]
fn api_get_groups() {
    let api = get_gitter_api();
    let groups = api.get_groups();

    assert!(groups.is_ok());
}

#[test]
fn api_get_group_rooms() {
    let api = get_gitter_api();
    let groups = api.get_groups().unwrap();
    let rooms = api.get_group_rooms(&groups[0].id);

    assert!(rooms.is_ok());
}

#[test]
fn api_get_unread_items() {
    let api = get_gitter_api();
    let user = api.get_user().unwrap();
    let user_rooms = api.get_user_rooms(&user.id).unwrap();

    let unread_items = api.get_unread_items(&user.id, &user_rooms[0].id);

    assert!(unread_items.is_ok());
}

#[test]
fn api_get_user_organizations() {
    let api = get_gitter_api();
    let user = api.get_user().unwrap();
    let user_orgs = api.get_user_organizations(&user.id);

    assert!(user_orgs.is_ok());
}

#[test]
fn api_get_user_repositories() {
    let api = get_gitter_api();
    let user = api.get_user().unwrap();
    let user_repos = api.get_user_repositories(&user.id);

    assert!(user_repos.is_ok());
}

#[test]
#[ignore]
fn api_get_user_channels() {
    let api = get_gitter_api();
    let user = api.get_user().unwrap();
    let user_channels = api.get_user_channels(&user.id);

    assert!(user_channels.is_ok());
}

#[test]
fn api_send_message() {
    let api = get_gitter_api();
    let room_id = api.get_room_id("gitter-rs/testing").unwrap();

    let msg = "@shmutalov this is a `test` message.\n\n```rust\nfn main() {}```";
    let result = api.send_message(&room_id, &msg).unwrap();
    assert_eq!(&result.text, &msg);
}