use blue_engine_utilities::egui::egui::{self, Widget};

use crate::utils::{temp_ui_style, COLOR_SURFACE_200};

pub struct Toolbar {
    pub window_action: Option<WindowActions>,
    pub maximized: bool,
    pub command: String,
}
impl Toolbar {
    pub fn new() -> Self {
        Self {
            window_action: None,
            maximized: false,
            command: String::new(),
        }
    }

    pub fn update(&mut self, window: &mut blue_engine::Window, input: &blue_engine::InputHelper) {
        if let Some(window_action) = self.window_action.take() {
            match window_action {
                WindowActions::Exit => {
                    std::process::exit(0);
                }

                WindowActions::Minimize => {
                    window.set_minimized(true);
                }

                WindowActions::Drag => {
                    if let Err(msg) = window.drag_window() {
                        println!("{}", msg);
                    }
                }

                WindowActions::Maximize => {
                    window.set_maximized(!window.is_maximized());
                    self.maximized = !self.maximized;
                }
            }

            self.window_action = None;
        }

        let mouse_pos = input.cursor();
        if mouse_pos.is_some() {
            let mouse_pos = mouse_pos.unwrap();
            let check_zone = 5;

            let resize_function =
                |cursor_icon: blue_engine::winit::window::CursorIcon,
                 resize_direction: blue_engine::winit::window::ResizeDirection| {
                    window.set_cursor_icon(cursor_icon);

                    if input.mouse_pressed(0) {
                        window.set_cursor_icon(blue_engine::winit::window::CursorIcon::Move);

                        let _ = window.drag_resize_window(resize_direction);
                    }
                };

            // corners
            if mouse_pos.0 < check_zone as f32 && mouse_pos.1 < check_zone as f32 {
                resize_function(
                    blue_engine::winit::window::CursorIcon::NwResize,
                    blue_engine::winit::window::ResizeDirection::NorthWest,
                );
            } else if mouse_pos.0 < check_zone as f32
                && mouse_pos.1 > (window.inner_size().height - check_zone) as f32
            {
                resize_function(
                    blue_engine::winit::window::CursorIcon::SwResize,
                    blue_engine::winit::window::ResizeDirection::SouthWest,
                );
            } else if mouse_pos.0 > (window.inner_size().width - check_zone) as f32
                && mouse_pos.1 < check_zone as f32
            {
                resize_function(
                    blue_engine::winit::window::CursorIcon::NeResize,
                    blue_engine::winit::window::ResizeDirection::NorthEast,
                );
            } else if mouse_pos.0 > (window.inner_size().width - check_zone) as f32
                && mouse_pos.1 > (window.inner_size().height - check_zone) as f32
            {
                resize_function(
                    blue_engine::winit::window::CursorIcon::SeResize,
                    blue_engine::winit::window::ResizeDirection::SouthEast,
                );
            }
            // edges
            else if mouse_pos.0 < check_zone as f32 {
                resize_function(
                    blue_engine::winit::window::CursorIcon::WResize,
                    blue_engine::winit::window::ResizeDirection::West,
                );
            } else if mouse_pos.0 > (window.inner_size().width - check_zone) as f32 {
                resize_function(
                    blue_engine::winit::window::CursorIcon::EResize,
                    blue_engine::winit::window::ResizeDirection::East,
                );
            } else if mouse_pos.1 < check_zone as f32 {
                resize_function(
                    blue_engine::winit::window::CursorIcon::NResize,
                    blue_engine::winit::window::ResizeDirection::North,
                );
            } else if mouse_pos.1 > (window.inner_size().height - check_zone) as f32 {
                resize_function(
                    blue_engine::winit::window::CursorIcon::SResize,
                    blue_engine::winit::window::ResizeDirection::South,
                );
            } else {
                window.set_cursor_icon(blue_engine::winit::window::CursorIcon::Default);
            }
        }
    }

    pub fn display(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar")
            .resizable(false)
            .show(ctx, |ui| {
                let (toolbar, toolbar_response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                ui.allocate_ui_at_rect(toolbar, |ui| {
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            ui.style_mut().visuals.extreme_bg_color =
                                egui::Color32::from_hex(COLOR_SURFACE_200).unwrap();
                            //ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                            ui.style_mut().visuals.widgets.hovered.expansion = 0f32;
                            ui.style_mut().visuals.interact_cursor = Some(egui::CursorIcon::Text);

                            egui::TextEdit::singleline(&mut self.command)
                                .hint_text("Command...")
                                .desired_width(100f32)
                                .ui(ui);
                        });

                        let mut toolbar_actions_rect = ui.min_rect();
                        toolbar_actions_rect.set_width(ui.spacing().interact_size.x * 3f32);
                        let distance = toolbar_actions_rect
                            .distance_to_pos(egui::pos2(ctx.screen_rect().width() - 1f32, 0f32));
                        toolbar_actions_rect =
                            toolbar_actions_rect.translate(egui::vec2(distance, 0f32));

                        ui.allocate_ui_at_rect(toolbar_actions_rect, |ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::default()),
                                |ui| {
                                    let mut new_style = egui::Style::default();
                                    new_style.visuals.widgets.inactive.weak_bg_fill =
                                        egui::Color32::TRANSPARENT;
                                    new_style.visuals.widgets.hovered.bg_stroke =
                                        egui::Stroke::NONE;

                                    let mut exit_style = new_style.clone();
                                    exit_style.visuals.widgets.hovered.weak_bg_fill =
                                        egui::Color32::from_rgb(150, 0, 0);

                                    temp_ui_style(ui, exit_style, |ui| {
                                        if egui::ImageButton::new(egui::include_image!(
                                            "../../resources/icons/exit.png"
                                        ))
                                        .tint(egui::Color32::GRAY)
                                        .ui(ui)
                                        .clicked()
                                        {
                                            std::process::exit(0);
                                        }
                                    });

                                    temp_ui_style(ui, new_style, |ui| {
                                        if egui::ImageButton::new(if self.maximized {
                                            egui::include_image!(
                                                "../../resources/icons/maximize_exit.png"
                                            )
                                        } else {
                                            egui::include_image!(
                                                "../../resources/icons/maximize.png"
                                            )
                                        })
                                        .tint(egui::Color32::GRAY)
                                        .ui(ui)
                                        .clicked()
                                        {
                                            self.window_action = Some(WindowActions::Maximize);
                                        }

                                        if egui::ImageButton::new(egui::include_image!(
                                            "../../resources/icons/minimize.png"
                                        ))
                                        .tint(egui::Color32::GRAY)
                                        .ui(ui)
                                        .clicked()
                                        {
                                            self.window_action = Some(WindowActions::Minimize);
                                        }
                                    });
                                },
                            );
                        });
                    });
                });

                if toolbar_response.dragged() {
                    self.window_action = Some(WindowActions::Drag);
                } else if toolbar_response.drag_released() {
                    self.window_action = None;
                }
            });
    }
}

#[allow(dead_code)]
pub enum WindowActions {
    Maximize,
    Minimize,
    Drag,
    Exit,
}
