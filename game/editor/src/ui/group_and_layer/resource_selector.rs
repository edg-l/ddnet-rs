use egui::Button;
use map::skeleton::resources::MapResourceRefSkeleton;
use ui_base::types::UiState;

#[derive(Debug)]
pub enum ResourceSelectionMode {
    Hovered(Option<usize>),
    Clicked(Option<usize>),
}

#[derive(Debug, Default)]
pub struct ResourceSelectionResult {
    pub mode: Option<ResourceSelectionMode>,

    pub pointer_was_outside: bool,
}

pub fn render<R>(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    pointer_is_used: &mut bool,
    resources: &[MapResourceRefSkeleton<R>],
) -> ResourceSelectionResult {
    let mut resource_res = ResourceSelectionResult::default();

    let window_res = {
        let window = egui::Window::new("Resource selector")
            .resizable(false)
            .collapsible(false);

        window.show(ui.ctx(), |ui| {
            let res = ui.add(Button::new("None"));
            if res.clicked() {
                resource_res.mode = Some(ResourceSelectionMode::Clicked(None));
            } else if res.hovered() {
                resource_res.mode = Some(ResourceSelectionMode::Hovered(None));
            }
            for (index, res) in resources.iter().enumerate() {
                let res = ui.add(Button::new(res.def.name.as_str()));
                if res.clicked() {
                    resource_res.mode = Some(ResourceSelectionMode::Clicked(Some(index)));
                } else if res.hovered() {
                    resource_res.mode = Some(ResourceSelectionMode::Hovered(Some(index)));
                }
            }
        })
    };

    *pointer_is_used |= if let Some(window_res) = &window_res {
        let intersected = ui.input(|i| {
            if i.pointer.primary_down() {
                Some((
                    !window_res.response.rect.intersects({
                        let min = i.pointer.interact_pos().unwrap_or_default();
                        let max = min;
                        [min, max].into()
                    }),
                    i.pointer.primary_pressed(),
                ))
            } else {
                None
            }
        });
        resource_res.pointer_was_outside =
            intersected.is_some_and(|(outside, clicked)| outside && clicked);
        intersected.is_some_and(|(outside, _)| !outside)
    } else {
        false
    };

    if let Some(window_res) = window_res {
        ui_state.add_blur_rect(window_res.response.rect, 0.0);
    }

    resource_res
}
