use serde::Deserialize;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Shared types for incoming messages
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// A message received from the Telegram `getUpdates` API.
#[derive(Clone, Debug)]
pub struct IncomingMessage {
    pub from_name: String,
    pub text: String,
    pub date: i64,
}

// ── Serde types for parsing getUpdates JSON response ───────────────

#[derive(Deserialize, Debug)]
struct GetUpdatesResponse {
    ok: bool,
    #[serde(default)]
    result: Vec<Update>,
}

#[derive(Deserialize, Debug)]
struct Update {
    update_id: i64,
    message: Option<TgMessage>,
}

#[derive(Deserialize, Debug)]
struct TgMessage {
    text: Option<String>,
    from: Option<TgUser>,
    date: Option<i64>,
}

#[derive(Deserialize, Debug)]
struct TgUser {
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
}

impl TgUser {
    fn display_name(&self) -> String {
        match (&self.first_name, &self.last_name, &self.username) {
            (Some(f), Some(l), _) => format!("{} {}", f, l),
            (Some(f), None, _) => f.clone(),
            (None, None, Some(u)) => format!("@{}", u),
            _ => "Unknown".to_string(),
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  NATIVE VERSION (Linux / Android / macOS / Windows)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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

/// Calls `getUpdates` with long-polling.
/// Returns the list of new messages and the next offset to use.
#[cfg(not(target_arch = "wasm32"))]
pub async fn get_updates(
    token: &str,
    offset: i64,
    timeout: u64,
) -> Result<(Vec<IncomingMessage>, i64), reqwest::Error> {
    let url = format!(
        "https://api.telegram.org/bot{}/getUpdates",
        token
    );

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout + 5))
        .build()?;

    let params = [
        ("offset", offset.to_string()),
        ("timeout", timeout.to_string()),
        ("allowed_updates", r#"["message"]"#.to_string()),
    ];

    let resp = client
        .get(&url)
        .query(&params)
        .send()
        .await?
        .json::<GetUpdatesResponse>()
        .await?;

    let mut messages = Vec::new();
    let mut next_offset = offset;

    if resp.ok {
        for update in &resp.result {
            if update.update_id >= next_offset {
                next_offset = update.update_id + 1;
            }
            if let Some(ref msg) = update.message {
                if let Some(ref text) = msg.text {
                    let from_name = msg
                        .from
                        .as_ref()
                        .map(|u| u.display_name())
                        .unwrap_or_else(|| "Unknown".to_string());
                    messages.push(IncomingMessage {
                        from_name,
                        text: text.clone(),
                        date: msg.date.unwrap_or(0),
                    });
                }
            }
        }
    }

    Ok((messages, next_offset))
}


// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  WASM VERSION (Browser)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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

/// Calls `getUpdates` for WASM.
/// Uses short timeout (0) since we can't do real long-polling easily in WASM fetch.
#[cfg(target_arch = "wasm32")]
pub async fn get_updates(
    token: &str,
    offset: i64,
    _timeout: u64,
) -> Result<(Vec<IncomingMessage>, i64), JsValue> {
    let url = format!(
        "https://api.telegram.org/bot{}/getUpdates?offset={}&timeout=0&allowed_updates=%5B%22message%22%5D",
        token, offset
    );

    let opts = RequestInit::new();
    opts.set_method("GET");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_val = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_val.into();

    let json_val = JsFuture::from(resp.json()?).await?;
    let parsed: GetUpdatesResponse = serde_wasm_bindgen::from_value(json_val)
        .map_err(|e| JsValue::from_str(&format!("parse error: {}", e)))?;

    let mut messages = Vec::new();
    let mut next_offset = offset;

    if parsed.ok {
        for update in &parsed.result {
            if update.update_id >= next_offset {
                next_offset = update.update_id + 1;
            }
            if let Some(ref msg) = update.message {
                if let Some(ref text) = msg.text {
                    let from_name = msg
                        .from
                        .as_ref()
                        .map(|u| u.display_name())
                        .unwrap_or_else(|| "Unknown".to_string());
                    messages.push(IncomingMessage {
                        from_name,
                        text: text.clone(),
                        date: msg.date.unwrap_or(0),
                    });
                }
            }
        }
    }

    Ok((messages, next_offset))
}
