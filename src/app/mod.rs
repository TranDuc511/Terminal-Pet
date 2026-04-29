// app.rs — Application state and the main game loop.
//
// Owns: Pet, current screen, message log, animation tick counter, theme.
// Drives: event polling → game update → render cycle at 4 ticks/second.

use std::time::{Duration, Instant};


use crate::core::{
    event::{AppEvent, EventHandler},
    pet::{Pet, Species},
    save,
    theme::ThemeColor,
};
use crate::ui;
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
    /// Shop overlay rendered over Home.
    Shop,
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
    pub load_options: Vec<crate::core::save::SaveFile>,

    /// Currently selected save index.
    pub selected_load: usize,

    /// Currently highlighted item in the shop.
    pub shop_selected: usize,

    /// Feedback message shown inside the shop overlay.
    pub shop_message: Option<String>,

    /// Whether the user is currently playing music (toggles wave visualiser + reduces bond decay).
    pub music_playing: bool,

    /// Set to `true` to break the game loop and quit.
    pub(crate) should_quit: bool,

    /// Timestamp of the last explicit auto-save.
    pub(crate) last_autosave: Instant,
}

mod handlers;

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
            shop_selected: 0,
            shop_message:  None,
            music_playing: false,
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
            if let Some(msg) = self.pet.tick(self.music_playing) {
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

        // Music active reminder every ~120 ticks (30s)
        if self.music_playing && self.anim_tick % 120 == 60 && self.screen == Screen::Home {
            self.push_message(format!(
                "🎵 {} vibes to the music~ bond decay is slower!",
                self.pet.name
            ));
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
