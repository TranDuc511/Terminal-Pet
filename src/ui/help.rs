use ratatui::{
    Frame,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};
use crate::core::theme::Theme;
use super::utils::centered_rect;

pub fn draw_help_overlay(frame: &mut Frame, theme: &Theme) {
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
