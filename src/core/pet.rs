// pet.rs — The Pet struct and all game mechanics.
//
// Covers: interactions (feed / pat / play), stat clamping, mood derivation,
// real-time bond decay (including offline catch-up on load), and cooldowns.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ─── Shop Items ───────────────────────────────────────────────────────────

/// Items available in the Shop. All are free; require a 5-day streak to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopItem {
    Coca,
    Popcorn,
    Snack,
    Grass,
}

impl ShopItem {
    pub const ALL: [ShopItem; 4] = [
        ShopItem::Coca,
        ShopItem::Popcorn,
        ShopItem::Snack,
        ShopItem::Grass,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ShopItem::Coca    => "Coca",
            ShopItem::Popcorn => "Popcorn",
            ShopItem::Snack   => "Snack",
            ShopItem::Grass   => "Grass",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            ShopItem::Coca    => "🥤",
            ShopItem::Popcorn => "🍿",
            ShopItem::Snack   => "🍪",
            ShopItem::Grass   => "🌿",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            ShopItem::Coca    => "Refreshing cola  ·  Hunger -20, Happiness +10",
            ShopItem::Popcorn => "Movie-night corn ·  Hunger -25, Happiness +15",
            ShopItem::Snack   => "Crunchy treat    ·  Hunger -15, Bond +5",
            ShopItem::Grass   => "Healthy greens   ·  Hunger -20, Bond +8, Joy +5",
        }
    }
}

// ─── Enums ────────────────────────────────────────────────────────────────

/// Pet species.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Species {
    Cat,
    Dog,
    Turtle,
}

/// Current animation / behavior state of the pet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PetState {
    Idle,
    Eating,
    Playing,
    BeingPatted,
    Sleeping,
    Sad,
}

/// Mood derived from pet stats — shown in the status bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Ecstatic,  // bond ≥ 85
    Happy,     // bond ≥ 60, hunger < 50
    Content,   // bond ≥ 35, hunger < 70
    Hungry,    // hunger ≥ 70 (overrides others)
    Sad,       // happiness < 30
    Lonely,    // bond < 20
}

impl Mood {
    pub fn label(self) -> &'static str {
        match self {
            Mood::Ecstatic => "Ecstatic ✨",
            Mood::Happy    => "Happy 😊",
            Mood::Content  => "Content 😌",
            Mood::Hungry   => "Hungry 😿",
            Mood::Sad      => "Sad 😿",
            Mood::Lonely   => "Lonely 🌧️",
        }
    }
}

// ─── Interaction results ──────────────────────────────────────────────────

/// Outcome of an attempted interaction, including the message to display.
pub struct InteractionResult {
    pub success: bool,
    pub message: String,
}

// ─── Cooldown tracker ─────────────────────────────────────────────────────

/// Tracks the last time a specific interaction was used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooldowns {
    pub last_feed:    Option<DateTime<Utc>>,
    pub last_pat:     Option<DateTime<Utc>>,
    pub last_play:    Option<DateTime<Utc>>,
}

impl Default for Cooldowns {
    fn default() -> Self {
        Self { last_feed: None, last_pat: None, last_play: None }
    }
}

impl Cooldowns {
    fn secs_since(ts: Option<DateTime<Utc>>) -> f64 {
        match ts {
            None => f64::MAX,
            Some(t) => (Utc::now() - t).num_milliseconds() as f64 / 1000.0,
        }
    }
    pub fn feed_ready(&self)  -> bool { Self::secs_since(self.last_feed) >= 10.0 }
    pub fn pat_ready(&self)   -> bool { Self::secs_since(self.last_pat)  >= 5.0 }
    pub fn play_ready(&self)  -> bool { Self::secs_since(self.last_play) >= 15.0 }

    pub fn feed_remaining(&self)  -> f64 { (10.0 - Self::secs_since(self.last_feed)).max(0.0) }
    pub fn pat_remaining(&self)   -> f64 { (5.0  - Self::secs_since(self.last_pat)).max(0.0)  }
    pub fn play_remaining(&self)  -> f64 { (15.0 - Self::secs_since(self.last_play)).max(0.0) }
}

// ─── Pet struct ───────────────────────────────────────────────────────────

/// The virtual pet and all associated game stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    /// Display name chosen by the player.
    pub name: String,

    pub species: Species,

    /// Bond level: 0.0 (estranged) → 100.0 (soulbound). Decays over time.
    pub bond: f64,

    /// Hunger level: 0.0 (full) → 100.0 (starving).
    pub hunger: f64,

    /// Happiness level: 0.0 (miserable) → 100.0 (delighted).
    pub happiness: f64,

    /// Current animation state.
    pub state: PetState,

    /// Timestamp of last *meaningful* interaction (used for offline decay).
    pub last_interaction: DateTime<Utc>,

    /// When this pet was first created.
    pub created_at: DateTime<Utc>,

    /// Per-action cooldown timestamps.
    pub cooldowns: Cooldowns,

    /// How many ticks the current state has been active (for reverting to idle).
    #[serde(default)]
    pub state_ticks_remaining: u32,
}

