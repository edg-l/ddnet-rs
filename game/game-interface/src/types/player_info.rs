pub use base::hash::Hash;
use base::network_string::NetworkString;
pub use ddnet_accounts_types::account_id::AccountId;
use hiarc::Hiarc;
use serde::{Deserialize, Serialize};

use super::{character_info::NetworkCharacterInfo, network_stats::PlayerNetworkStats};

/// Unique id for accounts, timeout codes etc.
#[derive(Debug, Hiarc, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize)]
pub enum PlayerUniqueId {
    Account(AccountId),
    CertFingerprint(Hash),
}

impl PlayerUniqueId {
    pub fn is_account_then<U, F: FnOnce(AccountId) -> Option<U>>(self, op: F) -> Option<U> {
        match self {
            PlayerUniqueId::Account(id) => op(id),
            PlayerUniqueId::CertFingerprint(_) => None,
        }
    }
}

/// a player from a client
#[derive(Debug, Hiarc, Serialize, Deserialize)]
pub struct PlayerClientInfo {
    pub info: NetworkCharacterInfo,
    /// An _unique_ id given by the client to this player, so the client
    /// can identify the player.
    /// It is also useful to restore after timeout in adition to
    /// the unique identifier. Because the unique identifier
    /// is shared among all players of a single client.
    pub id: u64,
    /// this is an (optional) unique identifier that
    /// allows to identify the client
    /// even after a reconnect.
    /// This is useful to store database entries
    /// using this id. (like an account)
    /// Or using it as timeout code (to restore the client if
    /// the player dropped).
    pub unique_identifier: PlayerUniqueId,
    /// Initial unreliable network statistic (might be guessed.).
    pub initial_network_stats: PlayerNetworkStats,
}

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub enum PlayerKickReason {
    Rcon,
    Custom(NetworkString<1024>),
}

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub enum PlayerBanReason {
    Vote,
    Rcon,
    Custom(NetworkString<1024>),
}

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub enum PlayerDropReason {
    /// Graceful disconnect
    Disconnect,
    /// Timeout
    Timeout,
    /// Kicked, e.g. by rcon
    Kicked(PlayerKickReason),
    /// Banned, e.g. by vote or rcon
    Banned {
        reason: PlayerBanReason,
        until: Option<chrono::DateTime<chrono::Utc>>,
    },
}
