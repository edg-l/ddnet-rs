use ui_base::types::{UiRenderPipe, UiState};

pub trait UiPageInterface<U> {
    /// actually render the ui
    fn render(&mut self, ui: &mut egui::Ui, pipe: &mut UiRenderPipe<U>, ui_state: &mut UiState);

    /// called exactly once, when the ui was mounted and the implementation that uses
    /// this ui supports this event.
    /// This event is usually useful to prepare some resources.
    fn mount(&mut self) {}

    /// Called exactly once, when the ui is about to be unmounted and the implementation that uses
    /// this ui supports this event.
    /// This event is usually useful to free some reasources.
    /// For reliable cleanup the destructor/Drop should still be prefered
    fn unmount(&mut self) {}
}
