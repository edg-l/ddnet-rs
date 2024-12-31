use ui_base::types::UiRenderPipe;

use super::user_data::{ChatEvent, ChatMode, UserData};

/// chat input
pub fn render(ui: &mut egui::Ui, pipe: &mut UiRenderPipe<UserData>) {
    if pipe.user_data.is_input_active {
        ui.horizontal(|ui| {
            ui.label(match pipe.user_data.mode {
                ChatMode::Global => "All:".to_string(),
                ChatMode::Team => "Team:".to_string(),
                ChatMode::Whisper(player_id) => format!("To {:?}:", player_id),
            });
            let label = ui.text_edit_singleline(pipe.user_data.msg);
            if label.lost_focus() {
                pipe.user_data.chat_events.push(ChatEvent::ChatClosed);
                if matches!(pipe.user_data.mode, ChatMode::Whisper(Some(_)))
                    || !matches!(pipe.user_data.mode, ChatMode::Whisper(_))
                {
                    pipe.user_data.chat_events.push(ChatEvent::MsgSend {
                        msg: pipe.user_data.msg.clone(),
                        mode: pipe.user_data.mode,
                    });
                }
            } else {
                pipe.user_data.chat_events.push(ChatEvent::CurMsg {
                    msg: pipe.user_data.msg.clone(),
                    mode: pipe.user_data.mode,
                });

                label.request_focus();
            }
        });
    }
}
