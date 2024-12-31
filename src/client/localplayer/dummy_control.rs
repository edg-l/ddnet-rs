use std::time::Duration;

#[derive(Debug, Default, Clone, Copy)]
pub enum DummyHammerState {
    #[default]
    None,
    Active {
        last_hammer: Option<Duration>,
    },
}

#[derive(Debug, Default)]
pub struct DummyControlState {
    // dummy controls
    pub dummy_copy_moves: bool,
    pub dummy_hammer: DummyHammerState,
}
