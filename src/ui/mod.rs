pub mod home;
pub mod menus;
pub mod help;
pub mod utils;

use ratatui::Frame;
use crate::app::{App, Screen};
use crate::core::theme::Theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let theme = Theme::from_color(app.theme);

    match app.screen {
        Screen::Home => home::draw_home(frame, app, &theme),
        Screen::PetSelection => menus::draw_pet_selection_overlay(frame, app, &theme),
        Screen::Naming => menus::draw_naming_overlay(frame, app, &theme),
        Screen::Help => {
            home::draw_home(frame, app, &theme);
            help::draw_help_overlay(frame, &theme);
        }
        Screen::LoadSaved => menus::draw_load_saved_overlay(frame, app, &theme),
    }
}
