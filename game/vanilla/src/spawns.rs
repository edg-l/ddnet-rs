use hiarc::Hiarc;
use math::math::vector::vec2;

#[derive(Debug, Hiarc)]
pub struct GameSpawns {
    pub spawns: Vec<vec2>,
    pub spawns_red: Vec<vec2>,
    pub spawns_blue: Vec<vec2>,
}
