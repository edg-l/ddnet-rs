use egui::{vec2, Button, FontId, Layout, Margin, Response, Rounding, Stroke, TextEdit};
use egui_extras::{Size, StripBuilder};

pub fn clearable_edit_field(
    ui: &mut egui::Ui,
    text: &mut String,
    input_at_most_size: Option<f32>,
    max_chars: Option<usize>,
) -> Option<Response> {
    let address = text;
    let style = ui.style_mut();
    let rounding = &style.visuals.widgets.inactive.rounding;
    let rounding = rounding.ne.max(rounding.nw);
    style.spacing.item_spacing = vec2(0.0, 0.0);
    let mut res = None;
    ui.horizontal(|ui| {
        StripBuilder::new(ui)
            .size(if let Some(input_at_most_size) = input_at_most_size {
                Size::remainder().at_most(input_at_most_size)
            } else {
                Size::remainder()
            })
            .size(Size::exact(20.0))
            .clip(true)
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    ui.style_mut().wrap_mode = None;
                    res = Some(
                        ui.with_layout(
                            Layout::left_to_right(egui::Align::Center)
                                .with_main_justify(true)
                                .with_cross_justify(true),
                            |ui| {
                                let style = ui.style_mut();
                                style.visuals.selection.stroke = Stroke::NONE;
                                let widgets = &mut style.visuals.widgets;
                                widgets.inactive.rounding = Rounding {
                                    nw: rounding,
                                    sw: rounding,
                                    ..Default::default()
                                };
                                widgets.inactive.expansion = 0.0;
                                widgets.inactive.bg_stroke = Stroke::NONE;
                                widgets.active.rounding = widgets.inactive.rounding;
                                widgets.active.expansion = widgets.inactive.expansion;
                                widgets.active.bg_stroke = widgets.inactive.bg_stroke;
                                widgets.hovered.rounding = widgets.inactive.rounding;
                                widgets.hovered.expansion = widgets.inactive.expansion;
                                widgets.hovered.bg_stroke = widgets.inactive.bg_stroke;
                                widgets.noninteractive.rounding = widgets.inactive.rounding;
                                widgets.noninteractive.expansion = widgets.inactive.expansion;
                                widgets.noninteractive.bg_stroke = widgets.inactive.bg_stroke;
                                widgets.open.rounding = widgets.inactive.rounding;
                                widgets.open.expansion = widgets.inactive.expansion;
                                widgets.open.bg_stroke = widgets.inactive.bg_stroke;
                                ui.add(
                                    TextEdit::singleline(address)
                                        .margin(Margin {
                                            left: 3.0,
                                            right: 3.0,
                                            top: 3.0,
                                            ..Margin::ZERO
                                        })
                                        .font(FontId::proportional(10.0))
                                        .char_limit(max_chars.unwrap_or(usize::MAX).max(1)),
                                )
                            },
                        )
                        .inner,
                    );
                });
                strip.cell(|ui| {
                    ui.style_mut().wrap_mode = None;
                    let style = ui.style_mut();
                    let widgets = &mut style.visuals.widgets;
                    widgets.inactive.rounding = Rounding {
                        ne: rounding,
                        se: rounding,
                        ..Default::default()
                    };
                    widgets.active.rounding = widgets.inactive.rounding;
                    widgets.active.expansion = widgets.inactive.expansion;
                    widgets.hovered.rounding = widgets.inactive.rounding;
                    widgets.hovered.expansion = widgets.inactive.expansion;
                    widgets.noninteractive.rounding = widgets.inactive.rounding;
                    widgets.noninteractive.expansion = widgets.inactive.expansion;
                    widgets.open.rounding = widgets.inactive.rounding;
                    widgets.noninteractive.expansion = widgets.inactive.expansion;
                    if ui
                        .add(Button::new("\u{f00d}").stroke(Stroke::NONE))
                        .clicked()
                    {
                        address.clear();
                    }
                });
            });
    });
    res
}
