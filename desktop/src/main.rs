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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Telegram App",
        options,
        Box::new(|_| Box::new(App::default())),
    )
    .unwrap();

}