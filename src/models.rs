use chrono::{DateTime, UTC};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GithubType {
    /// A room that represents a GitHub Organisation.
    Org,
    /// A room that represents a GitHub Repository.
    Repo,
    /// A one-to-one chat.
    #[serde(rename = "ONETOONE")]
    OneToOne, 
    /// A Gitter channel nested under a GitHub Organisation.
    OrgChannel, 
    /// A Gitter channel nested under a GitHub Repository.
    RepoChannel, 
    /// A Gitter channel nested under a GitHub User.
    UserChannel, 
}

/// A Room in Gitter can represent a GitHub Organization, a GitHub Repository,
/// a Gitter Channel or a One-to-one conversation.
/// In the case of the Organizations and Repositories,
/// the access control policies are inherited from GitHub.
#[derive(Deserialize, Debug)]
pub struct Room {
    /// Room ID
    pub id: String,

    /// Room name
    pub name: String,

    /// Room topic. (default: GitHub repo description)
    pub topic: String,

    /// Room URI on Gitter
    pub uri: Option<String>,

    /// Indicates if the room is a one-to-one chat
    #[serde(rename = "oneToOne")]
    pub one_to_one: bool,

    /// Count of users in the room
    #[serde(rename = "userCount")]
    pub user_count: i32,

    /// Number of unread messages for the current user
    #[serde(rename = "unreadItems")]
    pub unread_items: i32,

    /// Number of unread mentions for the current user
    pub mentions: i32,

    /// Last time the current user accessed the room in ISO format
    #[serde(rename = "lastAccessTime")]
    pub last_access_time: Option<DateTime<UTC>>,

    /// Indicates if the current user has disabled notifications
    pub lurk: bool,

    /// Path to the room on gitter
    pub url: String,

    /// Type of the room
    /// - ORG: A room that represents a GitHub Organization.
    /// - REPO: A room that represents a GitHub Repository.
    /// - ONETOONE: A one-to-one chat.
    /// - ORG_CHANNEL: A Gitter channel nested under a GitHub Organization.
    /// - REPO_CHANNEL A Gitter channel nested under a GitHub Repository.
    /// - USER_CHANNEL A Gitter channel nested under a GitHub User.
    #[serde(rename = "githubType")]
    pub github_type: GithubType,

    /// Tags that define the room
    pub tags: Option<Vec<String>>,

    /// Determines, whether current user is room member or not
    #[serde(rename = "roomMember")]
    pub room_member: bool,

    /// Room version
    #[serde(rename = "v")]
    pub version: Option<i32>,
}

