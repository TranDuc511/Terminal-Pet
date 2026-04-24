// ui.rs — Ratatui rendering for all screens.
//
// Renders:
//   • The main game screen (pet, stats, actions, message log)
//   • A "name your pet" input overlay (first launch)
//   • A "help" overlay

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, Gauge, Padding, Paragraph, Wrap,
    },
};

use crate::{
    app::{App, Screen},
    ascii_art::current_frame,
    theme::Theme,
};

// ─── Public entry point ───────────────────────────────────────────────────

/// Draw the current screen into the given `Frame`.
pub fn draw(frame: &mut Frame, app: &App) {
    let theme = Theme::from_color(app.theme);

    match app.screen {
        Screen::Home => draw_home(frame, app, &theme),
        Screen::PetSelection => draw_pet_selection_overlay(frame, app, &theme),
        Screen::Naming => draw_naming_overlay(frame, app, &theme),
        Screen::Help   => {
            draw_home(frame, app, &theme);
            draw_help_overlay(frame, &theme);
        }
        Screen::LoadSaved => draw_load_saved_overlay(frame, app, &theme),
    }
}

// ─── Main game screen ─────────────────────────────────────────────────────

fn draw_home(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();

    // Root layout: Title | Body | Actions | Log
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title bar
            Constraint::Min(14),     // Pet display
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
    let days = (chrono::Utc::now() - app.pet.created_at).num_days() + 1;
    let title = format!(
        " 🐱 Terminal Pet — \"{}\" (Day {})   Theme: {} {} ",
        app.pet.name,
        days,
        theme.variant.icon(),
        theme.variant.name(),
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

// ── Pet display panel ─────────────────────────────────────────────────────

fn draw_pet_panel(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let visual_state = app.pet.visual_state();
    let art          = current_frame(visual_state, app.anim_tick);

    // Build styled text — every non-empty line gets the primary theme color
    let styled_lines: Vec<Line> = art
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let width = Span::raw(line).width();
            let pad_len = 25_usize.saturating_sub(width);
            // Prepend 6 spaces to shift the cat's core body towards the visual center of the 25-char block
            let padded = format!("      {}{}", line, " ".repeat(pad_len));
            
            Line::from(Span::styled(
                padded,
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect();

    // Calculate dynamic vertical padding to center the art block
    let inner_height = area.height.saturating_sub(2); // subtract borders
    let art_height = styled_lines.len() as u16;
    let top_padding = inner_height.saturating_sub(art_height) / 2;

    let mut final_lines = Vec::new();
    // Add empty lines for top padding
    for _ in 0..top_padding {
        final_lines.push(Line::from(""));
    }
    final_lines.extend(styled_lines);

    let para = Paragraph::new(Text::from(final_lines))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    format!(" {} ", app.pet.mood().label()),
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

// ─── Naming overlay ───────────────────────────────────────────────────────

fn draw_naming_overlay(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();

    // Dim background
    frame.render_widget(Clear, area);

    // Centre box 60% wide, ~12 tall
    let popup = centered_rect(60, 12, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Welcome to Terminal Pet!  🐱",
            Style::default()
                .fg(theme.title)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  What would you like to name your cat?",
            Style::default().fg(theme.message),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  > {}█", app.name_input),
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  [Enter] Confirm   [Backspace] Delete",
            Style::default().fg(theme.muted),
        )),
    ];

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    " 🐾 New Pet ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                )),
        );

    frame.render_widget(Clear, popup);
    frame.render_widget(para, popup);
}

// ─── Pet Selection overlay ────────────────────────────────────────────────

fn draw_pet_selection_overlay(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();

    // Dim background
    frame.render_widget(Clear, area);

    // Centre box 60% wide, ~15 tall
    let popup = centered_rect(60, 15, area);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Welcome to Terminal Pet!  🐱",
            Style::default()
                .fg(theme.title)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Select your pet species:",
            Style::default().fg(theme.message),
        )),
        Line::from(""),
    ];

    let options = [
        "Cat",
        "Dog (Coming soon)",
        "Turtle (Coming soon)",
        "Load saved",
    ];

    for (i, option) in options.iter().enumerate() {
        if i == app.selected_species {
            lines.push(Line::from(Span::styled(
                format!("  > {}", option),
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("    {}", option),
                Style::default().fg(theme.message),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  [Up/Down] Select   [Enter] Confirm",
        Style::default().fg(theme.muted),
    )));

    if let Some(msg) = &app.selection_message {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        )));
    }

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    " 🐾 New Pet ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                )),
        );

    frame.render_widget(Clear, popup);
    frame.render_widget(para, popup);
}

