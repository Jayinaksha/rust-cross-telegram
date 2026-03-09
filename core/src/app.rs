use std::sync::{Arc, Mutex};

use crate::render::particles::ParticleSystem;
use crate::telegram;
use crate::telegram::IncomingMessage;

use egui::text::{CCursor, CCursorRange};
use egui::{self, Color32, RichText, Rounding, Stroke, TextEdit, Ui};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Material Design Dark — colour palette
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
const BG_BASE:       Color32 = Color32::from_rgb(18, 18, 24);
const SURFACE:       Color32 = Color32::from_rgb(30, 30, 46);
const SURFACE_ALT:   Color32 = Color32::from_rgb(36, 36, 52);
const SURFACE_HOVER: Color32 = Color32::from_rgb(44, 44, 62);
const ACCENT:        Color32 = Color32::from_rgb(187, 134, 252); // MD purple
const ACCENT_DIM:    Color32 = Color32::from_rgb(110, 78, 168);
const _SECONDARY:    Color32 = Color32::from_rgb(3, 218, 198);   // MD teal
const TEXT_PRIMARY:  Color32 = Color32::from_rgb(230, 225, 229);
const TEXT_SECONDARY:Color32 = Color32::from_rgb(147, 143, 153);
const TEXT_HINT:     Color32 = Color32::from_rgb(100, 96, 108);
const SUCCESS:       Color32 = Color32::from_rgb(76, 175, 80);
const ERROR_CLR:     Color32 = Color32::from_rgb(207, 102, 121);
const OUTLINE:       Color32 = Color32::from_rgb(73, 69, 79);
const SIDEBAR_BG:    Color32 = Color32::from_rgb(22, 22, 32);
const NAV_ACTIVE_BG: Color32 = Color32::from_rgb(55, 40, 80);
const INPUT_BG:      Color32 = Color32::from_rgb(22, 22, 34);
const TOPBAR_BG:     Color32 = Color32::from_rgb(20, 20, 30);

// Colour for incoming message bubbles
const MSG_INCOMING_BG: Color32 = Color32::from_rgb(38, 38, 56);
// Colour for outgoing (sent by us) message bubbles
const MSG_OUTGOING_BG: Color32 = Color32::from_rgb(55, 40, 80);

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Runtime layout mode — adapts to any screen size
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#[derive(PartialEq, Clone, Copy)]
enum LayoutMode {
    Compact,
    Wide,
}

#[derive(Clone, Copy)]
struct LayoutMetrics {
    mode: LayoutMode,
    _screen_w: f32,
    screen_h: f32,
    content_margin: f32,
    card_pad: f32,
    card_highlight_pad: f32,
    topbar_h: f32,
    sidebar_w: f32,
    bottomnav_h: f32,
    title_size: f32,
    section_size: f32,
    body_size: f32,
    hint_size: f32,
    stat_size: f32,
    send_btn_w: f32,
    send_btn_h: f32,
}

