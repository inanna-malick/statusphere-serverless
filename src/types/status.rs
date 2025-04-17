use atrium_api::types::string::Did;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

///Status table datatype
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusWithHandle {
    pub uri: String,
    pub author_did: Did,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub indexed_at: DateTime<Utc>,
    pub source: Source,
    pub handle: Option<String>,
}

///Status table datatype
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
    pub source: Source,
}

//Status methods
impl StatusFromDb {
    /// Creates a new [StatusFromDb]
    pub fn new(uri: String, author_did: Did, status: String, source: Source) -> Self {
        let now = chrono::Utc::now();
        Self {
            uri,
            author_did,
            status,
            created_at: now,
            indexed_at: now,
            source,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Source {
    Jetstream,
    ThisInstance,
    ThisInstanceAndJetstream,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Jetstream => write!(f, "jetstream"),
            Source::ThisInstance => write!(f, "this instance"),
            Source::ThisInstanceAndJetstream => write!(f, "this instance and jetstream"),
        }
    }
}

impl StatusWithHandle {
    pub fn from_db(db: StatusFromDb) -> Self {
        Self {
            uri: db.uri,
            author_did: db.author_did,
            status: db.status,
            created_at: db.created_at,
            indexed_at: db.indexed_at,
            source: db.source,
            handle: None,
        }
    }
}

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
    "😤",
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
    "🧌",
    "🚀",
    "🥔",
    "🦀",
    "🏳️‍⚧️",
    "💖",
    "🪞",
    "✨",
];