impl Pet {
    /// Create a brand-new pet with default stats.
    pub fn new(name: String, species: Species) -> Self {
        let now = Utc::now();
        Self {
            name,
            species,
            bond:       60.0,
            hunger:     20.0,
            happiness:  70.0,
            state:      PetState::Idle,
            last_interaction: now,
            created_at:       now,
            cooldowns:        Cooldowns::default(),
            state_ticks_remaining: 0,
        }
    }

    // ─── Derived accessors ────────────────────────────────────────────────

    /// Compute mood from current stats.
    pub fn mood(&self) -> Mood {
        if self.hunger >= 70.0 { return Mood::Hungry; }
        if self.happiness < 30.0 { return Mood::Sad; }
        if self.bond < 20.0 { return Mood::Lonely; }
        if self.bond >= 85.0 { return Mood::Ecstatic; }
        if self.bond >= 60.0 && self.hunger < 50.0 { return Mood::Happy; }
        Mood::Content
    }

    /// Determine the **visual** state for ASCII art (Sad overrides Idle).
    pub fn visual_state(&self) -> PetState {
        if self.state != PetState::Idle {
            return self.state;
        }
        match self.mood() {
            Mood::Sad | Mood::Lonely | Mood::Hungry => PetState::Sad,
            _ => PetState::Idle,
        }
    }

    // ─── Tick update ──────────────────────────────────────────────────────

    /// Called every game tick (250 ms). Updates decay and state transitions.
    /// Returns an optional log message if something noteworthy happened.
    pub fn tick(&mut self) -> Option<String> {
        let mut msg = None;

        // Decay bond slightly each tick (4 ticks/s → each tick = 15s equivalent game minute)
        // Rate: -0.5 per real minute → -0.5/240 per tick
        self.bond     = (self.bond - 0.00208).max(0.0);
        // Hunger increases naturally over time (gets hungrier)
        self.hunger   = (self.hunger + 0.00416).min(100.0);
        // Happiness decays slowly if not high-bond
        if self.bond < 40.0 {
            self.happiness = (self.happiness - 0.00208).max(0.0);
        }

        // State timeout — return to idle after action animation finishes
        if self.state != PetState::Idle && self.state != PetState::Sad {
            if self.state_ticks_remaining > 0 {
                self.state_ticks_remaining -= 1;
            } else {
                self.state = PetState::Idle;
            }
        }

        // Warn on low bond
        if self.bond < 15.0 && self.bond > 14.5 {
            msg = Some(format!("{} misses you deeply... 💔", self.name));
        }

        msg
    }

    /// Apply offline decay based on elapsed time since `last_interaction`.
    /// Called once on load. Capped at 50 decay points max.
    pub fn apply_offline_decay(&mut self, elapsed_minutes: f64) {
        let bond_loss      = (elapsed_minutes * 0.5).min(50.0);
        let hunger_gain    = (elapsed_minutes * 0.3).min(40.0);
        let happiness_loss = (elapsed_minutes * 0.2).min(30.0);

        self.bond      = (self.bond - bond_loss).max(0.0);
        self.hunger    = (self.hunger + hunger_gain).min(100.0);
        self.happiness = (self.happiness - happiness_loss).max(0.0);
    }

    // ─── Interactions ─────────────────────────────────────────────────────

    /// Feed the cat 🍖
    pub fn feed(&mut self) -> InteractionResult {
        if !self.cooldowns.feed_ready() {
            let rem = self.cooldowns.feed_remaining();
            return InteractionResult {
                success: false,
                message: format!(
                    "⏳ {} isn't hungry yet! ({:.0}s cooldown)",
                    self.name, rem
                ),
            };
        }
        self.bond      = (self.bond + 3.0).min(100.0);
        self.hunger    = (self.hunger - 30.0).max(0.0);
        self.happiness = (self.happiness + 5.0).min(100.0);
        self.cooldowns.last_feed = Some(Utc::now());
        self.last_interaction    = Utc::now();
        self.state               = PetState::Eating;
        self.state_ticks_remaining = 8; // ~2 seconds of animation

        InteractionResult {
            success: true,
            message: format!(
                "🍖 You fed {}! Bond +3, Hunger -30",
                self.name
            ),
        }
    }

