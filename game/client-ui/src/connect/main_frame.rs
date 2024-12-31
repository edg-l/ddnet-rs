use egui::{Frame, Layout, Pos2, Rect, UiBuilder, Vec2};

use ui_base::{
    style::bg_frame_color,
    types::{UiRenderPipe, UiState},
    utils::add_margins,
};

use crate::events::UiEvent;

use super::user_data::{ConnectModes, UserData};

pub fn render_modes(ui: &mut egui::Ui, pipe: &mut UiRenderPipe<UserData>) {
    match pipe.user_data.mode.get() {
        ConnectModes::Connecting { addr } => {
            ui.vertical(|ui| {
                ui.label(format!("Connecting to:\n{}", addr));
            });
            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                if ui.button("Cancel").clicked() {
                    pipe.user_data.events.push(UiEvent::Disconnect);
                    pipe.user_data.config.engine.ui.path.route("");
                }
            });
        }
        ConnectModes::ConnectingErr { msg } => {
            ui.vertical(|ui| {
                ui.label(format!(
                    "Connecting to {} failed:\n{}",
                    pipe.user_data.config.storage::<String>("server-addr"),
                    msg
                ));
            });
            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                if ui.button("Return").clicked() {
                    pipe.user_data.events.push(UiEvent::Disconnect);
                    pipe.user_data.config.engine.ui.path.route("");
                }
            });
        }
        ConnectModes::Queue { msg } => {
            ui.vertical(|ui| {
                ui.label(format!(
                    "Connecting to {}",
                    pipe.user_data.config.storage::<String>("server-addr")
                ));
                ui.label(format!("Waiting in queue: {}", msg));
            });
            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                if ui.button("Cancel").clicked() {
                    pipe.user_data.events.push(UiEvent::Disconnect);
                    pipe.user_data.config.engine.ui.path.route("");
                }
            });
        }
        ConnectModes::DisconnectErr { msg } => {
            ui.vertical(|ui| {
                ui.label(format!(
                    "Connection to {} lost:\n{}",
                    pipe.user_data.config.storage::<String>("server-addr"),
                    msg
                ));
            });
            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                if ui.button("Return").clicked() {
                    pipe.user_data.events.push(UiEvent::Disconnect);
                    pipe.user_data.config.engine.ui.path.route("");
                }
            });
        }
    }
}

/// top bar
/// big square, rounded edges
pub fn render(ui: &mut egui::Ui, ui_state: &mut UiState, pipe: &mut UiRenderPipe<UserData>) {
    let width = ui.available_width().min(200.0);
    let height = ui.available_height().min(100.0);
    let offset_x = (ui.available_width() / 2.0) - (width / 2.0);
    let offset_y = (ui.available_height() / 2.0) - (height / 2.0);
    let rect = Rect::from_min_size(Pos2::new(offset_x, offset_y), Vec2::new(width, height));
    ui.allocate_new_ui(UiBuilder::new().max_rect(rect), |ui| {
        ui.set_width(rect.width());
        ui.set_height(rect.height());

        let res = Frame::default()
            .fill(bg_frame_color())
            .rounding(5.0)
            .show(ui, |ui| {
                ui.set_width(rect.width());
                ui.set_height(rect.height());
                add_margins(ui, |ui| render_modes(ui, pipe));
            });
        ui_state.add_blur_rect(res.response.rect, 5.0);
    });
}
