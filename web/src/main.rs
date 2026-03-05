use core::{AppState, render_ui};
use eframe::egui;

struct App {
    state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_ui(ui, &mut self.state);
        });
    }

}

fn main() {

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions {
    follow_system_theme: true,
    ..Default::default()
};

    wasm_bindgen_futures::spawn_local(async {

        eframe::WebRunner::new()
            .start(
                "app",
                web_options,
                Box::new(|_| Box::new(App::default())),
            )
            .await
            .expect("failed to start");

    });

}