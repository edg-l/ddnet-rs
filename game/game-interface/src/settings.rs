use hiarc::Hiarc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub struct GameStateSettings {
    /// How many players are allowed to play.
    ///
    /// clients above this limits must then be spectators.
    pub max_ingame_players: u32,
    /// Whether the game is currently in a tournament mode
    pub tournament_mode: bool,
}
