use egui_extras::{Size, StripBuilder};

use game_base::server_browser::ServerFilter;
use ui_base::{components::clearable_edit_field::clearable_edit_field, types::UiRenderPipe};

use crate::main_menu::user_data::UserData;

fn exclude_menu(ui: &mut egui::Ui, pipe: &mut UiRenderPipe<UserData>) {
    ui.label("Exclude words\n(seperated by \";\")");
    let mut filter = pipe
        .user_data
        .config
        .storage::<ServerFilter>("browser_filter");
    if ui.text_edit_singleline(&mut filter.exclude).changed() {
        pipe.user_data.config.set_storage("browser_filter", &filter);
    }
}

/// search field
pub fn render(ui: &mut egui::Ui, pipe: &mut UiRenderPipe<UserData>) {
    ui.style_mut().spacing.item_spacing.x = 2.0;
    StripBuilder::new(ui)
        .size(Size::exact(20.0))
        .size(Size::remainder())
        .size(Size::exact(20.0))
        .horizontal(|mut strip| {
            strip.cell(|ui| {
                ui.style_mut().wrap_mode = None;
                // search icon
                ui.label("\u{1f50d}");
            });
            strip.cell(|ui| {
                ui.style_mut().wrap_mode = None;
                let mut filter = pipe
                    .user_data
                    .config
                    .storage::<ServerFilter>("browser_filter");
                if clearable_edit_field(ui, &mut filter.search, None, None)
                    .is_some_and(|r| r.changed())
                {
                    pipe.user_data.config.set_storage("browser_filter", &filter);
                }
            });
            strip.cell(|ui| {
                ui.style_mut().wrap_mode = None;
                // exclude
                ui.menu_button("\u{f05e}", |ui| exclude_menu(ui, pipe));
            });
        });
}
