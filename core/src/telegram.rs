#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit};


// ===============================
// NATIVE VERSION (Linux / Android)
// ===============================

#[cfg(not(target_arch = "wasm32"))]
pub async fn send_message(
    token: &str,
    chat_id: &str,
    text: &str,
) -> Result<(), reqwest::Error> {

    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token
    );

    let client = Client::new();

    let params = [
        ("chat_id", chat_id),
        ("text", text),
    ];

    client
        .post(url)
        .form(&params)
        .send()
        .await?;

    Ok(())
}


// ===============================
// WASM VERSION (Browser)
// ===============================

#[cfg(target_arch = "wasm32")]
pub async fn send_message(
    token: &str,
    chat_id: &str,
    text: &str,
) -> Result<(), JsValue> {

    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        token
    );

    let body = serde_json::json!({
        "chat_id": chat_id,
        "text": text,
    });

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&body.to_string()));

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();

    JsFuture::from(
        window.fetch_with_request(&request)
    ).await?;

    Ok(())
}