use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Gauge, Padding, Paragraph, Wrap},
};
use crate::app::App;
use crate::core::theme::Theme;
use crate::core::ascii_art::{current_frame, current_vibe_frame};

pub fn draw_home(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();

    // Root layout: Title | Body | Stats | Actions | Log
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title bar
            Constraint::Min(14),     // Pet display (with wave bars)
            Constraint::Length(5),   // Stats gauges
            Constraint::Length(3),   // Action bar
            Constraint::Length(5),   // Message log
        ])
        .split(area);

    draw_title_bar(frame, app, theme, root[0]);
    draw_pet_panel(frame, app, theme, root[1]);
    draw_stats_panel(frame, app, theme, root[2]);
    draw_action_bar(frame, app, theme, root[3]);
    draw_message_log(frame, app, theme, root[4]);
}

// ── Title bar ──────────────────────────────────────────────────────────────

fn draw_title_bar(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let days = chrono::Utc::now().signed_duration_since(app.pet.created_at).num_days() + 1;
    let music_tag = if app.music_playing { "  🎵 Vibing" } else { "" };
    let title = format!(
        " 🐱 Terminal Pet — \"{}\" (Day {})   Theme: {} {}{} ",
        app.pet.name,
        days,
        theme.variant.icon(),
        theme.variant.name(),
        music_tag,
    );

    let para = Paragraph::new(title)
        .style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border))
                .style(Style::default().bg(theme.status_bg)),
        );

    frame.render_widget(para, area);
}

// ── Pet display panel (with optional wave bars) ───────────────────────────

fn draw_pet_panel(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    if app.music_playing {
        // Split horizontally: wave | pet | wave
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(7),   // Left wave bar
                Constraint::Min(20),     // Pet art
                Constraint::Length(7),   // Right wave bar
            ])
            .split(area);

        draw_wave_bar(frame, theme, cols[0], app.anim_tick, false);
        draw_pet_art(frame, app, theme, cols[1], true);
        draw_wave_bar(frame, theme, cols[2], app.anim_tick, true);
    } else {
        draw_pet_art(frame, app, theme, area, false);
    }
}

// ─── Wave bar ─────────────────────────────────────────────────────────────
//
// Draws a simple ASCII equaliser column. We use two alternating patterns
// (frame A / frame B) that loop with the anim_tick — exactly 2 arts, as
// requested. The bar heights shift between frames for a pulsing effect.

/// Heights for each of the 5 columns in the wave bar, frame A and frame B.
/// Values are 1–7 (max inner height of a panel that is ≈ 12 rows tall).
const WAVE_A: [u16; 5] = [2, 5, 7, 4, 2];
const WAVE_B: [u16; 5] = [4, 2, 5, 7, 3];

fn draw_wave_bar(frame: &mut Frame, theme: &Theme, area: Rect, tick: u64, _mirror: bool) {
    let heights = if (tick / 3) % 2 == 0 { WAVE_A } else { WAVE_B };

    // Inner height available (subtract top/bottom border = 2)
    let inner_h = area.height.saturating_sub(2) as usize;
    let inner_w = area.width.saturating_sub(2) as usize; // minus borders

    // Build lines top→bottom
    let mut lines: Vec<Line> = Vec::new();
    for row in 0..inner_h {
        let row_from_bottom = (inner_h - 1 - row) as u16;
        // Each column of the bar is 1 char wide; distribute across inner_w
        let mut spans: Vec<Span> = Vec::new();
        for col_idx in 0..5 {
            let bar_h = heights[col_idx];
            let lit = row_from_bottom < bar_h;

            // Colour: top of bar is accent, bottom is muted
            let brightness = bar_h.saturating_sub(row_from_bottom);
            let color = if !lit {
                Color::DarkGray
            } else if brightness >= 5 {
                theme.accent
            } else if brightness >= 3 {
                theme.primary
            } else {
                theme.muted
            };

            let ch = if lit { "█" } else { "░" };
            spans.push(Span::styled(ch, Style::default().fg(color)));
        }
        // Pad to fill inner_w
        let used = 5;
        if inner_w > used {
            let pad = " ".repeat(inner_w - used);
            spans.insert(0, Span::raw(pad.clone()));
        }
        lines.push(Line::from(spans));
    }

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
        );

    frame.render_widget(para, area);
}

// ─── Pet art ──────────────────────────────────────────────────────────────

