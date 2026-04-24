// main.rs — Entry point for Terminal Pet.
//
// Responsibilities:
//   1. Set up the terminal (raw mode + alternate screen).
//   2. Install a panic hook to restore the terminal before printing a panic.
//   3. Parse CLI args for initial theme override (--color <theme>).
//   4. Run the app game loop.
//   5. Restore the terminal on exit (normal or error).

use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
pub mod core;
pub mod ui;

use app::App;
use core::event::EventHandler;
use core::theme::ThemeColor;

fn main() -> io::Result<()> {
    // ── Parse optional --color <theme> CLI arg ────────────────────────────
    let initial_theme = parse_color_arg();

    // ── Set up terminal ───────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend  = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ── Panic hook — always restore terminal ──────────────────────────────
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        orig_hook(info);
    }));

    // ── Build app state ───────────────────────────────────────────────────
    let mut app = App::new();

    // Override theme from CLI if provided
    if let Some(theme) = initial_theme {
        app.theme = theme;
    }

    // ── Event handler (4 ticks/second = 250 ms) ───────────────────────────
    let events = EventHandler::new(250);

    // ── Run ───────────────────────────────────────────────────────────────
    let result = app.run(&mut terminal, &events, 250);

    // ── Restore terminal ──────────────────────────────────────────────────
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

// ─── CLI arg parsing ──────────────────────────────────────────────────────

/// Look for `--color <name>` in `std::env::args()`.
/// Returns `None` if not provided or unrecognized.
fn parse_color_arg() -> Option<ThemeColor> {
    let args: Vec<String> = std::env::args().collect();
    let idx = args.iter().position(|a| a == "--color" || a == "-c")?;
    let value = args.get(idx + 1)?;

    match value.to_lowercase().as_str() {
        "red"    => Some(ThemeColor::Red),
        "blue"   => Some(ThemeColor::Blue),
        "green"  => Some(ThemeColor::Green),
        "pink"   => Some(ThemeColor::Pink),
        "yellow" => Some(ThemeColor::Yellow),
        _ => {
            eprintln!(
                "Unknown color '{}'. Valid options: red, blue, green, pink, yellow",
                value
            );
            None
        }
    }
}
