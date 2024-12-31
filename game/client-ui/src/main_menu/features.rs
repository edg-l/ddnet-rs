use hiarc::Hiarc;

#[derive(Debug, Hiarc, Default, Clone, Copy)]
pub struct EnabledFeatures {
    pub demo_to_video: bool,
    pub spatial_chat: bool,
}