impl LayoutMetrics {
    fn from_screen(w: f32, h: f32) -> Self {
        let mode = if w < 500.0 { LayoutMode::Compact } else { LayoutMode::Wide };
        match mode {
            LayoutMode::Compact => Self {
                mode,
                _screen_w: w,
                screen_h: h,
                content_margin: (w * 0.03).clamp(8.0, 16.0),
                card_pad: (w * 0.035).clamp(10.0, 16.0),
                card_highlight_pad: (w * 0.04).clamp(12.0, 20.0),
                topbar_h: 44.0,
                sidebar_w: 0.0,
                bottomnav_h: 52.0,
                title_size: (w * 0.045).clamp(16.0, 22.0),
                section_size: (w * 0.035).clamp(13.0, 15.0),
                body_size: (w * 0.032).clamp(11.0, 14.0),
                hint_size: (w * 0.025).clamp(9.0, 11.0),
                stat_size: (w * 0.07).clamp(24.0, 36.0),
                send_btn_w: (w * 0.18).clamp(56.0, 80.0),
                send_btn_h: 34.0,
            },
            LayoutMode::Wide => Self {
                mode,
                _screen_w: w,
                screen_h: h,
                content_margin: 24.0,
                card_pad: 16.0,
                card_highlight_pad: 20.0,
                topbar_h: 52.0,
                sidebar_w: (w * 0.2).clamp(160.0, 220.0),
                bottomnav_h: 0.0,
                title_size: 22.0,
                section_size: 15.0,
                body_size: 13.0,
                hint_size: 10.0,
                stat_size: 36.0,
                send_btn_w: 72.0,
                send_btn_h: 36.0,
            },
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Data types
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Dashboard,
    Settings,
}

pub struct LogEntry {
    pub text: String,
    pub success: bool,
}

/// A chat message displayed in the UI — either sent by us or received.
pub struct ChatMessage {
    pub from_name: String,
    pub text: String,
    pub is_ours: bool,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  App State
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub struct AppState {
    pub message: String,
    pub token: String,
    pub chat_id: String,
    pub text_field_focused: bool,
    pub text_changed_externally: bool,

    // UI-specific
    pub selected_tab: Tab,
    pub messages_sent: u32,
    pub activity_log: Vec<LogEntry>,
    pub particles: ParticleSystem,
    pub time: f32,

    // ── Incoming messages (polling) ────────────────────────────────
    /// Shared buffer: background polling task pushes here, UI drains each frame.
    pub incoming_buf: Arc<Mutex<Vec<IncomingMessage>>>,
    /// Chat messages displayed in the UI (sent + received, chronological).
    pub chat_messages: Vec<ChatMessage>,
    /// Whether the background polling task has been spawned.
    pub polling_started: bool,
    /// Shared flag the polling loop reads to know current token.
    pub poll_token: Arc<Mutex<String>>,
    /// Shared flag for current chat_id (so polling can filter).
    pub poll_chat_id: Arc<Mutex<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        let token = "TOKEN_ID".to_string();
        let chat_id = "CHAT_ID".to_string();
        Self {
            message: String::new(),
            token: token.clone(),
            chat_id: chat_id.clone(),
            text_field_focused: false,
            text_changed_externally: false,

            selected_tab: Tab::Dashboard,
            messages_sent: 0,
            activity_log: Vec::new(),
            particles: ParticleSystem::new(60),
            time: 0.0,

            incoming_buf: Arc::new(Mutex::new(Vec::new())),
            chat_messages: Vec::new(),
            polling_started: false,
            poll_token: Arc::new(Mutex::new(token)),
            poll_chat_id: Arc::new(Mutex::new(chat_id)),
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Frame helpers
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn card(pad: f32) -> egui::Frame {
    egui::Frame::none()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, OUTLINE))
        .rounding(Rounding::same(12.0))
        .inner_margin(egui::Margin::same(pad))
}

fn card_highlight(pad: f32) -> egui::Frame {
    egui::Frame::none()
        .fill(SURFACE_ALT)
        .stroke(Stroke::new(1.5, ACCENT_DIM))
        .rounding(Rounding::same(12.0))
        .inner_margin(egui::Margin::same(pad))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Theme application
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(TEXT_PRIMARY);
    style.visuals.window_fill = BG_BASE;
    style.visuals.panel_fill = BG_BASE;
    style.visuals.extreme_bg_color = INPUT_BG;

    style.visuals.widgets.noninteractive.bg_fill = SURFACE;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_SECONDARY);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.5, OUTLINE);

    style.visuals.widgets.inactive.bg_fill = SURFACE;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, OUTLINE);

