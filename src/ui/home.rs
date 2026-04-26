use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Gauge, Padding, Paragraph, Wrap},
};
use crate::app::App;
use crate::core::theme::Theme;
use crate::core::ascii_art::current_frame;

pub fn draw_home(frame: &mut Frame, app: &App, theme: &Theme) {
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
    let days = chrono::Utc::now().signed_duration_since(app.pet.created_at).num_days() + 1;
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
