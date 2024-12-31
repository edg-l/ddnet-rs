use serde::{Deserialize, Serialize};

use pool::datatypes::{PoolCow, PoolFxHashMap};

use crate::types::id_types::PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GhostResultPlayer {
    /// The ghost is currently inactive.
    /// In race this would be before a player
    /// started the race or after he finished
    /// the reace.
    ///
    /// __IMPORTANT__: The recording of the ghost
    /// can still be continued once it started.
    /// The client usually detects when to stop a
    /// recording by the kill & finish messages
    /// (see [`crate::events::GameWorldAction`]).
    GhostInactive {
        ghost_snapshot: PoolCow<'static, [u8]>,
    },
    /// The ghost recorded started in this tick.
    ///
    /// This variant is usually also used to start
    /// ghost replaying of the client (or resets it).
    GhostRecordStarted {
        ghost_snapshot: PoolCow<'static, [u8]>,
    },
    /// The ghost record is active, if it was previously not
    /// active, then this is equivalent to
    /// [`GhostResult::GhostRecordStarted`].
    ///
    /// This also means that [`GhostResult::GhostRecordStarted`]
    /// could theoretically be ignored, but e.g. for race mods
    /// if a player stands inside the start tile, then
    /// only [`GhostResult::GhostRecordStarted`] would reset the ghost
    /// replaying the whole time.
    GhostRecordActive {
        ghost_snapshot: PoolCow<'static, [u8]>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostResult {
    pub players: PoolFxHashMap<PlayerId, GhostResultPlayer>,
}
