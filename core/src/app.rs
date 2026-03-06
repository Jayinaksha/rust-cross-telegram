use crate::telegram;
use egui::text::{CCursor, CCursorRange};
use egui::{TextEdit, Ui};

pub struct AppState {
    pub message: String,
    pub token: String,
    pub chat_id: String,
    pub text_field_focused: bool,
    /// Set by platform wrappers when message was updated externally (e.g. Android IME).
    /// render_ui will move the cursor to the end and clear this flag.
    pub text_changed_externally: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            message: String::new(),
            token: "7772114599:AAEgZnG16-dAaY1_K8ag5zckjD8nLllu-J8".to_string(),
            chat_id: "1751201457".to_string(),
            text_field_focused: false,
            text_changed_externally: false,
        }
    }
}

pub fn render_ui(ui: &mut Ui, state: &mut AppState) {
    ui.vertical_centered(|ui| {
        ui.heading("Rust Telegram Messenger");

        ui.add_space(10.0);

        let output = TextEdit::singleline(&mut state.message).show(ui);
        let response = &output.response;

        // If text was changed externally (Android IME), move cursor to end
        if state.text_changed_externally {
            let len = state.message.len();
            let ccursor = CCursor::new(len);
            let mut edit_state = output.state.clone();
            edit_state
                .cursor
                .set_char_range(Some(CCursorRange::one(ccursor)));
            edit_state.store(ui.ctx(), response.id);
            state.text_changed_externally = false;
        }

        let was_focused = state.text_field_focused;
        state.text_field_focused = response.has_focus();

        if response.has_focus() {
            ui.ctx().request_repaint();
        }

        if ui.button("Send").clicked() {
            let msg = state.message.clone();
            let token = state.token.clone();
            let chat = state.chat_id.clone();

            // Native platforms (Linux / Android)
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = was_focused; // suppress unused warning
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        let _ = telegram::send_message(&token, &chat, &msg).await;
                    });
                });
            }

            // Browser (WASM)
            #[cfg(target_arch = "wasm32")]
            {
                let _ = was_focused;
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = telegram::send_message(&token, &chat, &msg).await;
                });
            }
        }
    });
}