    style.visuals.widgets.hovered.bg_fill = SURFACE_HOVER;
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, ACCENT);
    style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, ACCENT);

    style.visuals.widgets.active.bg_fill = ACCENT_DIM;
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    style.visuals.widgets.active.rounding = Rounding::same(8.0);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, ACCENT);

    style.visuals.selection.bg_fill = ACCENT_DIM;
    style.visuals.selection.stroke = Stroke::new(1.0, ACCENT);

    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);

    ctx.set_style(style);
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Ambient particle drawing
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn draw_ambient_particles(painter: &egui::Painter, particles: &ParticleSystem) {
    for p in &particles.particles {
        let ratio = (p.life / p.max_life).clamp(0.0, 1.0);
        let fade = if ratio > 0.8 {
            (1.0 - ratio) / 0.2
        } else if ratio < 0.2 {
            ratio / 0.2
        } else {
            1.0
        };
        let a = (p.alpha * fade * 255.0) as u8;
        let color = if (p.pos.x as i32) % 2 == 0 {
            Color32::from_rgba_unmultiplied(187, 134, 252, a)
        } else {
            Color32::from_rgba_unmultiplied(3, 218, 198, a)
        };
        painter.circle_filled(egui::pos2(p.pos.x, p.pos.y), p.size, color);
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Background polling — spawn once, runs forever
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Start the background polling task. Called once on the first frame.
fn start_polling(state: &mut AppState) {
    if state.polling_started {
        return;
    }
    state.polling_started = true;

    let buf = Arc::clone(&state.incoming_buf);
    let poll_token = Arc::clone(&state.poll_token);

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let mut offset: i64 = 0;
                // On first poll, use offset -1 with limit 0 to skip old messages
                // Actually, just use offset 0 and the first batch will contain
                // any unacknowledged updates. We'll mark them with the offset.
                loop {
                    let token = {
                        poll_token.lock().unwrap().clone()
                    };
                    if token.is_empty() {
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                        continue;
                    }
                    match telegram::get_updates(&token, offset, 10).await {
                        Ok((msgs, next)) => {
                            offset = next;
                            if !msgs.is_empty() {
                                if let Ok(mut locked) = buf.lock() {
                                    locked.extend(msgs);
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!("getUpdates error: {}", e);
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        }
                    }
                }
            });
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let mut offset: i64 = 0;
            loop {
                let token = {
                    poll_token.lock().unwrap().clone()
                };
                if token.is_empty() {
                    gloo_timers_sleep(3000).await;
                    continue;
                }
                match telegram::get_updates(&token, offset, 0).await {
                    Ok((msgs, next)) => {
                        offset = next;
                        if !msgs.is_empty() {
                            if let Ok(mut locked) = buf.lock() {
                                locked.extend(msgs);
                            }
                        }
                    }
                    Err(_e) => {
                        // Silently retry
                    }
                }
                // Poll every 3 seconds in WASM
                gloo_timers_sleep(3000).await;
            }
        });
    }
}

/// Simple async sleep for WASM using setTimeout.
#[cfg(target_arch = "wasm32")]
async fn gloo_timers_sleep(ms: i32) {
    use wasm_bindgen::JsCast;
    let (sender, receiver) = futures_channel::oneshot::channel::<()>();
    let closure = wasm_bindgen::closure::Closure::once(move || {
        let _ = sender.send(());
    });
    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            ms,
        )
        .unwrap();
    closure.forget();
    let _ = receiver.await;
}

