use crate::utils::Page;
use blue_engine_utilities::egui::egui::{self, Widget};

mod customers_page;
mod dashboard_page;

pub struct Dashboard {
    selected_page: usize,

    // pages
    dashboard_page: dashboard_page::DashboardPage,
    customers_page: customers_page::CustomersPage,
}
impl Dashboard {
    pub fn menu_button(
        &mut self,
        ui: &mut egui::Ui,
        text: impl Into<String>,
        image_source: egui::ImageSource<'_>,
        id: usize,
    ) {
        ui.horizontal(|ui| {
            let image = egui::Image::new(image_source)
                .max_size(egui::vec2(13f32, 13f32))
                .ui(ui);

            let mut text = egui::RichText::new(text).size(10.5f32);
            if self.selected_page == id {
                text = text.strong();
            }

            ui.style_mut().visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);

            let label = egui::Button::new(text).frame(false).small().ui(ui);
            if image.clicked() || label.clicked() {
                self.selected_page = id;
            }
        });

        ui.add_space(5f32);
    }
}
impl Page for Dashboard {
    fn new(_engine: &mut blue_engine::Engine) -> Self {
        Self {
            selected_page: 0,

            // pages
            dashboard_page: dashboard_page::DashboardPage::new(_engine),
            customers_page: customers_page::CustomersPage::new(_engine),
        }
    }

    fn update(
        &mut self,
        _renderer: &mut blue_engine::Renderer,
        _window: &mut blue_engine::Window,
        _objects: &mut blue_engine::ObjectStorage,
        _input: &blue_engine::InputHelper,
        _camera: &mut blue_engine::Camera,
        _signals: &mut blue_engine::SignalStorage,
    ) {
        self.dashboard_page
            .update(_renderer, _window, _objects, _input, _camera, _signals);
        self.customers_page
            .update(_renderer, _window, _objects, _input, _camera, _signals);
    }

    fn display(&mut self, ctx: &blue_engine_utilities::egui::egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(100f32)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10f32);
                    ui.label(
                        egui::RichText::new("ArkForge")
                            .color(egui::Color32::from_hex("#4090f3").unwrap())
                            .font(egui::FontId::new(
                                20f32,
                                egui::FontFamily::Name("InterBold".into()),
                            ))
                            .size(20f32),
                    );
                    ui.add_space(20f32);

                    self.menu_button(
                        ui,
                        "Dashboard",
                        egui::include_image!("../../resources/icons/dashboard.png"),
                        0,
                    );

                    self.menu_button(
                        ui,
                        "Customers",
                        egui::include_image!("../../resources/icons/customers.png"),
                        1,
                    );
                });
            });

        match self.selected_page {
            0 => self.dashboard_page.display(ctx),
            1 => self.customers_page.display(ctx),
            _ => {}
        }
    }
}
