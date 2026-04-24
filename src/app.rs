// app.rs — Application state and the main game loop.
//
// Owns: Pet, current screen, message log, animation tick counter, theme.
// Drives: event polling → game update → render cycle at 4 ticks/second.

use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    event::{AppEvent, EventHandler},
    pet::{Pet, Species},
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
    /// First-launch pet selection.
    PetSelection,
    /// First-launch pet naming input.
    Naming,
    /// Main game screen.
    Home,
    /// Help overlay rendered over Home.
    Help,
    /// Load previously saved pets.
    LoadSaved,
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

    /// Currently selected pet species index in the selection menu.
    pub selected_species: usize,

    /// Temporary message to display on the selection screen.
    pub selection_message: Option<String>,

    /// Available saves for the Load screen.
    pub load_options: Vec<crate::save::SaveFile>,

    /// Currently selected save index.
    pub selected_load: usize,

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
        let (pet, theme, is_new) = match save::load_latest() {
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
                let pet = Pet::new("...".to_string(), Species::Cat);
                (pet, ThemeColor::Blue, true)
            }
        };

        let screen = if is_new { Screen::PetSelection } else { Screen::Home };

        let mut app = App {
            pet,
            theme,
            screen,
            anim_tick:     0,
            messages:      Vec::new(),
            name_input:    String::new(),
            selected_species: 0,
            selection_message: None,
            load_options:  Vec::new(),
            selected_load: 0,
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
        if self.screen == Screen::Home || self.screen == Screen::Help {
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
            Screen::PetSelection => self.handle_selection_key(code),
            Screen::Naming => self.handle_naming_key(code),
            Screen::Home   => self.handle_home_key(code, modifiers),
            Screen::Help   => self.handle_help_key(code),
            Screen::LoadSaved => self.handle_load_saved_key(code),
        }
    }

    // ── Pet Selection screen ──────────────────────────────────────────────

    fn handle_selection_key(&mut self, code: KeyCode) {
        // Clear previous message
        self.selection_message = None;

        match code {
            KeyCode::Up => {
                if self.selected_species > 0 {
                    self.selected_species -= 1;
                } else {
                    self.selected_species = 3; // Wrap to bottom
                }
            }
            KeyCode::Down => {
                if self.selected_species < 3 {
                    self.selected_species += 1;
                } else {
                    self.selected_species = 0; // Wrap to top
                }
            }
            KeyCode::Enter => {
                if self.selected_species == 0 {
                    // Cat is selected, proceed to naming
                    self.screen = Screen::Naming;
                } else if self.selected_species == 3 {
                    // Load saved
                    self.screen = Screen::LoadSaved;
                    self.load_options = save::list_saves();
                    // Sort descending by saved_at
                    self.load_options.sort_by_key(|s| std::cmp::Reverse(s.saved_at));
                    self.selected_load = 0;
                } else {
                    // Dog or Turtle
                    self.selection_message = Some("Not available yet".to_string());
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                if code == KeyCode::Esc && self.pet.name != "..." {
                    self.screen = Screen::Home;
                } else {
                    self.should_quit = true;
                }
            }
            _ => {}
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
                self.pet = Pet::new(name.clone(), Species::Cat);
                self.screen = Screen::Home;
                self.push_message(format!(
                    "🐱 {} has joined your home! Take good care of them~",
                    name
                ));
            }
            KeyCode::Esc => {
                if self.pet.name != "..." {
                    self.screen = Screen::Home;
                } else {
                    // Use default name if the user presses Esc without entering a name
                    if self.name_input.trim().is_empty() {
                        self.name_input = "Whiskers".to_string();
                    }
                    let name = self.name_input.trim().to_string();
                    self.pet = Pet::new(name.clone(), Species::Cat);
                    self.screen = Screen::Home;
                    self.push_message(format!(
                        "🐱 {} has joined your home! Take good care of them~",
                        name
                    ));
                }
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

            // Menu (New Pet)
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if self.pet.name != "..." {
                    save::save(&self.pet, self.theme);
                }
                self.screen = Screen::PetSelection;
                self.selected_species = 0;
                self.name_input.clear();
            }

            _ => {}
        }
    }

    // ── Load Saved screen ─────────────────────────────────────────────────

    fn handle_load_saved_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                if self.selected_load > 0 {
                    self.selected_load -= 1;
                } else if !self.load_options.is_empty() {
                    self.selected_load = self.load_options.len() - 1;
                }
            }
            KeyCode::Down => {
                if !self.load_options.is_empty() {
                    if self.selected_load < self.load_options.len() - 1 {
                        self.selected_load += 1;
                    } else {
                        self.selected_load = 0;
                    }
                }
            }
            KeyCode::Enter => {
                if !self.load_options.is_empty() && self.selected_load < self.load_options.len() {
                    let mut saved = self.load_options.remove(self.selected_load);
                    let mins = save::minutes_since(saved.pet.last_interaction);
                    if mins > 0.5 {
                        saved.pet.apply_offline_decay(mins);
                    }
                    self.pet = saved.pet;
                    self.theme = saved.theme;
                    self.screen = Screen::Home;
                    self.push_message(format!(
                        "Welcome back! {} is happy to see you~ 🐱",
                        self.pet.name
                    ));
                }
            }
            KeyCode::Delete | KeyCode::Backspace => {
                if !self.load_options.is_empty() && self.selected_load < self.load_options.len() {
                    let saved = &self.load_options[self.selected_load];
                    save::delete_save(&saved.pet.name);
                    self.load_options.remove(self.selected_load);
                    
                    if self.selected_load >= self.load_options.len() && self.selected_load > 0 {
                        self.selected_load -= 1;
                    }
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.screen = Screen::PetSelection;
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
