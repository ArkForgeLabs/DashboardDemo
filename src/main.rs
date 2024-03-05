mod pages;
mod utils;
mod widgets;

fn main() {
    let mut engine = blue_engine::Engine::new_config(blue_engine::WindowDescriptor {
        width: 1280,
        height: 720,
        present_mode: blue_engine::wgpu::PresentMode::AutoVsync,
        decorations: false,
        ..Default::default()
    })
    .expect("Failed to create engine");

    let egui_signal = blue_engine_utilities::egui::EGUI::new(
        &engine.event_loop,
        &mut engine.renderer,
        &mut engine.window,
    );
    engine.signals.add_signal("egui", Box::new(egui_signal));

    let mut pages = pages::Pages::new(&mut engine);

    engine
        .update_loop(move |renderer, window, objects, input, camera, signals| {
            pages.update_loop(renderer, window, objects, input, camera, signals);
        })
        .expect("Failed to run update loop");
}
