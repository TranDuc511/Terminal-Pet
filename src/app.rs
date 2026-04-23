// app.rs — Application state and the main game loop.
//
// Owns: Pet, current screen, message log, animation tick counter, theme.
// Drives: event polling → game update → render cycle at 4 ticks/second.

use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    event::{AppEvent, EventHandler},
    pet::Pet,
    save,
    theme::ThemeColor,
    ui,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::Stdout;

// ─── Screen enum ──────────────────────────────────────────────────────────

/// Which screen is currently active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// First-launch pet naming input.
    Naming,
    /// Main game screen.
    Home,
    /// Help overlay rendered over Home.
    Help,
}

// ─── App struct ───────────────────────────────────────────────────────────

/// Central application state.
pub struct App {
    /// The virtual pet.
    pub pet: Pet,

    /// Active color theme.
    pub theme: ThemeColor,

    /// Current visible screen.
    pub screen: Screen,

    /// Rolling animation tick counter (incremented each Tick event).
    pub anim_tick: u64,

    /// Scrolling message log (newest last).
    pub messages: Vec<String>,

    /// Text buffer for pet-naming overlay.
    pub name_input: String,

    /// Set to `true` to break the game loop and quit.
    should_quit: bool,

    /// Timestamp of the last explicit auto-save.
    last_autosave: Instant,
}

impl App {
    // ─── Constructor ──────────────────────────────────────────────────────

    /// Initialize app state. Loads save file if present, otherwise starts
    /// the naming screen for first-launch.
    pub fn new() -> Self {
        let (pet, theme, is_new) = match save::load() {
            Some(mut saved) => {
                // Apply offline bond/hunger/happiness decay
                let mins = save::minutes_since(saved.pet.last_interaction);
                if mins > 0.5 {
                    saved.pet.apply_offline_decay(mins);
                }
                (saved.pet, saved.theme, false)
            }
            None => {
                // First launch — placeholder pet until name is entered
                let pet = Pet::new("...".to_string());
                (pet, ThemeColor::Blue, true)
            }
        };

        let screen = if is_new { Screen::Naming } else { Screen::Home };

        let mut app = App {
            pet,
            theme,
            screen,
            anim_tick:     0,
            messages:      Vec::new(),
            name_input:    String::new(),
            should_quit:   false,
            last_autosave: Instant::now(),
        };

        // Welcome back message
        if screen == Screen::Home {
            app.push_message(format!(
                "Welcome back! {} is happy to see you~ 🐱",
                app.pet.name
            ));
        }

        app
    }

    // ─── Main loop ────────────────────────────────────────────────────────

    /// Run the application until the user quits.
    /// `tick_rate_ms`: milliseconds per tick (250 ms → 4 ticks/s).
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        events: &EventHandler,
        _tick_rate_ms: u64,
    ) -> std::io::Result<()> {
        loop {
            // ── Render ────────────────────────────────────────────────────
            terminal.draw(|f| ui::draw(f, self))?;

            // ── Event handling ────────────────────────────────────────────
            match events.next() {
                Ok(AppEvent::Tick) => {
                    self.on_tick();
                }
                Ok(AppEvent::Key(key)) => {
                    self.on_key(key.code, key.modifiers);
                }
                Ok(AppEvent::Resize(_, _)) => {
                    // ratatui handles resize automatically on next draw
                }
                Ok(AppEvent::Mouse(_)) | Err(_) => {}
            }

            // ── Auto-save every 60 seconds ────────────────────────────────
            if self.last_autosave.elapsed() >= Duration::from_secs(60) {
                save::save(&self.pet, self.theme);
                self.last_autosave = Instant::now();
            }

            // ── Quit guard ────────────────────────────────────────────────
            if self.should_quit {
                break;
            }
        }

        // Save on clean exit
        save::save(&self.pet, self.theme);
        Ok(())
    }

    // ─── Tick update ──────────────────────────────────────────────────────

    fn on_tick(&mut self) {
        self.anim_tick = self.anim_tick.wrapping_add(1);

        // Only run pet logic on the main screen
        if self.screen != Screen::Naming {
            if let Some(msg) = self.pet.tick() {
                self.push_message(msg);
            }
        }

        // Slow bond-decay warning — emit every ~60 ticks (15s) if very low
        if self.anim_tick % 240 == 0 && self.pet.bond < 30.0 && self.screen == Screen::Home {
            self.push_message(format!(
                "💔 {}'s bond is fading... spend more time together!",
                self.pet.name
            ));
        }
    }

    // ─── Key dispatch ─────────────────────────────────────────────────────

    fn on_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        match self.screen {
            Screen::Naming => self.handle_naming_key(code),
            Screen::Home   => self.handle_home_key(code, modifiers),
            Screen::Help   => self.handle_help_key(code),
        }
    }

    // ── Naming screen ─────────────────────────────────────────────────────

    fn handle_naming_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) if self.name_input.len() < 20 => {
                self.name_input.push(c);
            }
            KeyCode::Backspace => {
                self.name_input.pop();
            }
            KeyCode::Enter if !self.name_input.trim().is_empty() => {
                let name = self.name_input.trim().to_string();
                self.pet = Pet::new(name.clone());
                self.screen = Screen::Home;
                self.push_message(format!(
                    "🐱 {} has joined your home! Take good care of them~",
                    name
                ));
            }
            KeyCode::Esc => {
                // Use default name if the user presses Esc without entering a name
                if self.name_input.trim().is_empty() {
                    self.name_input = "Whiskers".to_string();
                }
                let name = self.name_input.trim().to_string();
                self.pet = Pet::new(name.clone());
                self.screen = Screen::Home;
                self.push_message(format!(
                    "🐱 {} has joined your home! Take good care of them~",
                    name
                ));
            }
            _ => {}
        }
    }

    // ── Home screen ───────────────────────────────────────────────────────

    fn handle_home_key(&mut self, code: KeyCode, _modifiers: KeyModifiers) {
        match code {
            // Quit
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.should_quit = true;
            }

            // Feed
            KeyCode::Char('f') | KeyCode::Char('F') => {
                let result = self.pet.feed();
                self.push_message(result.message);
            }

            // Pat
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let result = self.pet.pat();
                self.push_message(result.message);
                if result.success {
                    self.push_message(format!(
                        "💗 {} purrs contentedly~",
                        self.pet.name
                    ));
                }
            }

            // Play
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                let result = self.pet.play();
                self.push_message(result.message);
                if result.success {
                    self.push_message(format!(
                        "🧶 {} leaps after the toy! Purr purr~",
                        self.pet.name
                    ));
                }
            }

            // Cycle theme
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.theme = self.theme.next();
                self.push_message(format!(
                    "🎨 Theme changed to {}!",
                    self.theme.name()
                ));
            }

            // Help overlay
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.screen = Screen::Help;
            }

            _ => {}
        }
    }

    // ── Help overlay ──────────────────────────────────────────────────────

    fn handle_help_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('h') | KeyCode::Char('H')
            | KeyCode::Esc | KeyCode::Enter => {
                self.screen = Screen::Home;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    // ─── Message log helper ───────────────────────────────────────────────

    /// Append a message to the log, keeping at most 50 entries.
    pub fn push_message(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 50 {
            self.messages.remove(0);
        }
    }
}
