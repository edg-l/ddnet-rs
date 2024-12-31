use base::network_string::NetworkReducedAsciiString;
use game_database::types::UnixUtcTimestamp;
use hiarc::Hiarc;
use serde::{Deserialize, Serialize};

pub const MAX_ACCOUNT_NAME_LEN: usize = 32;

/// Account information that the client can interpret by default.
#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// The name of the account on this game server
    pub name: NetworkReducedAsciiString<MAX_ACCOUNT_NAME_LEN>,
    /// The date when the account was first registered
    /// on this game server.
    pub creation_date: UnixUtcTimestamp,
}