/// Drain the shared incoming buffer into AppState's chat_messages.
fn drain_incoming(state: &mut AppState) {
    if let Ok(mut locked) = state.incoming_buf.try_lock() {
        for msg in locked.drain(..) {
            state.chat_messages.push(ChatMessage {
                from_name: msg.from_name,
                text: msg.text,
                is_ours: false,
            });
        }
        // Cap at 200 messages
        if state.chat_messages.len() > 200 {
            let excess = state.chat_messages.len() - 200;
            state.chat_messages.drain(..excess);
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  render_ui — public entry point (responsive)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub fn render_ui(ui: &mut Ui, state: &mut AppState) {
    let ctx = ui.ctx().clone();
    let dt = ctx.input(|i| i.stable_dt);
    state.time += dt;

    // ── Start polling on first frame ───────────────────────────────
    start_polling(state);

    // ── Sync shared token/chat_id for the polling thread ───────────
    if let Ok(mut t) = state.poll_token.try_lock() {
        if *t != state.token {
            *t = state.token.clone();
        }
    }
    if let Ok(mut c) = state.poll_chat_id.try_lock() {
        if *c != state.chat_id {
            *c = state.chat_id.clone();
        }
    }

    // ── Drain incoming messages ────────────────────────────────────
    drain_incoming(state);

    // ── Runtime screen-size detection ──────────────────────────────
    let screen = ctx.screen_rect();
    let lm = LayoutMetrics::from_screen(screen.width(), screen.height());

    state.particles.update(dt, screen.width(), screen.height());
    apply_theme(&ctx);

    // ── Background + particles ─────────────────────────────────────
    let painter = ui.painter().clone();
    painter.rect_filled(screen, 0.0, BG_BASE);
    draw_ambient_particles(&painter, &state.particles);
    ctx.request_repaint();

    // ── Top bar ────────────────────────────────────────────────────
    egui::TopBottomPanel::top("top_bar")
        .exact_height(lm.topbar_h)
        .frame(
            egui::Frame::none()
                .fill(TOPBAR_BG)
                .stroke(Stroke::new(1.0, OUTLINE))
                .inner_margin(egui::Margin::symmetric(
                    if lm.mode == LayoutMode::Compact { 12.0 } else { 20.0 },
                    0.0,
                )),
        )
        .show(&ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let title_sz = if lm.mode == LayoutMode::Compact { 14.0 } else { 17.0 };
                ui.label(
                    RichText::new("Telegram Console")
                        .size(title_sz)
                        .color(TEXT_PRIMARY)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Connected").size(12.0).color(SUCCESS));
                    let pulse = ((state.time * 2.5).sin() * 0.35 + 0.65).clamp(0.4, 1.0);
                    let (dot, _) =
                        ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
                    ui.painter().circle_filled(
                        dot.center(),
                        4.0,
                        Color32::from_rgb(
                            (76.0 * pulse) as u8,
                            (175.0 * pulse) as u8,
                            (80.0 * pulse) as u8,
                        ),
                    );
                });
            });
        });

    // ── Bottom nav (Compact mode only) ─────────────────────────────
    if lm.mode == LayoutMode::Compact {
        egui::TopBottomPanel::bottom("bottom_nav")
            .exact_height(lm.bottomnav_h)
            .frame(
                egui::Frame::none()
                    .fill(SIDEBAR_BG)
                    .stroke(Stroke::new(1.0, OUTLINE))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0)),
            )
            .show(&ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    let btn_w = (ui.available_width() / 2.0 - 8.0).min(180.0);
                    compact_nav_button(ui, "◆ Dashboard", Tab::Dashboard, &mut state.selected_tab, btn_w);
                    compact_nav_button(ui, "● Settings", Tab::Settings, &mut state.selected_tab, btn_w);
                });
            });
    }

    // ── Sidebar (Wide mode only) ───────────────────────────────────
    if lm.mode == LayoutMode::Wide {
        egui::SidePanel::left("sidebar")
            .exact_width(lm.sidebar_w)
            .frame(
                egui::Frame::none()
                    .fill(SIDEBAR_BG)
                    .stroke(Stroke::new(1.0, OUTLINE))
                    .inner_margin(egui::Margin::symmetric(12.0, 16.0)),
            )
            .show(&ctx, |ui| {
                ui.add_space(8.0);
                let nav_w = (lm.sidebar_w - 24.0).max(80.0);
                nav_button(ui, "Dashboard", Tab::Dashboard, &mut state.selected_tab, nav_w);
                ui.add_space(4.0);
                nav_button(ui, "Settings", Tab::Settings, &mut state.selected_tab, nav_w);

                ui.add_space(20.0);
                let r = ui.available_rect_before_wrap();
                ui.painter().line_segment(
                    [egui::pos2(r.left(), r.top()), egui::pos2(r.right(), r.top())],
                    Stroke::new(0.5, OUTLINE),
                );
                ui.add_space(12.0);

                ui.label(RichText::new("QUICK STATS").size(lm.hint_size).color(TEXT_HINT));
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("{}", state.messages_sent))
                        .size(32.0)
                        .color(ACCENT)
                        .strong(),
                );
                ui.label(RichText::new("messages sent").size(11.0).color(TEXT_SECONDARY));

                ui.add_space(8.0);
                let recv_count = state.chat_messages.iter().filter(|m| !m.is_ours).count();
                ui.label(
                    RichText::new(format!("{}", recv_count))
                        .size(32.0)
                        .color(SUCCESS)
                        .strong(),
                );
                ui.label(RichText::new("received").size(11.0).color(TEXT_SECONDARY));
            });
    }

    // ── Main content area (scrollable on ALL layouts) ──────────────
    egui::CentralPanel::default()
        .frame(
            egui::Frame::none()
                .fill(Color32::TRANSPARENT)
                .inner_margin(egui::Margin::same(lm.content_margin)),
        )
        .show(&ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    match state.selected_tab {
                        Tab::Dashboard => render_dashboard(ui, state, &lm),
                        Tab::Settings => render_settings(ui, state, &lm),
                    }
                });
        });
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Navigation buttons
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn nav_button(ui: &mut Ui, label: &str, tab: Tab, selected: &mut Tab, width: f32) {
    let active = *selected == tab;
    let fill = if active { NAV_ACTIVE_BG } else { Color32::TRANSPARENT };
    let text_clr = if active { ACCENT } else { TEXT_SECONDARY };
    let stroke = if active { Stroke::new(1.0, ACCENT_DIM) } else { Stroke::NONE };

    let icon = match tab {
        Tab::Dashboard => "◆",
        Tab::Settings  => "●",
    };

    let btn = egui::Button::new(
        RichText::new(format!("  {}  {}", icon, label))
            .size(13.5)
            .color(text_clr),
    )
    .fill(fill)
    .stroke(stroke)
    .rounding(Rounding::same(8.0));

    if ui.add_sized([width, 38.0], btn).clicked() {
        *selected = tab;
    }
}