// ─── Load Saved overlay ───────────────────────────────────────────────────

fn draw_load_saved_overlay(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();

    // Dim background
    frame.render_widget(Clear, area);

    // Centre box 60% wide, variable height
    let popup_height = std::cmp::max(15, app.load_options.len() as u16 + 10);
    let popup = centered_rect(60, popup_height, area);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Load Saved Pet  💾",
            Style::default()
                .fg(theme.title)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if app.load_options.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No saved pets found.",
            Style::default().fg(theme.message),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "  Select a pet to load:",
            Style::default().fg(theme.message),
        )));
        lines.push(Line::from(""));

        for (i, save) in app.load_options.iter().enumerate() {
            let days = (chrono::Utc::now() - save.pet.created_at).num_days() + 1;
            let label = format!("{} ({:?} - Day {})", save.pet.name, save.pet.species, days);
            if i == app.selected_load {
                lines.push(Line::from(Span::styled(
                    format!("  > {}", label),
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                )));
            } else {
                lines.push(Line::from(Span::styled(
                    format!("    {}", label),
                    Style::default().fg(theme.message),
                )));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  [Up/Down] Select   [Enter] Confirm   [Del] Delete   [Esc] Back",
        Style::default().fg(theme.muted),
    )));

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    " 💾 Load Pet ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                )),
        );

    frame.render_widget(Clear, popup);
    frame.render_widget(para, popup);
}

// ─── Help overlay ─────────────────────────────────────────────────────────

fn draw_help_overlay(frame: &mut Frame, theme: &Theme) {
    let area  = frame.area();
    let popup = centered_rect(55, 18, area);

    let dim = Style::default().fg(theme.help_text);
    let key = Style::default()
        .fg(theme.primary)
        .add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ── Keybindings ──────────────────────",
            Style::default().fg(theme.accent),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [F]", key), Span::styled("  Feed your cat (10s cooldown)", dim),
        ]),
        Line::from(vec![
            Span::styled("  [P]", key), Span::styled("  Head-pat your cat (5s cooldown)", dim),
        ]),
        Line::from(vec![
            Span::styled("  [Y]", key), Span::styled("  Play with toy (15s cooldown)", dim),
        ]),
        Line::from(vec![
            Span::styled("  [T]", key), Span::styled("  Cycle color theme", dim),
        ]),
        Line::from(vec![
            Span::styled("  [M]", key), Span::styled("  Exit to menu (New Pet)", dim),
        ]),
        Line::from(vec![
            Span::styled("  [H]", key), Span::styled("  Toggle this help screen", dim),
        ]),
        Line::from(vec![
            Span::styled("  [Q]", key), Span::styled("  Quit (auto-saves)", dim),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  ── Bond Decay ───────────────────────",
            Style::default().fg(theme.accent),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Bond decays -0.5 per real minute.",
            dim,
        )),
        Line::from(Span::styled(
            "  Offline decay applies on next launch.",
            dim,
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Press [H] or [Esc] to close",
            Style::default().fg(theme.muted),
        )),
    ];

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(theme.border))
                .title(Span::styled(
                    " 🐱 Help ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                )),
        );

    frame.render_widget(Clear, popup);
    frame.render_widget(para, popup);
}

// ─── Utility ──────────────────────────────────────────────────────────────

/// Return a centred `Rect` that is `percent_x`% of `r`'s width and
/// at most `height` rows tall, vertically centred in `r`.
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect::new(x, y, popup_width.min(r.width), height.min(r.height))
}
