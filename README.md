# rust-cross-telegram

A cross-platform Telegram messenger built with Rust and [egui](https://github.com/emilk/egui)/[eframe](https://github.com/emilk/egui/tree/master/crates/eframe). Send Telegram messages from a single shared codebase that targets desktop (Linux/macOS/Windows), Android, and the web (WebAssembly).

## Project Structure

```
rust-cross-telegram/
├── core/       # Shared application logic (UI + Telegram Bot API client)
├── desktop/    # Native desktop entry point (Linux / macOS / Windows)
├── android/    # Android native-activity entry point
└── web/        # WebAssembly entry point (runs in the browser via Trunk)
```

| Crate   | Description |
|---------|-------------|
| `core`  | Platform-agnostic UI rendered with egui and HTTP client using `reqwest` (native) or `web-sys` fetch (WASM) |
| `desktop` | Thin wrapper that boots `eframe` as a native window |
| `android` | `cdylib` that initialises the Android logger and runs `eframe` via `android-native-activity` |
| `web`   | WASM binary wired up to an HTML canvas; served with [Trunk](https://trunkrs.dev/) |

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

### Desktop

No extra steps required — the default Rust toolchain is sufficient.

### Web

```bash
# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Android

```bash
# Add Android targets (choose the ones you need)
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Install cargo-ndk
cargo install cargo-ndk
```

You also need the Android NDK installed and `ANDROID_NDK_HOME` set in your environment.

## Configuration

The bot token and chat ID are read at startup. **Do not hard-code credentials in source files.** Instead, supply them via environment variables and read them in `core/src/app.rs`:

```rust
token: std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default(),
chat_id: std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default(),
```

Set the variables in your shell before running:

```bash
export TELEGRAM_BOT_TOKEN="<your_bot_token>"
export TELEGRAM_CHAT_ID="<your_chat_id>"
```

You can obtain a bot token from [@BotFather](https://t.me/BotFather) on Telegram.

> **Security note:** Never commit tokens or chat IDs to version control. Add any local `.env` files to `.gitignore`.

## Building & Running

### Desktop

```bash
cargo run -p desktop
```

### Web

```bash
cd web
trunk serve
```

Then open `http://localhost:8080` in your browser.

### Android

```bash
cargo ndk -t aarch64-linux-android build -p android
```

Package and deploy the resulting shared library using the standard Android build toolchain.

## Usage

1. Launch the app on your target platform.
2. Type your message in the text field.
3. Press **Send** — the message is delivered to the configured Telegram chat via the Bot API.

## License

No license file is currently present in this repository. All rights are reserved by the repository owner until an explicit license is added. If you intend to use or contribute to this project, please open an issue to request a license.
