use std::time::Duration;

use base::network_string::PoolNetworkString;
use hiarc::Hiarc;
use math::math::vector::ubvec4;
use pool::datatypes::{PoolFxLinkedHashMap, PoolVec};
use serde::{Deserialize, Serialize};

use crate::{
    client_commands::MAX_TEAM_NAME_LEN,
    interface::MAX_MAP_NAME_LEN,
    types::{
        id_types::{CharacterId, StageId},
        network_stats::PlayerNetworkStats,
    },
};

#[derive(Debug, Hiarc, Clone, Copy, Serialize, Deserialize)]
pub enum ScoreboardConnectionType {
    /// "Normal" network connection
    Network(PlayerNetworkStats),
    /// A local/server-side bot
    Bot,
}

#[derive(Debug, Hiarc, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScoreboardScoreType {
    Points(i64),
    RaceFinishTime(Duration),
    None,
}

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub struct ScoreboardCharacterInfo {
    pub id: CharacterId,
    pub score: ScoreboardScoreType,
    pub ping: ScoreboardConnectionType,
}

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub struct ScoreboardStageInfo {
    pub characters: PoolVec<ScoreboardCharacterInfo>,
    pub name: PoolNetworkString<MAX_TEAM_NAME_LEN>,
    pub max_size: usize,
    pub color: ubvec4,

    /// If score related gameplay is active, this is the highest score.
    ///
    /// In team play this is usually the score of red or blue,
    /// in solo or race this could be the highest score or fastest time.
    pub score: ScoreboardScoreType,
}

pub type ScoreboardPlayerSpectatorInfo = ScoreboardCharacterInfo;

pub const MAX_SIDE_NAME_LEN: usize = MAX_TEAM_NAME_LEN;
#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub enum ScoreboardGameType {
    /// side = vanilla team/side
    /// stage = ddrace team
    SidedPlay {
        red_stages: PoolFxLinkedHashMap<StageId, ScoreboardStageInfo>,
        blue_stages: PoolFxLinkedHashMap<StageId, ScoreboardStageInfo>,
        spectator_players: PoolVec<ScoreboardPlayerSpectatorInfo>,

        /// This stage is going to be ignored in scoreboard rendering
        /// E.g. team 0 in ddrace has no background color
        ignore_stage: StageId,

        red_side_name: PoolNetworkString<MAX_SIDE_NAME_LEN>,
        blue_side_name: PoolNetworkString<MAX_SIDE_NAME_LEN>,
    },
    SoloPlay {
        stages: PoolFxLinkedHashMap<StageId, ScoreboardStageInfo>,

        /// This stage is going to be ignored in scoreboard rendering
        /// E.g. team 0 in ddrace has no background color
        ignore_stage: StageId,

        spectator_players: PoolVec<ScoreboardPlayerSpectatorInfo>,
    },
}

#[derive(Debug, Hiarc, Clone, Copy, Serialize, Deserialize)]
pub enum ScoreboardGameTypeOptions {
    Match {
        score_limit: u64,
        time_limit: Option<Duration>,
    },
    Race {
        time_limit: Option<Duration>,
    },
}

#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub struct ScoreboardGameOptions {
    pub ty: ScoreboardGameTypeOptions,
    pub map_name: PoolNetworkString<MAX_MAP_NAME_LEN>,
}

#[derive(Debug, Hiarc, Serialize, Deserialize)]
pub struct Scoreboard {
    pub game: ScoreboardGameType,
    pub options: ScoreboardGameOptions,
}
