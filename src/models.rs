use chrono::{DateTime, UTC};

// A Room in Gitter can represent a GitHub Organization, a GitHub Repository,
// a Gitter Channel or a One-to-one conversation.
// In the case of the Organizations and Repositories,
// the access control policies are inherited from GitHub.
#[derive(Deserialize, Debug)]
pub struct Room {
    // Room ID
    pub id: String,

    // Room name
    pub name: String,

    // Room topic. (default: GitHub repo description)
    pub topic: String,

    // Room URI on Gitte
    pub uri: String,

    // Indicates if the room is a one-to-one chat
    #[serde(rename = "oneToOne")]
    pub one_to_one: bool,

    // Count of users in the room
    pub user_count: i32,

    // Number of unread messages for the current user
    #[serde(rename = "unreadItems")]
    pub unread_items: i32,

    // Number of unread mentions for the current user
    pub mentions: i32,

    // Last time the current user accessed the room in ISO format
    #[serde(rename = "lastAccessTime")]
    pub last_access_time: DateTime<UTC>,

    // Indicates if the current user has disabled notifications
    pub lurk: bool,

    // Path to the room on gitter
    pub url: String,

    // Type of the room
    // - ORG: A room that represents a GitHub Organization.
    // - REPO: A room that represents a GitHub Repository.
    // - ONETOONE: A one-to-one chat.
    // - ORG_CHANNEL: A Gitter channel nested under a GitHub Organization.
    // - REPO_CHANNEL A Gitter channel nested under a GitHub Repository.
    // - USER_CHANNEL A Gitter channel nested under a GitHub User.
    #[serde(rename = "githubType")]
    pub github_type: String,

    // Tags that define the room
    pub tags: Vec<String>,

    // Determines, whether current user is room member or not
    #[serde(rename = "roomMember")]
    pub room_member: bool,

    // Room version
    #[serde(rename = "v")]
    pub version: i32,
}

// Join room request model
#[derive(Serialize, Debug)]
pub struct JoinRoom {
    // Room ID to join
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    // Gitter User ID
    pub id: String,

    // Gitter/GitHub username
    #[serde(rename = "username")]
    pub username: String,

    // Gitter/GitHub user real name
    #[serde(rename = "displayName")]
    pub display_name: String,

    // Path to the user on Gitter
    pub url: String,

    // User avatar URI (small)
    #[serde(rename = "avatarUrlSmall")]
    pub avatar_url_small: String,

    // User avatar URI (medium)
    #[serde(rename = "avatarUrlMedium")]
    pub avatar_url_medium: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    // ID of the message
    pub id: String,

    // Original message in plain-text/markdown
    pub text: String,

    // HTML formatted message
    pub html: String,

    // ISO formatted date of the message
    pub sent: DateTime<UTC>,

    // ISO formatted date of the message if edited
    #[serde(rename = "editedAt")]
    pub edited_at: DateTime<UTC>,

    // User that sent the message
    #[serde(rename = "fromUser")]
    pub from: User,

    // Boolean that indicates if the current user has read the message.
    pub unread: bool,

    // Number of users that have read the message
    #[serde(rename = "readBy")]
    pub read_by: i32,

    // List of URLs present in the message
    pub urls: Vec<Url>,

    // List of @Mentions in the message
    pub mentions: Vec<Mention>,

    // List of #Issues referenced in the message
    pub issues: Vec<Issue>,

    // Version
    #[serde(rename = "v")]
    pub version: i32,
}

// Send message request model
#[derive(Serialize, Debug)]
pub struct OutMessage {
    // Original message in plain-text/markdown
    pub text: String
}

// Mention holds data about mentioned user in the message
#[derive(Deserialize, Debug)]
pub struct Mention {
    // User's username
    #[serde(rename = "screenName")]
    pub screen_name: String,

    // Gitter User ID
    #[serde(rename = "userID")]
    pub user_id: String,
}

// Issue references issue in the message
#[derive(Deserialize, Debug)]
pub struct Issue {
    // Issue number
    pub number: String,
}

// URL presented in the message
#[derive(Deserialize, Debug)]
pub struct Url {
    // URL
    pub url: String,
}

// Search rooms result
#[derive(Deserialize, Debug)]
pub struct SearchResult {
    #[serde(rename = "results")]
    pub rooms: Vec<Room>
}
