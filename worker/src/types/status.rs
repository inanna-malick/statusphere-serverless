use atrium_api::types::string::Did;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

///Status table datatype
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusWithHandle {
    pub uri: String,
    pub author_did: Did,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub indexed_at: DateTime<Utc>,
    pub seen_on_jetstream: bool,
    pub created_via_this_app: bool,
    pub handle: Option<String>,
}

///this is what we write to the db
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub uri: String,
    #[serde(rename = "authorDid")]
    pub author_did: Did,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,
}

///this is what we read from the db
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusFromDb {
    pub uri: String,
    #[serde(rename = "authorDid")]
    pub author_did: Did,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,
    #[serde(rename = "seenOnJetstream")]
    pub seen_on_jetstream: usize, // janky hax, it's stored as a number in sql...
    #[serde(rename = "createdViaThisApp")]
    pub created_via_this_app: usize, // janky hax, it's stored as a number in sql...
}

//Status methods
impl Status {
    pub fn new(uri: String, author_did: Did, status: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            uri,
            author_did,
            status,
            created_at: now,
            indexed_at: now,
        }
    }
}

impl From<StatusFromDb> for StatusWithHandle {
    fn from(value: StatusFromDb) -> Self {
        Self {
            uri: value.uri,
            author_did: value.author_did,
            status: value.status,
            created_at: value.created_at,
            indexed_at: value.indexed_at,
            seen_on_jetstream: value.seen_on_jetstream != 0,
            created_via_this_app: value.created_via_this_app != 0,
            handle: None,
        }
    }
}

// impl From<Status> for StatusWithHandle {
//     fn from(value: Status) -> Self {
//         Self {
//             uri: value.uri,
//             author_did: value.author_did,
//             status: value.status,
//             created_at: value.created_at,
//             indexed_at: value.indexed_at,
//             seen_on_jetstream: false,
//             handle: None,
//         }
//     }
// }

/// All the available emoji status options
pub const STATUS_OPTIONS: [&str; 30] = [
    "👍",
    "👎",
    "💙",
    "🥹",
    "😤",
    "🙃",
    "😉",
    "😎",
    "🤨",
    "🥳",
    "😭",
    "🏴‍☠️",
    "🤯",
    "🫡",
    "💀",
    "✊",
    "🤘",
    "👀",
    "🧠",
    "👩‍💻",
    "🧑‍💻",
    "🥷",
    "🏳️‍🌈",
    "🚀",
    "🥔",
    "🦀",
    "🏳️‍⚧️",
    "💖",
    "🪞",
    "✨",
];
