use crate::utils::Page;
use blue_engine_utilities::egui::egui;

pub struct CustomersPage {}
impl Page for CustomersPage {
    fn new(_engine: &mut blue_engine::Engine) -> Self {
        Self {}
    }

    fn display(&mut self, context: &blue_engine_utilities::egui::egui::Context) {
        egui::Window::new("CustomersPage")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(context, |ui| {
                ui.label("Hello, customers!");
            });
    }
}