fn draw_pet_art(frame: &mut Frame, app: &App, theme: &Theme, area: Rect, vibing: bool) {
    let art = if vibing {
        current_vibe_frame(app.anim_tick)
    } else {
        let visual_state = app.pet.visual_state();
        current_frame(visual_state, app.anim_tick)
    };

    // Build styled text — every non-empty line gets the primary theme color
    let styled_lines: Vec<Line> = art
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let width = Span::raw(line).width();
            let pad_len = 25_usize.saturating_sub(width);
            let padded = format!("      {}{}", line, " ".repeat(pad_len));

            // Accent colour for the pet while vibing, primary otherwise
            let fg = if vibing { theme.accent } else { theme.primary };
            Line::from(Span::styled(
                padded,
                Style::default()
                    .fg(fg)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect();

    // Calculate dynamic vertical padding to center the art block
    let inner_height = area.height.saturating_sub(2);
    let art_height = styled_lines.len() as u16;
    let top_padding = inner_height.saturating_sub(art_height) / 2;

    let mut final_lines = Vec::new();
    for _ in 0..top_padding {
        final_lines.push(Line::from(""));
    }
    final_lines.extend(styled_lines);

    let mood_label = if vibing {
        format!(" {} 🎵 ", app.pet.mood().label())
    } else {
        format!(" {} ", app.pet.mood().label())
    };

    let border_style = if vibing {
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.border)
    };

    let para = Paragraph::new(Text::from(final_lines))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title(Span::styled(
                    mood_label,
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center)
                .padding(Padding::uniform(1)),
        );

    frame.render_widget(para, area);
}

// ── Stats gauges ──────────────────────────────────────────────────────────

fn draw_stats_panel(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    // Bond gauge
    let bond_label = format!("❤️  Bond  {:.0}%", app.pet.bond);
    let bond_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border)),
        )
        .gauge_style(Style::default().fg(theme.gauge_bond).bg(Color::Reset))
        .percent(app.pet.bond_pct())
        .label(bond_label);
    frame.render_widget(bond_gauge, cols[0]);

    // Hunger gauge (inverted display: show "fullness")
    let fullness = 100u16.saturating_sub(app.pet.hunger_pct());
    let hunger_label = format!("🍖  Fullness  {}%", fullness);
    let hunger_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border)),
        )
        .gauge_style(Style::default().fg(theme.gauge_hunger).bg(Color::Reset))
        .percent(fullness)
        .label(hunger_label);
    frame.render_widget(hunger_gauge, cols[1]);

    // Happiness gauge
    let happy_label = format!("✨  Joy  {:.0}%", app.pet.happiness);
    let happy_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border)),
        )
        .gauge_style(Style::default().fg(theme.gauge_happiness).bg(Color::Reset))
        .percent(app.pet.happiness_pct())
        .label(happy_label);
    frame.render_widget(happy_gauge, cols[2]);
}

// ── Action bar ────────────────────────────────────────────────────────────

fn draw_action_bar(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Build styled keybind hints; grey out if on cooldown
    let cd = &app.pet.cooldowns;

    let feed_style = if cd.feed_ready() {
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.muted)
    };
    let pat_style = if cd.pat_ready() {
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.muted)
    };
    let play_style = if cd.play_ready() {
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.muted)
    };

    // Music button style: accent when on, muted when off
    let music_style = if app.music_playing {
        Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.muted).add_modifier(Modifier::BOLD)
    };
    let music_label = if app.music_playing { "Music ♪" } else { "Music" };

    let sep = Span::styled("  │  ", Style::default().fg(theme.muted));
    let dim = Style::default().fg(theme.help_text);

    let line = Line::from(vec![
        Span::styled("[F] ", feed_style),
        Span::styled("Feed", dim),
        sep.clone(),
        Span::styled("[P] ", pat_style),
        Span::styled("Pat", dim),
        sep.clone(),
        Span::styled("[Y] ", play_style),
        Span::styled("Play", dim),
        sep.clone(),
        Span::styled("[U] ", music_style),
        Span::styled(music_label, dim),
        sep.clone(),
        Span::styled("[S] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Shop", dim),
        sep.clone(),
        Span::styled("[T] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Theme", dim),
        sep.clone(),
        Span::styled("[M] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Menu", dim),
        sep.clone(),
        Span::styled("[H] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled("Help", dim),
        sep.clone(),
        Span::styled("[Q] ", Style::default().fg(theme.muted).add_modifier(Modifier::BOLD)),
        Span::styled("Quit", dim),
    ]);

    let para = Paragraph::new(line)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    " Actions ",
                    Style::default().fg(theme.accent),
                )),
        );

    frame.render_widget(para, area);
}

// ── Message log ───────────────────────────────────────────────────────────

fn draw_message_log(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Show the most recent messages that fit (max 3 lines visible)
    let lines: Vec<Line> = app
        .messages
        .iter()
        .rev()
        .take(3)
        .rev()
        .map(|msg| {
            Line::from(Span::styled(
                format!(" ▸ {}", msg),
                Style::default().fg(theme.message),
            ))
        })
        .collect();

    let para = Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    " Log ",
                    Style::default().fg(theme.accent),
                )),
        );

    frame.render_widget(para, area);
}
