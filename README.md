# rust-cross-telegram

A cross-platform Telegram messenger built with Rust and [egui](https://github.com/emilk/egui)/[eframe](https://github.com/emilk/egui/tree/master/crates/eframe). Send Telegram messages from a single shared codebase that targets desktop (Linux/macOS/Windows), Android, and the web (WebAssembly).

## UI Preview

The app features a polished **Material Design dark theme** with an animated particle background, a responsive sidebar navigation, and real-time bot status indicators.

| Desktop (compact window) | Desktop (full-width) |
|:---:|:---:|
| ![Rust Telegram App – Dashboard compact view](https://github.com/user-attachments/assets/b658fd93-ad08-4e4e-a9c8-346f218b37b3) | ![Rust Telegram App – Dashboard full view with Recent Activity](https://github.com/user-attachments/assets/cedc0079-6a57-4729-a521-01289ff29750) |

Key UI highlights:
- **Dark Material palette** — deep `#121218` background, purple accent (`#BB86FC`), teal secondary (`#03DAC6`)
- **Ambient particle system** — 60 gently floating purple/teal dots that fade in and out
- **Sidebar navigation** — Dashboard and Settings tabs with an active-state highlight
- **Quick Stats** — live *Messages Sent* counter in the sidebar
- **Dashboard cards** — *Messages Sent* and *Bot Status* metric cards
- **Quick Send** panel — one-line text input + Send button wired to the Telegram Bot API
- **Recent Activity** feed — every sent message appears as a timestamped entry
- **Connection indicator** — green *Connected* dot in the top-right corner

## Project Structure

```
rust-cross-telegram/
├── core/       # Shared application logic (UI + Telegram Bot API client)
│   └── src/
│       ├── app.rs          # AppState, render_ui, theme, layout
│       ├── lib.rs          # Re-exports
│       ├── telegram.rs     # Async sendMessage (native + WASM)
│       └── render/
│           ├── mod.rs
│           ├── particles.rs   # Ambient floating-dot particle system
│           ├── matrix.rs
│           └── starfield.rs
├── desktop/    # Native desktop entry point (Linux / macOS / Windows)
├── android/    # Android native-activity entry point
└── web/        # WebAssembly entry point (runs in the browser via Trunk)
```

| Crate   | Description |
|---------|-------------|
| `core`  | Platform-agnostic UI rendered with egui; HTTP client using `reqwest` (native) or `web-sys` fetch (WASM) |
| `desktop` | Thin wrapper that boots `eframe` as a native window |
| `android` | `cdylib` that initialises the Android logger and runs `eframe` via `android-native-activity` |
| `web`   | WASM binary wired up to an HTML canvas; served with [Trunk](https://trunkrs.dev/) |

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                        core crate                        │
│                                                          │
│  AppState  ──────────────►  render_ui()                  │
│  • message: String             │                         │
│  • token / chat_id             │  egui panels & widgets  │
│  • messages_sent: u32          │                         │
│  • activity_log: Vec<LogEntry> ▼                         │
│  • particles: ParticleSystem   Top bar                   │
│                                Sidebar (tabs)            │
│  telegram::send_message() ◄──  Dashboard / Settings      │
│  (async, native + WASM)        Particle background       │
└──────────────────────────────────────────────────────────┘
           │                              │
    ┌──────┘──────┐               ┌───────┘──────┐
    │   desktop   │               │     web      │
    │  eframe     │               │ eframe/WASM  │
    │  native     │               │ Trunk + HTML │
    └─────────────┘               └──────────────┘
```

### `AppState` fields

| Field | Type | Purpose |
|-------|------|---------|
| `message` | `String` | Current text in the Quick Send input |
| `token` | `String` | Telegram Bot API token |
| `chat_id` | `String` | Target Telegram chat / channel ID |
| `text_field_focused` | `bool` | Tracks keyboard focus on the input |
| `text_changed_externally` | `bool` | Forces a cursor reset when text is set programmatically |
| `selected_tab` | `Tab` | Active sidebar tab (`Dashboard` or `Settings`) |
| `messages_sent` | `u32` | Running count of successfully sent messages |
| `activity_log` | `Vec<LogEntry>` | Recent sent-message history shown in the feed |
| `particles` | `ParticleSystem` | Animated background particles |
| `time` | `f32` | Accumulated frame time used for animations |

### Color palette (`core/src/app.rs`)

| Constant | Hex | Role |
|----------|-----|------|
| `BG_BASE` | `#121218` | Window / panel background |
| `SURFACE` | `#1E1E2E` | Card / widget fill |
| `SURFACE_ALT` | `#242434` | Highlighted card fill |
| `SURFACE_HOVER` | `#2C2C3E` | Hover state fill |
| `ACCENT` | `#BB86FC` | Primary purple accent |
| `ACCENT_DIM` | `#6E4EA8` | Dimmed accent (active states) |
| `TEXT_PRIMARY` | `#E6E1E5` | Body text |
| `TEXT_SECONDARY` | `#938F99` | Secondary / muted text |
| `SUCCESS` | `#4CAF50` | *Active* bot status |
| `ERROR_CLR` | `#CF6679` | Error / failure indicators |

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

1. Set `TELEGRAM_BOT_TOKEN` and `TELEGRAM_CHAT_ID` in your shell (see [Configuration](#configuration)).
2. Launch the app on your target platform (see [Building & Running](#building--running)).
3. The **Dashboard** opens automatically. The top-right corner shows a green *Connected* dot when the bot credentials are present.
4. Type your message in the **Quick Send** text field.
5. Press **Send** (or hit Enter) — the message is delivered to the configured Telegram chat via the Bot API.
6. The **Messages Sent** counter increments and the message appears in the **Recent Activity** feed below the Quick Send panel.
7. Switch to the **Settings** tab in the sidebar to update the bot token or chat ID at runtime.

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Enter` | Send the current message |
| `Tab` / click | Move focus between controls |

## License

No license file is currently present in this repository. All rights are reserved by the repository owner until an explicit license is added. If you intend to use or contribute to this project, please open an issue to request a license.
