pub mod types {
    use std::{ops::Deref, rc::Rc, time::Duration};

    use hiarc::Hiarc;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Hiarc, Clone, Copy, Default, Serialize, Deserialize)]
    pub enum GameType {
        #[default]
        Solo,
        Team,
    }

    #[derive(Debug, Hiarc, Clone, Copy)]
    pub struct GameOptionsInner {
        pub ty: GameType,
        pub score_limit: u64,
        pub time_limit: Option<Duration>,
        pub sided_balance_time: Option<Duration>,
        pub friendly_fire: bool,
        pub laser_hit_self: bool,
    }

    #[derive(Debug, Hiarc, Clone)]
    pub struct GameOptions(Rc<GameOptionsInner>);

    impl GameOptions {
        pub fn new(
            ty: GameType,
            score_limit: u64,
            time_limit: Option<Duration>,
            sided_balance_time: Option<Duration>,
            friendly_fire: bool,
            laser_hit_self: bool,
        ) -> Self {
            Self(Rc::new(GameOptionsInner {
                ty,
                score_limit,
                time_limit,
                sided_balance_time,
                friendly_fire,
                laser_hit_self,
            }))
        }
    }

    impl Deref for GameOptions {
        type Target = GameOptionsInner;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
