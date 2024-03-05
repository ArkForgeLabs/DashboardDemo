use blue_engine::{Camera, InputHelper, ObjectStorage, Renderer, SignalStorage, Window};
use blue_engine_utilities::egui::egui::{self};

/** DARK THEME PRIMARY COLORS */
#[allow(dead_code)]
pub const COLOR_PRIMARY_100: &'static str = "#03a9f4";
#[allow(dead_code)]
pub const COLOR_PRIMARY_200: &'static str = "#4ab2f5";
#[allow(dead_code)]
pub const COLOR_PRIMARY_300: &'static str = "#6abcf7";
#[allow(dead_code)]
pub const COLOR_PRIMARY_400: &'static str = "#84c5f8";
#[allow(dead_code)]
pub const COLOR_PRIMARY_500: &'static str = "#9bcefa";
#[allow(dead_code)]
pub const COLOR_PRIMARY_600: &'static str = "#b1d8fb";
/** DARK THEME SURFACE COLORS */
#[allow(dead_code)]
pub const COLOR_SURFACE_100: &'static str = "#121212";
#[allow(dead_code)]
pub const COLOR_SURFACE_200: &'static str = "#282828";
#[allow(dead_code)]
pub const COLOR_SURFACE_300: &'static str = "#3f3f3f";
#[allow(dead_code)]
pub const COLOR_SURFACE_400: &'static str = "#575757";
#[allow(dead_code)]
pub const COLOR_SURFACE_500: &'static str = "#717171";
#[allow(dead_code)]
pub const COLOR_SURFACE_600: &'static str = "#8b8b8b";

pub trait Page {
    fn new(engine: &mut blue_engine::Engine) -> Self;

    fn update(
        &mut self,
        _renderer: &mut Renderer,
        _window: &mut Window,
        _objects: &mut ObjectStorage,
        _input: &InputHelper,
        _camera: &mut Camera,
        _signals: &mut SignalStorage,
    ) {
    }

    fn display(&mut self, context: &egui::Context);
}

pub fn global_style() -> egui::Style {
    let mut style = egui::Style::default();
    style.visuals.panel_fill = egui::Color32::from_hex(COLOR_SURFACE_100).unwrap();
    style.visuals.window_fill = egui::Color32::from_hex(COLOR_SURFACE_200).unwrap();
    style.visuals.window_stroke =
        egui::Stroke::new(0.0, egui::Color32::from_hex(COLOR_SURFACE_200).unwrap());

    style
}

#[allow(dead_code)]
pub fn temp_ui_style(
    ui: &mut egui::Ui,
    new_style: egui::Style,
    callback: impl FnOnce(&mut egui::Ui),
) {
    let old_style = ui.style().clone();
    ui.set_style(new_style);
    callback(ui);
    ui.set_style(old_style);
}

#[allow(dead_code)]
pub fn temp_ctx_style(
    ctx: &mut egui::Context,
    new_style: egui::Style,
    callback: impl FnOnce(&mut egui::Context),
) {
    let old_style = ctx.style().clone();
    ctx.set_style(new_style);
    callback(ctx);
    ctx.set_style(old_style);
}

pub type WidgetSizeStorage = std::collections::HashMap<String, egui::Vec2>;

pub fn import_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Inter".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../resources/fonts/Inter/Inter-VariableFont_slnt,wght.ttf"
        )),
    );
    fonts.font_data.insert(
        "InterBold".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../resources/fonts/Inter/static/Inter-ExtraBold.ttf"
        )),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Inter".to_owned());
    fonts.families.insert(
        egui::FontFamily::Name("InterBold".into()),
        vec!["InterBold".to_owned()],
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("Inter".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("InterBold".to_owned());

    ctx.set_fonts(fonts);
}