/// Join room request model
#[derive(Serialize, Debug)]
pub struct JoinRoom {
    /// Room ID to join
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl JoinRoom {
    /// Create join room request from room ID
    pub fn from_id<S>(id: S) -> JoinRoom 
        where S: AsRef<str>
    {
        JoinRoom {
            id: Some(id.as_ref().to_string()),
            uri: None,
        }
    }

    /// Create join room request from URI
    pub fn from_uri<S>(uri: S) -> JoinRoom 
        where S: AsRef<str> 
    {
        JoinRoom {
            id: None,
            uri: Some(uri.as_ref().to_string()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UpdateRoom {
    /// Room topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// Whether the room is indexed by search engines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noindex: Option<bool>,

    /// Tags that define the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>
}

impl UpdateRoom {
    /// Create update room request only with topic parameter
    pub fn from_topic<S>(topic: S) -> UpdateRoom 
        where S: AsRef<str>
    {
        UpdateRoom {
            topic: Some(topic.as_ref().to_string()),
            noindex: None,
            tags: None
        }
    }

    /// Create update room request only with noindex parameter
    pub fn from_noindex(noindex: bool) -> UpdateRoom {
        UpdateRoom {
            topic: None,
            noindex: Some(noindex),
            tags: None
        }
    }

    /// Create update room request only with tags parameter
    pub fn from_tags<S>(tags: S) -> UpdateRoom 
        where S: AsRef<str>
    {
        UpdateRoom {
            topic: None,
            noindex: None,
            tags: Some(tags.as_ref().to_string()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    /// Gitter User ID
    pub id: String,

    /// Gitter/GitHub username
    #[serde(rename = "username")]
    pub username: String,

    /// Gitter/GitHub user real name
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// Path to the user on Gitter
    pub url: String,

    /// User avatar URI (small)
    #[serde(rename = "avatarUrlSmall")]
    pub avatar_url_small: String,

    /// User avatar URI (medium)
    #[serde(rename = "avatarUrlMedium")]
    pub avatar_url_medium: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    /// ID of the message
    pub id: String,

    /// Original message in plain-text/markdown
    pub text: String,

    /// HTML formatted message
    pub html: String,

    /// ISO formatted date of the message
    pub sent: DateTime<UTC>,

    /// ISO formatted date of the message if edited
    #[serde(rename = "editedAt")]
    pub edited_at: Option<DateTime<UTC>>,

    /// User that sent the message
    #[serde(rename = "fromUser")]
    pub from: User,

    /// Boolean that indicates if the current user has read the message.
    pub unread: bool,

    /// Number of users that have read the message
    #[serde(rename = "readBy")]
    pub read_by: i32,

    /// List of URLs present in the message
    pub urls: Vec<Url>,

    /// List of @Mentions in the message
    pub mentions: Vec<Mention>,

    /// List of #Issues referenced in the message
    pub issues: Vec<Issue>,

    /// Version
    #[serde(rename = "v")]
    pub version: i32,
}

/// Send message request model
#[derive(Serialize, Debug)]
pub struct OutMessage {
    /// Original message in plain-text/markdown
    pub text: String,
}

/// Mention holds data about mentioned user in the message
#[derive(Deserialize, Debug)]
pub struct Mention {
    /// User's username
    #[serde(rename = "screenName")]
    pub screen_name: String,

    /// Gitter User ID
    #[serde(rename = "userID")]
    pub user_id: String,
}

/// Issue references issue in the message
#[derive(Deserialize, Debug)]
pub struct Issue {
    /// Issue number
    pub number: String,
}

/// URL presented in the message
#[derive(Deserialize, Debug)]
pub struct Url {
    /// URL
    pub url: String,
}

/// Search rooms result
#[derive(Deserialize, Debug)]
pub struct SearchResult {
    #[serde(rename = "results")]
    pub rooms: Vec<Room>,
}

#[derive(Deserialize, Debug)]
pub struct Group {
    /// Group ID
    pub id: String,

    /// Group name.
    pub name: String,

    /// Group URI on Gitter.
    pub uri: String,

    /// Security descriptor. Describes the backing object we get permissions from.
    #[serde(rename = "backedBy")]
    pub backed_by: BackedBy,

    /// Base avatar URL (add s parameter to size)
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

/// Security descriptor. Describes the backing object we get permissions from.
#[derive(Deserialize, Debug)]
pub struct BackedBy {
    #[serde(rename = "type")]
    pub group_type: Option<GroupType>,

    /// Represents how we find the backing object given the type
    #[serde(rename = "linkPath")]
    pub link_path: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GroupType {
    OneToOne,
    GhRepo,
    GhOrg,
    GhUser,
}

/// Unread messages and mentions
#[derive(Serialize, Deserialize, Debug)]
pub struct UnreadItems {
    /// Unread messages
    pub chat: Option<Vec<String>>,

    /// Mentioned messaged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention: Option<Vec<String>>,
}

impl UnreadItems {
    pub fn from_msg_ids(msg_ids: &Vec<String>) -> UnreadItems {
        UnreadItems {
            chat: Some(msg_ids.clone()),
            mention: None,
        }
    }
}

/// Github organization
#[derive(Deserialize, Debug)]
pub struct Organization {
    /// Organization ID
    pub id: u64,

    /// Organization name
    pub name: String,

    /// Organization avatar url
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,

    /// Organization respective room
    pub room: Option<Room>,
}

/// Github repository info
#[derive(Deserialize, Debug)]
pub struct Repository {
    /// Repository ID
    pub id: u64,

    /// Repository name
    pub name: String,
    
    /// Repository URI
    pub uri: String,

    /// Determines whether repository is private or not
    pub private: bool,

    /// Repository respective room
    pub room: Option<Room>,
}

/// Gitter channel
#[derive(Deserialize, Debug)]
pub struct Channel {
    /// Channel ID
    pub id: String,

    /// Channel name
    pub name: String,

    /// Channel topic
    pub topic: String,

    /// Channel URI
    pub uri: Option<String>,

    /// Is channel one-to-one
    #[serde(rename = "oneToOne")]
    pub one_to_one: bool,

    #[serde(rename = "unreadItems")]
    /// Unread items in channel
    pub unread_items: i32,

    /// Mentions in channel
    pub mentions: i32,

    /// Last access date and time
    #[serde(rename = "lastAccessTime")]
    pub last_access_time: Option<DateTime<UTC>>,

    pub lurk: bool,

    pub url: String,

    #[serde(rename = "githubType")]
    pub github_type: GithubType,

    pub security: String,
}

