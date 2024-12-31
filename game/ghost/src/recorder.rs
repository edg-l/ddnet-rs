use std::collections::HashMap;

use demo::recorder::{DemoRecorder, DemoRecorderCreateProps};
use game_interface::{
    events::{GameEvents, GameWorldAction, GameWorldEvent, GameWorldNotificationEvent},
    ghosts::GhostResultPlayer,
    interface::GameStateInterface,
    types::{game::NonZeroGameTickType, id_types::PlayerId},
};

use pool::mt_datatypes::PoolCow as MtPoolCow;

#[derive(Debug)]
pub struct GhostRecorder {
    players: HashMap<PlayerId, DemoRecorder>,

    props: DemoRecorderCreateProps,
    ticks_per_second: NonZeroGameTickType,
    base_name: String,
}

impl GhostRecorder {
    pub fn new(
        props: DemoRecorderCreateProps,
        ticks_per_second: NonZeroGameTickType,
        base_name: String,
    ) -> Self {
        Self {
            players: Default::default(),
            props,
            ticks_per_second,
            base_name,
        }
    }

    pub fn on_snapshot(
        &mut self,
        monotonic_tick: u64,
        snapshot: &MtPoolCow<'static, [u8]>,
        game: &mut dyn GameStateInterface,
    ) {
        let mut ghosts = game.build_ghosts_from_snapshot(snapshot);

        for (player_id, ghost) in ghosts.players.drain() {
            match ghost {
                GhostResultPlayer::GhostInactive { ghost_snapshot } => {
                    if let Some(demo_recorder) = self.players.get_mut(&player_id) {
                        demo_recorder.add_snapshot(monotonic_tick, ghost_snapshot.to_vec());
                    }
                }
                GhostResultPlayer::GhostRecordStarted { ghost_snapshot }
                | GhostResultPlayer::GhostRecordActive { ghost_snapshot } => {
                    let demo_recorder = self.players.entry(player_id).or_insert_with(|| {
                        DemoRecorder::new(
                            self.props.clone(),
                            self.ticks_per_second,
                            Some("ghosts".as_ref()),
                            Some(self.base_name.clone()),
                        )
                    });

                    demo_recorder.add_snapshot(monotonic_tick, ghost_snapshot.to_vec());
                }
            }
        }
    }

    pub fn on_event(&mut self, events: &GameEvents) {
        fn on_finish(players: &mut HashMap<PlayerId, DemoRecorder>, player_id: &PlayerId) {
            // finish
            players.remove(player_id);
        }

        for world in events.worlds.values() {
            for event in world.events.values() {
                match event {
                    GameWorldEvent::Sound(_) | GameWorldEvent::Effect(_) => {
                        // ignore
                    }
                    GameWorldEvent::Notification(event) => match event {
                        GameWorldNotificationEvent::Action(ev) => match ev {
                            GameWorldAction::RaceFinish { character, .. } => {
                                on_finish(&mut self.players, character)
                            }
                            GameWorldAction::RaceTeamFinish { characters, .. } => {
                                for character in characters.iter() {
                                    on_finish(&mut self.players, character);
                                }
                            }
                            GameWorldAction::Kill { victims, .. } => {
                                // reset ghost on kill
                                for victim in victims.iter() {
                                    if let Some(demo) = self.players.remove(victim) {
                                        demo.cancel();
                                    }
                                }
                            }
                            GameWorldAction::Custom(_) => {
                                // ignore
                            }
                        },
                        GameWorldNotificationEvent::System(_)
                        | GameWorldNotificationEvent::Motd { .. } => {
                            // ignore
                        }
                    },
                }
            }
        }
    }
}