    /// Head pat the cat 🤚
    pub fn pat(&mut self) -> InteractionResult {
        if !self.cooldowns.pat_ready() {
            let rem = self.cooldowns.pat_remaining();
            return InteractionResult {
                success: false,
                message: format!(
                    "⏳ {} needs a moment! ({:.0}s cooldown)",
                    self.name, rem
                ),
            };
        }
        self.bond      = (self.bond + 5.0).min(100.0);
        self.happiness = (self.happiness + 15.0).min(100.0);
        self.cooldowns.last_pat  = Some(Utc::now());
        self.last_interaction    = Utc::now();
        self.state               = PetState::BeingPatted;
        self.state_ticks_remaining = 6; // ~1.5 seconds

        InteractionResult {
            success: true,
            message: format!(
                "🤚 You gently patted {}! Bond +5, Happiness +15",
                self.name
            ),
        }
    }

    /// Play with the cat 🧶
    pub fn play(&mut self) -> InteractionResult {
        if !self.cooldowns.play_ready() {
            let rem = self.cooldowns.play_remaining();
            return InteractionResult {
                success: false,
                message: format!(
                    "⏳ {} is tired! ({:.0}s cooldown)",
                    self.name, rem
                ),
            };
        }
        self.bond      = (self.bond + 8.0).min(100.0);
        self.hunger    = (self.hunger + 10.0).min(100.0); // playing makes them hungry
        self.happiness = (self.happiness + 25.0).min(100.0);
        self.cooldowns.last_play = Some(Utc::now());
        self.last_interaction    = Utc::now();
        self.state               = PetState::Playing;
        self.state_ticks_remaining = 12; // ~3 seconds

        InteractionResult {
            success: true,
            message: format!(
                "🧶 You played with {}! Bond +8, Happiness +25 (but gained hunger!)",
                self.name
            ),
        }
    }

    // ─── Helpers ──────────────────────────────────────────────────────────

    /// Bond as a u16 percentage for Gauge widgets (0–100).
    pub fn bond_pct(&self) -> u16 { self.bond.round() as u16 }

    /// Hunger as a u16 percentage for Gauge widgets (0–100).
    pub fn hunger_pct(&self) -> u16 { self.hunger.round() as u16 }

    /// Happiness as a u16 percentage for Gauge widgets (0–100).
    pub fn happiness_pct(&self) -> u16 { self.happiness.round() as u16 }

    /// Bond label with colored hearts based on level.
    #[allow(dead_code)]
    pub fn bond_label(&self) -> String {
        let hearts = if self.bond >= 80.0 { "❤️❤️❤️" }
            else if self.bond >= 50.0 { "❤️❤️" }
            else if self.bond >= 20.0 { "❤️" }
            else { "🖤" };
        format!("{} {:.0}%", hearts, self.bond)
    }

    /// How many consecutive days this pet has been alive (1-indexed).
    pub fn streak_days(&self) -> i64 {
        Utc::now().signed_duration_since(self.created_at).num_days() + 1
    }

    /// Feed a shop item to the pet.
    /// Requires a 5-day streak; also shares the regular feed cooldown.
    pub fn feed_shop_item(&mut self, item: ShopItem) -> InteractionResult {
        let days = self.streak_days();
        if days < 5 {
            let remaining = 5 - days;
            return InteractionResult {
                success: false,
                message: format!(
                    "🔒 Shop treats unlock at day 5! ({} day{} to go)",
                    remaining,
                    if remaining == 1 { "" } else { "s" }
                ),
            };
        }

        if !self.cooldowns.feed_ready() {
            let rem = self.cooldowns.feed_remaining();
            return InteractionResult {
                success: false,
                message: format!(
                    "⏳ {} isn't hungry yet! ({:.0}s cooldown)",
                    self.name, rem
                ),
            };
        }

        // Apply item-specific effects
        match item {
            ShopItem::Coca => {
                self.hunger    = (self.hunger - 20.0).max(0.0);
                self.happiness = (self.happiness + 10.0).min(100.0);
            }
            ShopItem::Popcorn => {
                self.hunger    = (self.hunger - 25.0).max(0.0);
                self.happiness = (self.happiness + 15.0).min(100.0);
            }
            ShopItem::Snack => {
                self.hunger    = (self.hunger - 15.0).max(0.0);
                self.bond      = (self.bond + 5.0).min(100.0);
            }
            ShopItem::Grass => {
                self.hunger    = (self.hunger - 20.0).max(0.0);
                self.bond      = (self.bond + 8.0).min(100.0);
                self.happiness = (self.happiness + 5.0).min(100.0);
            }
        }

        self.cooldowns.last_feed  = Some(Utc::now());
        self.last_interaction     = Utc::now();
        self.state                = PetState::Eating;
        self.state_ticks_remaining = 8;

        InteractionResult {
            success: true,
            message: format!(
                "{} You gave {} a {}! {}",
                item.icon(), self.name, item.name(), item.description()
            ),
        }
    }
}
