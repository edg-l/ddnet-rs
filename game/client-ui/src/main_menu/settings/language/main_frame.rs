use crate::main_menu::user_data::UserData;
use ui_base::types::{UiRenderPipe, UiState};

pub fn render(ui: &mut egui::Ui, pipe: &mut UiRenderPipe<UserData>, ui_state: &mut UiState) {
    super::list::lang_list(ui, pipe, ui_state)
}
