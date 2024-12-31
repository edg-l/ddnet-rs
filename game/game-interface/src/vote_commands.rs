use base::network_string::{NetworkReducedAsciiString, NetworkString};
use hiarc::Hiarc;
use serde::{Deserialize, Serialize};

use crate::{
    interface::MAX_MAP_NAME_LEN, types::id_types::PlayerId, votes::RandomUnfinishedMapKey,
};

/// Represents a vote command that usually
/// results from a completed vote.
#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub enum VoteCommand {
    /// A player was moved to spec
    JoinSpectator(PlayerId),
    /// A rcon command as a result of a vote
    Misc(NetworkString<{ 65536 * 2 + 1 }>),
    /// A random unfinished map was loaded
    RandomUnfinishedMap(RandomUnfinishedMapKey),
}

/// Represents a vote command result type.
#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub enum VoteCommandResultEvent {
    LoadMap {
        map: NetworkReducedAsciiString<MAX_MAP_NAME_LEN>,
    },
}

/// Represents a vote command result.
#[derive(Debug, Hiarc, Default, Clone, Serialize, Deserialize)]
pub struct VoteCommandResult {
    pub events: Vec<VoteCommandResultEvent>,
}
