use blue_engine_utilities::egui::egui;

use crate::{
    utils::{WidgetSizeStorage, COLOR_SURFACE_300},
    widgets::{center_div::center_div_horizontally, primitive::button},
};

pub struct Login {
    element_sizes: WidgetSizeStorage,
    password_hash: String,
    password_visible: bool,
    password_focus: bool,
    remember_me: bool,
}
impl crate::utils::Page for Login {
    fn new(_engine: &mut blue_engine::Engine) -> Self {
        Self {
            element_sizes: WidgetSizeStorage::new(),
            password_hash: String::new(),
            password_visible: true,
            password_focus: false,
            remember_me: false,
        }
    }

    fn display(&mut self, ctx: &blue_engine_utilities::egui::egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let window_width = if ui.available_width() > 300.0 {
                300.0
            } else {
                ui.available_width()
            };

            egui::Window::new("login")
                .title_bar(false)
                .resizable(false)
                .constrain(true)
                .max_width(window_width)
                .min_width(window_width)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10f32);
                        ui.label("Welcome back to Nexus!");
                        ui.heading(
                            egui::RichText::new("Log into your account")
                                .color(egui::Color32::WHITE),
                        );
                        ui.add_space(10f32);

                        ui.horizontal(|ui| {
                            ui.add_space(10f32);
                            ui.label(egui::RichText::new("Password:").color(egui::Color32::WHITE));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::default()),
                                |ui| {
                                    ui.style_mut().visuals.button_frame = false;
                                    ui.style_mut().visuals.interact_cursor =
                                        Some(egui::CursorIcon::PointingHand);
                                    ui.add_space(10f32);
                                    if ui.button("👁").clicked() {
                                        self.password_visible = !self.password_visible;
                                    }
                                },
                            );
                        });

                        center_div_horizontally("password", &mut self.element_sizes, ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.style_mut().visuals.extreme_bg_color =
                                    egui::Color32::from_hex(COLOR_SURFACE_300).unwrap();
                                let password_input =
                                    egui::TextEdit::singleline(&mut self.password_hash)
                                        .password(self.password_visible)
                                        .desired_width(275f32)
                                        .show(ui)
                                        .response;

                                if !self.password_focus {
                                    password_input.request_focus();
                                    self.password_focus = true;
                                }

                                if password_input.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    println!("Password: {}", self.password_hash);
                                }
                            })
                            .response
                            .rect
                            .size()
                        });

                        ui.add_space(10f32);

                        if button(ui, "Connect", (285f32, 25f32)).clicked() {
                            println!("Password: {}", self.password_hash);
                        }

                        ui.horizontal(|ui| {
                            ui.add_space(7.5f32);
                            ui.style_mut().visuals.override_text_color = Some(egui::Color32::GRAY);
                            ui.checkbox(&mut self.remember_me, "Remember me");
                        });

                        ui.add_space(10f32);
                    });
                });
        });
    }
}
