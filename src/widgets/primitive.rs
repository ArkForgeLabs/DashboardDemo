use blue_engine_utilities::egui::egui::{self, Widget};

use crate::utils::{COLOR_PRIMARY_400, COLOR_PRIMARY_500};

pub fn button(
    ui: &mut egui::Ui,
    text: impl Into<egui::WidgetText>,
    size: (f32, f32),
) -> egui::Response {
    ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
        egui::Color32::from_hex(COLOR_PRIMARY_500).unwrap();
    ui.style_mut().visuals.widgets.inactive.bg_stroke =
        egui::Stroke::new(0f32, egui::Color32::TRANSPARENT);
    ui.style_mut().visuals.widgets.inactive.fg_stroke =
        egui::Stroke::new(3f32, egui::Color32::BLACK);

    ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
        egui::Color32::from_hex(COLOR_PRIMARY_400).unwrap();
    ui.style_mut().visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(0f32, egui::Color32::TRANSPARENT);
    ui.style_mut().visuals.widgets.hovered.fg_stroke =
        egui::Stroke::new(3f32, egui::Color32::BLACK);
    ui.style_mut().visuals.widgets.hovered.expansion =
        ui.style_mut().visuals.widgets.inactive.expansion;

    ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::TRANSPARENT;
    ui.style_mut().visuals.widgets.active.bg_stroke =
        egui::Stroke::new(1f32, egui::Color32::from_hex(COLOR_PRIMARY_500).unwrap());

    ui.style_mut().visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);

    egui::Button::new(text)
        .min_size(egui::vec2(size.0, size.1))
        .ui(ui)
}
