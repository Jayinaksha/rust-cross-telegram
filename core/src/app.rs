use crate::render::particles::ParticleSystem;
use crate::telegram;

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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            message: String::new(),
            token: "TOKEN_ID".to_string(),
            chat_id: "CHAT_ID".to_string(),
            text_field_focused: false,
            text_changed_externally: false,

            selected_tab: Tab::Dashboard,
            messages_sent: 0,
            activity_log: Vec::new(),
            particles: ParticleSystem::new(60),
            time: 0.0,
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

    // Non-interactive (labels, separators)
    style.visuals.widgets.noninteractive.bg_fill = SURFACE;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_SECONDARY);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.5, OUTLINE);

    // Idle interactive widgets
    style.visuals.widgets.inactive.bg_fill = SURFACE;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, OUTLINE);

    // Hovered
    style.visuals.widgets.hovered.bg_fill = SURFACE_HOVER;
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, ACCENT);
    style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, ACCENT);

    // Active / pressed
    style.visuals.widgets.active.bg_fill = ACCENT_DIM;
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    style.visuals.widgets.active.rounding = Rounding::same(8.0);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, ACCENT);

    // Selection highlight
    style.visuals.selection.bg_fill = ACCENT_DIM;
    style.visuals.selection.stroke = Stroke::new(1.0, ACCENT);

    // Spacing
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
        // Smooth fade-in / fade-out at the edges of the lifetime
        let fade = if ratio > 0.8 {
            (1.0 - ratio) / 0.2
        } else if ratio < 0.2 {
            ratio / 0.2
        } else {
            1.0
        };
        let a = (p.alpha * fade * 255.0) as u8;
        // Alternate accent purple / teal based on screen position
        let color = if (p.pos.x as i32) % 2 == 0 {
            Color32::from_rgba_unmultiplied(187, 134, 252, a) // purple
        } else {
            Color32::from_rgba_unmultiplied(3, 218, 198, a) // teal
        };
        painter.circle_filled(egui::pos2(p.pos.x, p.pos.y), p.size, color);
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  render_ui — public entry point
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
pub fn render_ui(ui: &mut Ui, state: &mut AppState) {
    let ctx = ui.ctx().clone();
    let dt = ctx.input(|i| i.stable_dt);
    state.time += dt;

    // Update ambient particles to match current window size
    let screen = ctx.screen_rect();
    state.particles.update(dt, screen.width(), screen.height());

    // Apply the Material dark theme
    apply_theme(&ctx);

    // ── Background + particles ─────────────────────────────────────
    let painter = ui.painter().clone();
    painter.rect_filled(screen, 0.0, BG_BASE);
    draw_ambient_particles(&painter, &state.particles);

    // Keep animating
    ctx.request_repaint();

    // ── Top bar ────────────────────────────────────────────────────
    egui::TopBottomPanel::top("top_bar")
        .exact_height(52.0)
        .frame(
            egui::Frame::none()
                .fill(TOPBAR_BG)
                .stroke(Stroke::new(1.0, OUTLINE))
                .inner_margin(egui::Margin::symmetric(20.0, 0.0)),
        )
        .show(&ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(
                    RichText::new("Telegram Console")
                        .size(17.0)
                        .color(TEXT_PRIMARY)
                        .strong(),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Connected").size(12.0).color(SUCCESS));
                    // Pulsing green dot
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

    // ── Sidebar ────────────────────────────────────────────────────
    egui::SidePanel::left("sidebar")
        .exact_width(190.0)
        .frame(
            egui::Frame::none()
                .fill(SIDEBAR_BG)
                .stroke(Stroke::new(1.0, OUTLINE))
                .inner_margin(egui::Margin::symmetric(12.0, 16.0)),
        )
        .show(&ctx, |ui| {
            ui.add_space(8.0);
            nav_button(ui, "Dashboard", Tab::Dashboard, &mut state.selected_tab);
            ui.add_space(4.0);
            nav_button(ui, "Settings", Tab::Settings, &mut state.selected_tab);

            ui.add_space(20.0);
            // Thin separator
            let r = ui.available_rect_before_wrap();
            ui.painter().line_segment(
                [egui::pos2(r.left(), r.top()), egui::pos2(r.right(), r.top())],
                Stroke::new(0.5, OUTLINE),
            );
            ui.add_space(12.0);

            ui.label(RichText::new("QUICK STATS").size(10.0).color(TEXT_HINT));
            ui.add_space(8.0);
            ui.label(
                RichText::new(format!("{}", state.messages_sent))
                    .size(32.0)
                    .color(ACCENT)
                    .strong(),
            );
            ui.label(RichText::new("messages sent").size(11.0).color(TEXT_SECONDARY));
        });

    // ── Main content area ──────────────────────────────────────────
    egui::CentralPanel::default()
        .frame(
            egui::Frame::none()
                .fill(Color32::TRANSPARENT)
                .inner_margin(egui::Margin::same(24.0)),
        )
        .show(&ctx, |ui| match state.selected_tab {
            Tab::Dashboard => render_dashboard(ui, state),
            Tab::Settings => render_settings(ui, state),
        });
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Sidebar navigation button
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn nav_button(ui: &mut Ui, label: &str, tab: Tab, selected: &mut Tab) {
    let active = *selected == tab;
    let fill = if active { NAV_ACTIVE_BG } else { Color32::TRANSPARENT };
    let text_clr = if active { ACCENT } else { TEXT_SECONDARY };
    let stroke = if active {
        Stroke::new(1.0, ACCENT_DIM)
    } else {
        Stroke::NONE
    };

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

    if ui.add_sized([166.0, 38.0], btn).clicked() {
        *selected = tab;
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Dashboard tab
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn render_dashboard(ui: &mut Ui, state: &mut AppState) {
    ui.label(RichText::new("Dashboard").size(22.0).color(TEXT_PRIMARY).strong());
    ui.add_space(2.0);
    ui.label(
        RichText::new("Send messages and monitor activity")
            .size(13.0)
            .color(TEXT_SECONDARY),
    );
    ui.add_space(16.0);

    // ── Stat cards row ─────────────────────────────────────────────
    ui.columns(2, |cols| {
        // Messages sent
        card(16.0).show(&mut cols[0], |ui| {
            ui.label(RichText::new("MESSAGES SENT").size(10.0).color(TEXT_HINT));
            ui.add_space(6.0);
            ui.label(
                RichText::new(format!("{}", state.messages_sent))
                    .size(36.0)
                    .color(ACCENT)
                    .strong(),
            );
        });

        // Bot status
        card(16.0).show(&mut cols[1], |ui| {
            ui.label(RichText::new("BOT STATUS").size(10.0).color(TEXT_HINT));
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("●").size(12.0).color(SUCCESS));
                ui.label(RichText::new("Active").size(15.0).color(SUCCESS).strong());
            });
            ui.add_space(4.0);
            let masked = if state.token.len() > 10 {
                format!(
                    "{}...{}",
                    &state.token[..6],
                    &state.token[state.token.len() - 4..]
                )
            } else {
                "Not configured".to_string()
            };
            ui.label(RichText::new(masked).size(11.0).color(TEXT_HINT));
        });
    });

    ui.add_space(12.0);

    // ── Quick send card ────────────────────────────────────────────
    card_highlight(20.0).show(ui, |ui| {
        ui.label(
            RichText::new("Quick Send")
                .size(15.0)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(10.0);

        let avail = ui.available_width();
        ui.horizontal(|ui| {
            let input_w = (avail - 96.0).max(80.0);

            // Text input
            let output = TextEdit::singleline(&mut state.message)
                .hint_text(RichText::new("Type your message...").color(TEXT_HINT))
                .desired_width(input_w)
                .show(ui);

            let response = &output.response;

            // Android IME cursor fix
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

            ui.add_space(6.0);

            // Send button
            let btn = egui::Button::new(
                RichText::new("Send")
                    .size(14.0)
                    .color(Color32::from_rgb(18, 18, 24))
                    .strong(),
            )
            .fill(ACCENT)
            .rounding(Rounding::same(8.0));

            if ui.add_sized([72.0, 36.0], btn).clicked()
                && !state.message.trim().is_empty()
            {
                fire_send(state);
            }
        });
    });

    ui.add_space(12.0);

    // ── Activity log card ──────────────────────────────────────────
    card(20.0).show(ui, |ui| {
        ui.label(
            RichText::new("Recent Activity")
                .size(15.0)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(10.0);

        if state.activity_log.is_empty() {
            ui.label(
                RichText::new(
                    "No messages sent yet. Type a message above to get started.",
                )
                .size(12.0)
                .color(TEXT_HINT),
            );
        } else {
            egui::ScrollArea::vertical()
                .max_height(180.0)
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
                                RichText::new(&entry.text).size(12.0).color(TEXT_PRIMARY),
                            );
                        });
                    }
                });
        }
    });
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  Settings tab
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn render_settings(ui: &mut Ui, state: &mut AppState) {
    ui.label(RichText::new("Settings").size(22.0).color(TEXT_PRIMARY).strong());
    ui.add_space(2.0);
    ui.label(
        RichText::new("Configure your Telegram bot connection")
            .size(13.0)
            .color(TEXT_SECONDARY),
    );
    ui.add_space(16.0);

    card(20.0).show(ui, |ui| {
        ui.label(
            RichText::new("Bot Configuration")
                .size(15.0)
                .color(TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(14.0);

        // Token
        ui.label(RichText::new("BOT TOKEN").size(10.0).color(TEXT_HINT));
        ui.add_space(4.0);
        ui.add(
            TextEdit::singleline(&mut state.token)
                .password(true)
                .hint_text("Enter bot token...")
                .desired_width(ui.available_width()),
        );

        ui.add_space(14.0);

        // Chat ID
        ui.label(RichText::new("CHAT ID").size(10.0).color(TEXT_HINT));
        ui.add_space(4.0);
        ui.add(
            TextEdit::singleline(&mut state.chat_id)
                .hint_text("Enter chat ID...")
                .desired_width(ui.available_width()),
        );

        ui.add_space(18.0);

        ui.label(
            RichText::new(
                "Get your bot token from @BotFather on Telegram.\n\
                 Chat ID can be obtained from @userinfobot.",
            )
            .size(11.0)
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
