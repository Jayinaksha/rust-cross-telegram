use std::sync::OnceLock;

use core::{render_ui, AppState};
use eframe::egui;

use winit::platform::android::activity::input::{TextInputState, TextSpan};
use winit::platform::android::activity::AndroidApp;
use winit::platform::android::EventLoopBuilderExtAndroid;

static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

pub struct App {
    state: AppState,
    keyboard_shown: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            keyboard_shown: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Before rendering: if keyboard is active, read what the user typed
        // from GameActivity's TextInputState and sync it into our AppState.
        if self.keyboard_shown {
            if let Some(app) = ANDROID_APP.get() {
                let android_state = app.text_input_state();
                if android_state.text != self.state.message {
                    self.state.message = android_state.text;
                    self.state.text_changed_externally = true;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            render_ui(ui, &mut self.state);
        });

        // Show/hide the soft keyboard based on text field focus
        if let Some(app) = ANDROID_APP.get() {
            if self.state.text_field_focused && !self.keyboard_shown {
                // Tell GameActivity what the current text is before showing keyboard
                let len = self.state.message.len();
                app.set_text_input_state(TextInputState {
                    text: self.state.message.clone(),
                    selection: TextSpan {
                        start: len,
                        end: len,
                    },
                    compose_region: None,
                });
                app.show_soft_input(true);
                self.keyboard_shown = true;
            } else if !self.state.text_field_focused && self.keyboard_shown {
                app.hide_soft_input(false);
                self.keyboard_shown = false;
            }
        }

        // Keep repainting while keyboard is active to pick up new input
        if self.keyboard_shown {
            ctx.request_repaint();
        }
    }
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let _ = ANDROID_APP.set(app.clone());

    let options = eframe::NativeOptions {
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Telegram App",
        options,
        Box::new(|_| Box::new(App::default())),
    )
    .expect("failed to start eframe");
}
