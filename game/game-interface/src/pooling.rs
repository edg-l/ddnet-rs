use std::borrow::Cow;

use base::network_string::{MtNetworkStringPool, NetworkStringPool};
use hashlink::LinkedHashMap;
use hiarc::Hiarc;
use pool::mt_pool::Pool as MtPool;
use pool::pool::Pool;
use rustc_hash::FxHashSet;

use crate::account_info::MAX_ACCOUNT_NAME_LEN;
use crate::client_commands::MAX_TEAM_NAME_LEN;
use crate::events::{EventId, GameWorldEvent, GameWorldEvents};
use crate::interface::MAX_MAP_NAME_LEN;
use crate::types::character_info::{MAX_ASSET_NAME_LEN, MAX_CHARACTER_NAME_LEN};
use crate::types::id_types::{CharacterId, CtfFlagId, LaserId, PickupId, ProjectileId, StageId};
use crate::types::render::character::{
    CharacterBuff, CharacterBuffInfo, CharacterDebuff, CharacterDebuffInfo, CharacterInfo,
    CharacterRenderInfo, MAX_SCORE_STR_LEN,
};
use crate::types::render::flag::FlagRenderInfo;
use crate::types::render::game::MatchRoundGameOverWinnerCharacter;
use crate::types::render::laser::LaserRenderInfo;
use crate::types::render::pickup::PickupRenderInfo;
use crate::types::render::projectiles::ProjectileRenderInfo;
use crate::types::render::scoreboard::{
    ScoreboardCharacterInfo, ScoreboardPlayerSpectatorInfo, ScoreboardStageInfo,
};
use crate::types::render::stage::StageRenderInfo;
use crate::types::resource_key::{MtNetworkResourceKeyPool, NetworkResourceKeyPool};

/// Make your life easier by simply using all required pools for the interface
#[derive(Debug, Hiarc)]
pub struct GamePooling {
    pub mt_network_string_name_pool: MtNetworkStringPool<MAX_CHARACTER_NAME_LEN>,
    pub network_string_name_pool: NetworkStringPool<MAX_CHARACTER_NAME_LEN>,
    pub mt_network_string_common_pool: MtNetworkStringPool<1024>,
    pub network_string_map_pool: NetworkStringPool<MAX_MAP_NAME_LEN>,
    pub network_string_team_pool: NetworkStringPool<MAX_TEAM_NAME_LEN>,
    pub network_string_score_pool: NetworkStringPool<MAX_SCORE_STR_LEN>,
    pub network_string_account_name_pool: NetworkStringPool<MAX_ACCOUNT_NAME_LEN>,
    pub resource_key_pool: NetworkResourceKeyPool<MAX_ASSET_NAME_LEN>,
    pub mt_resource_key_pool: MtNetworkResourceKeyPool<MAX_ASSET_NAME_LEN>,
    pub game_over_winner_character_pool: Pool<Vec<MatchRoundGameOverWinnerCharacter>>,
    pub stage_render_info: Pool<LinkedHashMap<StageId, StageRenderInfo, rustc_hash::FxBuildHasher>>,
    pub character_render_info_pool:
        Pool<LinkedHashMap<CharacterId, CharacterRenderInfo, rustc_hash::FxBuildHasher>>,
    pub character_info_pool:
        Pool<LinkedHashMap<CharacterId, CharacterInfo, rustc_hash::FxBuildHasher>>,
    pub character_id_pool: MtPool<Vec<CharacterId>>,
    pub character_id_hashset_pool: Pool<FxHashSet<CharacterId>>,
    pub projectile_render_info_pool:
        Pool<LinkedHashMap<ProjectileId, ProjectileRenderInfo, rustc_hash::FxBuildHasher>>,
    pub flag_render_info_pool:
        Pool<LinkedHashMap<CtfFlagId, FlagRenderInfo, rustc_hash::FxBuildHasher>>,
    pub laser_render_info_pool:
        Pool<LinkedHashMap<LaserId, LaserRenderInfo, rustc_hash::FxBuildHasher>>,
    pub pickup_render_info_pool:
        Pool<LinkedHashMap<PickupId, PickupRenderInfo, rustc_hash::FxBuildHasher>>,
    pub stage_scoreboard_pool:
        Pool<LinkedHashMap<StageId, ScoreboardStageInfo, rustc_hash::FxBuildHasher>>,
    pub character_scoreboard_pool: Pool<Vec<ScoreboardCharacterInfo>>,
    pub player_spectator_scoreboard_pool: Pool<Vec<ScoreboardPlayerSpectatorInfo>>,
    pub character_buffs:
        Pool<LinkedHashMap<CharacterBuff, CharacterBuffInfo, rustc_hash::FxBuildHasher>>,
    pub character_debuffs:
        Pool<LinkedHashMap<CharacterDebuff, CharacterDebuffInfo, rustc_hash::FxBuildHasher>>,
    pub snapshot_pool: MtPool<Cow<'static, [u8]>>,
    pub worlds_events_pool:
        MtPool<LinkedHashMap<StageId, GameWorldEvents, rustc_hash::FxBuildHasher>>,
    pub world_events_pool:
        MtPool<LinkedHashMap<EventId, GameWorldEvent, rustc_hash::FxBuildHasher>>,
}

impl GamePooling {
    pub fn new(hint_max_characters: Option<usize>) -> Self {
        // limit the maximum to prevent early memory exhaustion
        let hint_max_characters = hint_max_characters.unwrap_or(64).min(512);
        let hint_max_characters_client = hint_max_characters.min(64);
        Self {
            mt_network_string_name_pool: MtPool::with_capacity(hint_max_characters),
            network_string_name_pool: Pool::with_capacity(hint_max_characters),
            mt_network_string_common_pool: MtPool::with_capacity(2),
            network_string_map_pool: Pool::with_capacity(2),
            network_string_team_pool: Pool::with_capacity(2),
            network_string_score_pool: Pool::with_capacity(2),
            network_string_account_name_pool: Pool::with_capacity(hint_max_characters),
            resource_key_pool: NetworkResourceKeyPool::with_capacity(hint_max_characters),
            mt_resource_key_pool: MtNetworkResourceKeyPool::with_capacity(hint_max_characters),
            game_over_winner_character_pool: Pool::with_capacity(2),
            stage_render_info: Pool::with_capacity(2),
            character_render_info_pool: Pool::with_capacity(hint_max_characters_client),
            character_info_pool: Pool::with_capacity(hint_max_characters),
            character_id_pool: MtPool::with_capacity(hint_max_characters),
            character_id_hashset_pool: Pool::with_capacity(hint_max_characters),
            projectile_render_info_pool: Pool::with_capacity(hint_max_characters_client),
            flag_render_info_pool: Pool::with_capacity(hint_max_characters_client),
            laser_render_info_pool: Pool::with_capacity(hint_max_characters_client),
            pickup_render_info_pool: Pool::with_capacity(hint_max_characters_client),
            stage_scoreboard_pool: Pool::with_capacity(2),
            character_scoreboard_pool: Pool::with_capacity(hint_max_characters_client),
            player_spectator_scoreboard_pool: Pool::with_capacity(hint_max_characters_client),
            character_buffs: Pool::with_capacity(hint_max_characters_client),
            character_debuffs: Pool::with_capacity(hint_max_characters_client),
            snapshot_pool: MtPool::with_capacity(2),
            worlds_events_pool: MtPool::with_capacity(2),
            world_events_pool: MtPool::with_capacity(hint_max_characters),
        }
    }
}
