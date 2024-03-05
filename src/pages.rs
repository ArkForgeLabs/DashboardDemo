use crate::{
    utils::{global_style, import_fonts, Page},
    widgets::toolbar::Toolbar,
};
use blue_engine::{Camera, Engine, InputHelper, ObjectStorage, Renderer, SignalStorage, Window};

mod dashboard;
mod login;
pub struct Pages {
    pub index: usize,

    // pages
    pub login: login::Login,
    pub dashboard: dashboard::Dashboard,

    pub toolbar: Toolbar,
}
impl Pages {
    pub fn new(engine: &mut Engine) -> Self {
        let egui_context = engine
            .signals
            .get_signal::<blue_engine_utilities::egui::EGUI>("egui")
            .expect("Failed to get egui signal")
            .expect("Failed to downcast egui signal");
        import_fonts(&egui_context.context);
        egui_extras::install_image_loaders(&egui_context.context);
        egui_context.context.set_style(global_style());

        Self {
            index: 1,
            login: login::Login::new(engine),
            dashboard: dashboard::Dashboard::new(engine),

            toolbar: Toolbar::new(),
        }
    }

    pub fn update_loop(
        &mut self,
        renderer: &mut Renderer,
        window: &mut Window,
        objects: &mut ObjectStorage,
        input: &InputHelper,
        camera: &mut Camera,
        signals: &mut SignalStorage,
    ) {
        let Self {
            index,
            login,
            dashboard,
            toolbar,
        } = self;

        // page updates
        login.update(renderer, window, objects, input, camera, signals);
        dashboard.update(renderer, window, objects, input, camera, signals);
        toolbar.update(window, input);

        // egui
        let egui_context = signals
            .get_signal::<blue_engine_utilities::egui::EGUI>("egui")
            .expect("Failed to get egui signal")
            .expect("Failed to downcast egui signal");

        egui_context.ui(
            move |ctx| {
                match index {
                    0 => login.display(ctx),
                    1 => dashboard.display(ctx),
                    _ => {}
                }

                toolbar.display(ctx);
            },
            window,
        );
    }
}
