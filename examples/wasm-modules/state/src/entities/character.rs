pub mod core;
pub mod player;
pub mod pos {
    pub use ::vanilla::entities::character::pos::*;
}
pub mod hook {
    pub use ::vanilla::entities::character::hook::*;
}
pub mod score {
    pub use ::vanilla::entities::character::score::*;
}

use api_macros::character_mod;

#[character_mod("../../../")]
pub mod character {}
