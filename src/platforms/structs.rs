use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResolvedPeer {
    pub peer_id: String,
    pub peer_username: String,
}

impl ResolvedPeer {
    pub fn new(id: String, username: String) -> Self {
        Self {
            peer_id: id,
            peer_username: username,
        }
    }
}

impl Display for ResolvedPeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PEER_ID: {}\nPEER_USERNAME: {}",
            &self.peer_id, &self.peer_username
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerMessage {
    pub id: String,
    pub user_id: String,
    pub message: String,
    pub date: DateTime<Utc>,
}

impl PeerMessage {
    pub fn new(id: String, user_id: String, message: String, date: DateTime<Utc>) -> Self {
        Self {
            id,
            user_id,
            message,
            date,
        }
    }
}

impl Display for PeerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[USER_ID: {}, MESSAGE: {}, DATE: {}]",
            self.user_id, self.message, self.date
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DumpedPeer {
    pub peer: ResolvedPeer,
    pub chunks: Vec<PeerMessage>,
}

impl DumpedPeer {
    pub fn new(peer: ResolvedPeer, chunks: Vec<PeerMessage>) -> Self {
        Self {
            peer: peer,
            chunks: chunks,
        }
    }
}

impl Display for DumpedPeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[DUMPED_PEER]\n{}\n\n[", self.peer)?;
        for item in self.chunks.iter() {
            write!(f, "   {},\n", item)?;
        }
        write!(f, "]")
    }
}
