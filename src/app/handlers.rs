use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::{App, Screen};
use crate::core::pet::{Pet, ShopItem, Species};
use crate::core::save;

impl App {
    pub(crate) fn on_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        match self.screen {
            Screen::PetSelection => self.handle_selection_key(code),
            Screen::Naming => self.handle_naming_key(code),
            Screen::Home   => self.handle_home_key(code, modifiers),
            Screen::Help   => self.handle_help_key(code),
            Screen::LoadSaved => self.handle_load_saved_key(code),
            Screen::Shop  => self.handle_shop_key(code),
        }
    }

    fn handle_selection_key(&mut self, code: KeyCode) {
        self.selection_message = None;

        match code {
            KeyCode::Up => {
                if self.selected_species > 0 {
                    self.selected_species -= 1;
                } else {
                    self.selected_species = 3;
                }
            }
            KeyCode::Down => {
                if self.selected_species < 3 {
                    self.selected_species += 1;
                } else {
                    self.selected_species = 0;
                }
            }
            KeyCode::Enter => {
                if self.selected_species == 0 {
                    self.screen = Screen::Naming;
                } else if self.selected_species == 3 {
                    self.screen = Screen::LoadSaved;
                    self.load_options = save::list_saves();
                    self.load_options.sort_by_key(|s| std::cmp::Reverse(s.saved_at));
                    self.selected_load = 0;
                } else {
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

    fn handle_home_key(&mut self, code: KeyCode, _modifiers: KeyModifiers) {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                let result = self.pet.feed();
                self.push_message(result.message);
            }
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
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.theme = self.theme.next();
                self.push_message(format!(
                    "🎨 Theme changed to {}!",
                    self.theme.name()
                ));
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.screen = Screen::Help;
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if self.pet.name != "..." {
                    save::save(&self.pet, self.theme);
                }
                self.screen = Screen::PetSelection;
                self.selected_species = 0;
                self.name_input.clear();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.screen = Screen::Shop;
                self.shop_selected = 0;
                self.shop_message = None;
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                self.music_playing = !self.music_playing;
                if self.music_playing {
                    self.push_message(format!(
                        "🎵 Music on! {} starts vibing~ (bond decay ↓)",
                        self.pet.name
                    ));
                } else {
                    self.push_message(format!(
                        "🔇 Music off. {} settles down.",
                        self.pet.name
                    ));
                }
            }
            _ => {}
        }
    }

    fn handle_shop_key(&mut self, code: KeyCode) {
        const ITEM_COUNT: usize = 4;
        match code {
            KeyCode::Up => {
                if self.shop_selected > 0 {
                    self.shop_selected -= 1;
                } else {
                    self.shop_selected = ITEM_COUNT - 1;
                }
                self.shop_message = None;
            }
            KeyCode::Down => {
                if self.shop_selected < ITEM_COUNT - 1 {
                    self.shop_selected += 1;
                } else {
                    self.shop_selected = 0;
                }
                self.shop_message = None;
            }
            KeyCode::Enter => {
                let item = ShopItem::ALL[self.shop_selected];
                let result = self.pet.feed_shop_item(item);
                if result.success {
                    self.push_message(result.message.clone());
                }
                self.shop_message = Some(result.message);
            }
            KeyCode::Char('s') | KeyCode::Char('S')
            | KeyCode::Char('q') | KeyCode::Char('Q')
            | KeyCode::Esc => {
                self.screen = Screen::Home;
                self.shop_message = None;
            }
            _ => {}
        }
    }

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
}
