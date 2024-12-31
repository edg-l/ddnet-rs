use pool::datatypes::PoolVec;
use serde::{Deserialize, Serialize};

use crate::types::{
    id_types::PlayerId,
    player_info::{PlayerBanReason, PlayerKickReason},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TickEvent {
    Kick {
        player_id: PlayerId,
        reason: PlayerKickReason,
    },
    Ban {
        player_id: PlayerId,
        until: Option<chrono::DateTime<chrono::Utc>>,
        reason: PlayerBanReason,
    },
}

/// The tick result contains per tick data
/// usually only used inside the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickResult {
    /// Events that the server should handle
    pub events: PoolVec<TickEvent>,
}
