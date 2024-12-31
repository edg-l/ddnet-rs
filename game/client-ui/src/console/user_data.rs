use std::collections::VecDeque;

use client_containers::skins::SkinContainer;
use client_render_base::render::tee::RenderTee;
use client_types::console::ConsoleEntry;
use command_parser::parser::ParserCache;
use game_config::config::Config;
use ui_base::types::UiState;

pub struct UserData<'a> {
    pub entries: &'a Vec<ConsoleEntry>,
    pub config: &'a mut Config,
    pub msgs: &'a mut String,
    pub msg: &'a mut String,
    pub msg_history: &'a mut VecDeque<String>,
    pub msg_history_index: &'a mut Option<usize>,
    pub cursor: &'a mut usize,
    pub select_index: &'a mut Option<usize>,

    pub custom_matches: &'a dyn Fn(&str) -> Option<Vec<String>>,
    #[allow(clippy::type_complexity)]
    pub render_custom_matches:
        &'a dyn Fn(&str, &str, &mut egui::Ui, &mut UiState, &mut SkinContainer, &RenderTee),

    pub skin_container: &'a mut SkinContainer,
    pub render_tee: &'a RenderTee,

    pub cache: &'a mut ParserCache,

    pub can_change_client_config: bool,
}
