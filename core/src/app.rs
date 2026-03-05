use crate::telegram;
use egui::Ui;

pub struct AppState {
    pub message: String,
    pub token: String,
    pub chat_id: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            message: String::new(),
            token: "7772114599:AAEgZnG16-dAaY1_K8ag5zckjD8nLllu-J8".to_string(),
            chat_id: "1751201457".to_string(),
        }
    }
}

pub fn render_ui(ui: &mut Ui, state: &mut AppState) {

    ui.vertical_centered(|ui| {

        ui.heading("Rust Telegram Messenger");

        ui.add_space(10.0);

        ui.text_edit_singleline(&mut state.message);

        if ui.button("Send").clicked() {

            let msg = state.message.clone();
            let token = state.token.clone();
            let chat = state.chat_id.clone();

            // Native platforms (Linux / Android)
            #[cfg(not(target_arch = "wasm32"))]
            {
                std::thread::spawn(move || {

                    let rt = tokio::runtime::Runtime::new().unwrap();

                    rt.block_on(async {

                        let _ = telegram::send_message(
                            &token,
                            &chat,
                            &msg
                        ).await;

                    });

                });
            }

            // Browser (WASM)
            #[cfg(target_arch = "wasm32")]
            {
                wasm_bindgen_futures::spawn_local(async move {

                    let _ = telegram::send_message(
                        &token,
                        &chat,
                        &msg
                    ).await;

                });
            }

        }

    });

}