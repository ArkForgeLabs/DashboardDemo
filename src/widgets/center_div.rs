use blue_engine::StringBuffer;
use blue_engine_utilities::egui::egui::{self};
use blue_engine_utilities::egui::egui::{Response, Ui};

use crate::utils::WidgetSizeStorage;
/// A custom widget that acts like a div from HTML and centers its content both horizontally and vertically.

pub fn center_div_horizontally(
    name: impl StringBuffer,
    widget_size_storage: &mut WidgetSizeStorage,
    ui: &mut Ui,
    content: impl FnOnce(&mut Ui) -> egui::Vec2,
) -> Response {
    let (mut rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ui.style().spacing.interact_size.y),
        egui::Sense::focusable_noninteractive(),
    );

    if let Some(width_difference) = widget_size_storage.get(&name.as_string()) {
        rect.set_width(width_difference.x);
        rect = rect.translate(egui::vec2(
            (ui.available_width() - rect.width()) / 2.0,
            0f32,
        ));
    }

    let allocation = ui.allocate_ui_at_rect(rect, content);
    widget_size_storage.insert(name.as_string(), allocation.inner);

    response
}
