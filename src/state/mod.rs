pub mod scene;
pub mod menu;
pub mod pause;

use self::scene::Scene;
use self::menu::Menu;
use self::pause::PauseMenu;
use keyboard::Keyboard;
use piston::input::{ButtonArgs, UpdateArgs};

#[derive(Clone)]
pub enum State {
    Gameplay(Scene),
    MainMenu(Menu),
    Pause(PauseMenu, Scene),
    PreInit,
}

pub enum StateChangeRequest {
    NewGame,
    LoadGame,
    Continue,
    SaveGame,
    Pause,
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
