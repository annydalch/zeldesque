pub mod scene;
pub mod menu;

use self::scene::Scene;
use self::menu::Menu;
use keyboard::Keyboard;
use piston::input::{ButtonArgs, UpdateArgs};

pub enum State {
    Gameplay(Scene),
    MainMenu(Menu),
    PreInit,
}

pub enum StateChangeRequest {
    NewGame,
    LoadGame,
    Quit,
    MainMenu,
}

pub trait Update {
    fn update(
        &mut self,
        args: &UpdateArgs,
        keyboard: &Keyboard,
        events: &mut Vec<ButtonArgs>,
    ) -> Option<StateChangeRequest>;
}