fn compact_nav_button(ui: &mut Ui, label: &str, tab: Tab, selected: &mut Tab, width: f32) {
    let active = *selected == tab;
    let fill = if active { NAV_ACTIVE_BG } else { Color32::TRANSPARENT };
    let text_clr = if active { ACCENT } else { TEXT_SECONDARY };
    let stroke = if active { Stroke::new(1.0, ACCENT_DIM) } else { Stroke::NONE };

    let btn = egui::Button::new(
        RichText::new(label).size(13.0).color(text_clr),
    )
    .fill(fill)
    .stroke(stroke)
    .rounding(Rounding::same(8.0));

    if ui.add_sized([width, 38.0], btn).clicked() {
        *selected = tab;
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Dashboard tab (responsive)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn render_dashboard(ui: &mut Ui, state: &mut AppState, lm: &LayoutMetrics) {
    ui.label(RichText::new("Dashboard").size(lm.title_size).color(TEXT_PRIMARY).strong());
    ui.add_space(2.0);
    ui.label(
        RichText::new("Send messages and monitor activity")
            .size(lm.body_size)
            .color(TEXT_SECONDARY),
    );
    ui.add_space(if lm.mode == LayoutMode::Compact { 10.0 } else { 16.0 });

    // ── Stat cards ─────────────────────────────────────────────────
    let avail_w = ui.available_width();
    let use_two_cols = avail_w > 340.0;

    if use_two_cols {
        ui.columns(2, |cols| {
            card(lm.card_pad).show(&mut cols[0], |ui| {
                ui.label(RichText::new("MESSAGES SENT").size(lm.hint_size).color(TEXT_HINT));
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("{}", state.messages_sent))
                        .size(lm.stat_size)
                        .color(ACCENT)
                        .strong(),
                );
            });

            card(lm.card_pad).show(&mut cols[1], |ui| {
                ui.label(RichText::new("BOT STATUS").size(lm.hint_size).color(TEXT_HINT));
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("●").size(12.0).color(SUCCESS));
                    ui.label(RichText::new("Active").size(lm.section_size).color(SUCCESS).strong());
                });
                ui.add_space(4.0);
                let masked = mask_token(&state.token);
                ui.label(RichText::new(masked).size(lm.hint_size).color(TEXT_HINT));
            });
        });
    } else {
        card(lm.card_pad).show(ui, |ui| {
            ui.label(RichText::new("MESSAGES SENT").size(lm.hint_size).color(TEXT_HINT));
            ui.add_space(4.0);
            ui.label(
                RichText::new(format!("{}", state.messages_sent))
                    .size(lm.stat_size)
                    .color(ACCENT)
                    .strong(),
            );
        });
        ui.add_space(8.0);
        card(lm.card_pad).show(ui, |ui| {
            ui.label(RichText::new("BOT STATUS").size(lm.hint_size).color(TEXT_HINT));
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("●").size(12.0).color(SUCCESS));
                ui.label(RichText::new("Active").size(lm.section_size).color(SUCCESS).strong());
            });
            ui.add_space(2.0);
            let masked = mask_token(&state.token);
            ui.label(RichText::new(masked).size(lm.hint_size).color(TEXT_HINT));
        });
    }

    ui.add_space(if lm.mode == LayoutMode::Compact { 8.0 } else { 12.0 });

    // ── Messages card (chat view) ──────────────────────────────────
    card(lm.card_pad).show(ui, |ui| {
        ui.label(
            RichText::new("Messages")
                .size(lm.section_size)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(4.0);
        ui.label(
            RichText::new("Sent and received messages appear here")
                .size(lm.hint_size)
                .color(TEXT_HINT),
        );
        ui.add_space(8.0);

        let chat_h = if lm.mode == LayoutMode::Compact {
            (lm.screen_h * 0.30).clamp(120.0, 280.0)
        } else {
            (lm.screen_h * 0.35).clamp(150.0, 400.0)
        };

        if state.chat_messages.is_empty() {
            ui.label(
                RichText::new("No messages yet. Send one or receive from Telegram!")
                    .size(lm.body_size - 1.0)
                    .color(TEXT_HINT),
            );
        } else {
            let scroll_out = egui::ScrollArea::vertical()
                .id_source("chat_scroll")
                .max_height(chat_h)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for msg in &state.chat_messages {
                        render_chat_bubble(ui, msg, lm);
                        ui.add_space(4.0);
                    }
                });
            let _ = scroll_out;
        }
    });

    ui.add_space(if lm.mode == LayoutMode::Compact { 8.0 } else { 12.0 });

    // ── Quick send card ────────────────────────────────────────────
    card_highlight(lm.card_highlight_pad).show(ui, |ui| {
        ui.label(
            RichText::new("Quick Send")
                .size(lm.section_size)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(8.0);

        let avail = ui.available_width();

        if lm.mode == LayoutMode::Compact {
            let output = TextEdit::singleline(&mut state.message)
                .hint_text(RichText::new("Type your message...").color(TEXT_HINT))
                .desired_width(avail)
                .show(ui);

            let response = &output.response;

            if state.text_changed_externally {
                let ccursor = CCursor::new(state.message.len());
                let mut es = output.state.clone();
                es.cursor.set_char_range(Some(CCursorRange::one(ccursor)));
                es.store(ui.ctx(), response.id);
                state.text_changed_externally = false;
            }

            let _was_focused = state.text_field_focused;
            state.text_field_focused = response.has_focus();
            if response.has_focus() {
                ui.ctx().request_repaint();
            }

            // Send on Enter key while the text field has focus
            if response.lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                && !state.message.trim().is_empty()
            {
                fire_send(state);
            }

            ui.add_space(6.0);

            let btn = egui::Button::new(
                RichText::new("Send")
                    .size(lm.body_size)
                    .color(Color32::from_rgb(18, 18, 24))
                    .strong(),
            )
            .fill(ACCENT)
            .rounding(Rounding::same(8.0));

            if ui.add_sized([avail, lm.send_btn_h], btn).clicked()
                && !state.message.trim().is_empty()
            {
                fire_send(state);
            }
        } else {
            ui.horizontal(|ui| {
                let input_w = (avail - lm.send_btn_w - 24.0).max(80.0);

                let output = TextEdit::singleline(&mut state.message)
                    .hint_text(RichText::new("Type your message...").color(TEXT_HINT))
                    .desired_width(input_w)
                    .show(ui);

                let response = &output.response;

                if state.text_changed_externally {
                    let ccursor = CCursor::new(state.message.len());
                    let mut es = output.state.clone();
                    es.cursor.set_char_range(Some(CCursorRange::one(ccursor)));
                    es.store(ui.ctx(), response.id);
                    state.text_changed_externally = false;
                }

                let _was_focused = state.text_field_focused;
                state.text_field_focused = response.has_focus();
                if response.has_focus() {
                    ui.ctx().request_repaint();
                }

                // Send on Enter key while the text field has focus
                if response.lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    && !state.message.trim().is_empty()
                {
                    fire_send(state);
                }

                ui.add_space(6.0);

                let btn = egui::Button::new(
                    RichText::new("Send")
                        .size(14.0)
                        .color(Color32::from_rgb(18, 18, 24))
                        .strong(),
                )
                .fill(ACCENT)
                .rounding(Rounding::same(8.0));

                if ui.add_sized([lm.send_btn_w, lm.send_btn_h], btn).clicked()
                    && !state.message.trim().is_empty()
                {
                    fire_send(state);
                }
            });
        }
    });

    ui.add_space(if lm.mode == LayoutMode::Compact { 8.0 } else { 12.0 });

    // ── Activity log card ──────────────────────────────────────────
    card(lm.card_pad).show(ui, |ui| {
        ui.label(
            RichText::new("Recent Activity")
                .size(lm.section_size)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(8.0);

        if state.activity_log.is_empty() {
            ui.label(
                RichText::new(
                    "No messages sent yet. Type a message above to get started.",
                )
                .size(lm.body_size - 1.0)
                .color(TEXT_HINT),
            );
        } else {
            let max_log_h = if lm.mode == LayoutMode::Compact {
                (lm.screen_h * 0.25).clamp(80.0, 200.0)
            } else {
                (lm.screen_h * 0.25).clamp(100.0, 250.0)
            };

            egui::ScrollArea::vertical()
                .id_source("activity_scroll")
                .max_height(max_log_h)
                .show(ui, |ui| {
                    for (i, entry) in state.activity_log.iter().enumerate() {
                        if i > 0 {
                            ui.add_space(2.0);
                            let r = ui.available_rect_before_wrap();
                            ui.painter().line_segment(
                                [
                                    egui::pos2(r.left(), r.top()),
                                    egui::pos2(r.right(), r.top()),
                                ],
                                Stroke::new(0.5, OUTLINE),
                            );
                            ui.add_space(4.0);
                        }
                        ui.horizontal(|ui| {
                            let (icon, clr) = if entry.success {
                                ("✓", SUCCESS)
                            } else {
                                ("✗", ERROR_CLR)
                            };
                            ui.label(RichText::new(icon).size(12.0).color(clr));
                            ui.label(
                                RichText::new(&entry.text).size(lm.body_size - 1.0).color(TEXT_PRIMARY),
                            );
                        });
                    }
                });
        }
    });
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Chat bubble renderer
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn render_chat_bubble(ui: &mut Ui, msg: &ChatMessage, lm: &LayoutMetrics) {
    let bg = if msg.is_ours { MSG_OUTGOING_BG } else { MSG_INCOMING_BG };
    let align = if msg.is_ours {
        egui::Layout::right_to_left(egui::Align::TOP)
    } else {
        egui::Layout::left_to_right(egui::Align::TOP)
    };

    ui.with_layout(align, |ui| {
        let max_bubble_w = ui.available_width() * 0.8;
        ui.set_max_width(max_bubble_w);

        egui::Frame::none()
            .fill(bg)
            .rounding(Rounding::same(10.0))
            .inner_margin(egui::Margin::symmetric(10.0, 6.0))
            .show(ui, |ui| {
                let name_color = if msg.is_ours { ACCENT } else { SUCCESS };
                let label = if msg.is_ours { "You" } else { &msg.from_name };
                ui.label(
                    RichText::new(label)
                        .size(lm.hint_size + 1.0)
                        .color(name_color)
                        .strong(),
                );
                ui.label(
                    RichText::new(&msg.text)
                        .size(lm.body_size)
                        .color(TEXT_PRIMARY),
                );
            });
    });
}

/// Mask a bot token for display.
fn mask_token(token: &str) -> String {
    if token.len() > 10 {
        format!(
            "{}...{}",
            &token[..6],
            &token[token.len() - 4..]
        )
    } else {
        "Not configured".to_string()
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Settings tab (responsive)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn render_settings(ui: &mut Ui, state: &mut AppState, lm: &LayoutMetrics) {
    ui.label(RichText::new("Settings").size(lm.title_size).color(TEXT_PRIMARY).strong());
    ui.add_space(2.0);
    ui.label(
        RichText::new("Configure your Telegram bot connection")
            .size(lm.body_size)
            .color(TEXT_SECONDARY),
    );
    ui.add_space(if lm.mode == LayoutMode::Compact { 10.0 } else { 16.0 });

    card(lm.card_pad).show(ui, |ui| {
        ui.label(
            RichText::new("Bot Configuration")
                .size(lm.section_size)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(if lm.mode == LayoutMode::Compact { 10.0 } else { 14.0 });

        ui.label(RichText::new("BOT TOKEN").size(lm.hint_size).color(TEXT_HINT));
        ui.add_space(4.0);
        ui.add(
            TextEdit::singleline(&mut state.token)
                .password(true)
                .hint_text("Enter bot token...")
                .desired_width(ui.available_width()),
        );

        ui.add_space(if lm.mode == LayoutMode::Compact { 10.0 } else { 14.0 });

        ui.label(RichText::new("CHAT ID").size(lm.hint_size).color(TEXT_HINT));
        ui.add_space(4.0);
        ui.add(
            TextEdit::singleline(&mut state.chat_id)
                .hint_text("Enter chat ID...")
                .desired_width(ui.available_width()),
        );

        ui.add_space(if lm.mode == LayoutMode::Compact { 12.0 } else { 18.0 });

        ui.label(
            RichText::new(
                "Get your bot token from @BotFather on Telegram.\n\
                 Chat ID can be obtained from @userinfobot.",
            )
            .size(lm.hint_size + 1.0)
            .color(TEXT_HINT),
        );
    });
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Send helper  (platform-conditional)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn fire_send(state: &mut AppState) {
    let msg   = state.message.clone();
    let token = state.token.clone();
    let chat  = state.chat_id.clone();

    state.message.clear();
    state.messages_sent += 1;

    // Add to chat messages as "ours"
    state.chat_messages.push(ChatMessage {
        from_name: "You".to_string(),
        text: msg.clone(),
        is_ours: true,
    });

    // Truncate for the activity log display
    let display = if msg.len() > 50 {
        format!("{}...", &msg[..50])
    } else {
        msg.clone()
    };
    state.activity_log.insert(
        0,
        LogEntry {
            text: display,
            success: true,
        },
    );
    if state.activity_log.len() > 30 {
        state.activity_log.truncate(30);
    }

    // Native platforms
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = telegram::send_message(&token, &chat, &msg).await;
            });
        });
    }

    // WASM
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let _ = telegram::send_message(&token, &chat, &msg).await;
        });
    }
}
