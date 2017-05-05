extern crate serde;
extern crate serde_json;
extern crate gitter;

use gitter::*;

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
fn api_get_rooms() {
    let api = get_gitter_api();
    let user = api.get_user().unwrap();

    let rooms = api.get_user_rooms(user.id).unwrap();

    // assert!(rooms.is_ok());
}

fn get_gitter_api<'a>() -> Gitter<'a> {
    let token = std::env::var("GITTER_BOT_TOKEN").unwrap();
    Gitter::new(token)
}