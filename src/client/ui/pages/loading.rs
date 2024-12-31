use game_config::config::Config;
use ui_generic::traits::UiPageInterface;

pub struct LoadingPage {}

impl Default for LoadingPage {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadingPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl UiPageInterface<Config> for LoadingPage {
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        _pipe: &mut ui_base::types::UiRenderPipe<Config>,
        _ui_state: &mut ui_base::types::UiState,
    ) {
        ui.label("Loading page...");
    }
}
