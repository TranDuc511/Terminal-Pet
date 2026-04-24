use ratatui::{
    Frame,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};
use crate::app::App;
use crate::core::theme::Theme;
use super::utils::centered_rect;

pub fn draw_naming_overlay(frame: &mut Frame, app: &App, theme: &Theme) {
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

pub fn draw_pet_selection_overlay(frame: &mut Frame, app: &App, theme: &Theme) {
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

pub fn draw_load_saved_overlay(frame: &mut Frame, app: &App, theme: &Theme) {
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
            let days = chrono::Utc::now().signed_duration_since(save.pet.created_at).num_days() + 1;
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
