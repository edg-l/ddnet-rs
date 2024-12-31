use ui_base::{components::clearable_edit_field::clearable_edit_field, types::UiRenderPipe};

use crate::main_menu::user_data::UserData;

/// server address input field
pub fn render(ui: &mut egui::Ui, pipe: &mut UiRenderPipe<UserData>) {
    ui.horizontal(|ui| {
        ui.label("\u{f233} - Address:");
    });
    let mut cur_address: String = pipe.user_data.config.storage::<String>("server-addr");
    if clearable_edit_field(ui, &mut cur_address, Some(200.0), None)
        .map(|res| res.changed())
        .unwrap_or_default()
    {
        pipe.user_data
            .config
            .set_storage("server-addr", &cur_address);
    }
}
