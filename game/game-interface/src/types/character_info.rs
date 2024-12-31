use base::network_string::NetworkString;
use hiarc::Hiarc;
use math::math::vector::ubvec4;
use serde::{Deserialize, Serialize};

use crate::types::resource_key::NetworkResourceKey;

use super::render::character::TeeEye;

// # network part
#[derive(Debug, Hiarc, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSkinInfo {
    #[default]
    Original,
    Custom {
        body_color: ubvec4,
        feet_color: ubvec4,
    },
}

#[derive(Debug, Hiarc, Copy, Clone, Serialize, Deserialize)]
pub struct NetworkLaserInfo {
    pub inner_color: ubvec4,
    pub outer_color: ubvec4,
}

impl Default for NetworkLaserInfo {
    fn default() -> Self {
        Self {
            inner_color: ubvec4::new(255, 255, 255, 255),
            outer_color: ubvec4::new(255, 255, 255, 255),
        }
    }
}

pub const MAX_CHARACTER_NAME_LEN: usize = 16;
pub const MAX_CHARACTER_CLAN_LEN: usize = 12;
pub const MAX_FLAG_NAME_LEN: usize = 7;
pub const MAX_LANG_NAME_LEN: usize = 13;
pub const MAX_ASSET_NAME_LEN: usize = 24;

#[derive(Debug, Hiarc, Default, Clone, Serialize, Deserialize)]
pub struct NetworkCharacterInfo {
    pub name: NetworkString<MAX_CHARACTER_NAME_LEN>,
    pub clan: NetworkString<MAX_CHARACTER_CLAN_LEN>,
    /// Country has a max length of 7 characters
    /// ISO 3166-2 needs 6 characters.
    /// The word "default" 7 (for the tee-ish flag).
    pub flag: NetworkString<MAX_FLAG_NAME_LEN>,
    /// The language id has a max length of 13 (language+script or language+region)
    /// characters allowing most common languages.
    /// (https://www.rfc-editor.org/rfc/rfc5646#section-4.4.1)
    pub lang: NetworkString<MAX_LANG_NAME_LEN>,

    pub skin_info: NetworkSkinInfo,
    pub laser_info: NetworkLaserInfo,

    // resources
    pub skin: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub weapon: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub freeze: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub ninja: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub game: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub ctf: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub hud: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub entities: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub emoticons: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub particles: NetworkResourceKey<MAX_ASSET_NAME_LEN>,
    pub hook: NetworkResourceKey<MAX_ASSET_NAME_LEN>,

    /// The default eyes to use, if the server supports settings
    /// custom eyes.
    pub default_eyes: TeeEye,
}

impl NetworkCharacterInfo {
    // only provide a default that makes clear you used default
    pub fn explicit_default() -> Self {
        Self {
            name: NetworkString::new("TODO").unwrap(),
            clan: NetworkString::new("TODO").unwrap(),

            flag: NetworkString::new("default").unwrap(),
            lang: NetworkString::new("en").unwrap(),

            skin_info: NetworkSkinInfo::Original,
            laser_info: NetworkLaserInfo::default(),

            skin: "default".try_into().unwrap(),
            weapon: "default".try_into().unwrap(),
            ninja: "default".try_into().unwrap(),
            freeze: "default".try_into().unwrap(),
            game: "default".try_into().unwrap(),
            ctf: "default".try_into().unwrap(),
            hud: "default".try_into().unwrap(),
            entities: "default".try_into().unwrap(),
            emoticons: "default".try_into().unwrap(),
            particles: "default".try_into().unwrap(),
            hook: "default".try_into().unwrap(),

            default_eyes: TeeEye::Normal,
        }
    }
}
