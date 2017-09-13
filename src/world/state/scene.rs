use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, UpdateArgs};
use graphics::math::Matrix2d;
use opengl_graphics::Texture;
use std::rc::Rc;

use world::coordinates::{MapCoord, Vec2};
use world::characters::Player;
use world::keyboard::Keyboard;
use super::{StateChangeRequest, Update};

pub struct Scene {
    pub background: Rc<Texture>,
    pub pos: MapCoord,
    pub player: Player,
}

impl Scene {
    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        use graphics::image;
        use std::borrow::Borrow;

        image(self.background.borrow(), transform, gl);
        self.player.draw(gl, transform);
    }

    pub fn new(textures: &mut ::world::asset_manager::TextureManager) -> Self {
        use world::asset_manager::TextureID::*;
        let player = Player::new(textures.get(PlayerSprite), Vec2 { x: 100.0, y: 100.0 });

        Scene {
            background: textures.get(Background),
            player,
            pos: MapCoord { x: 0, y: 0 },
        }
    }
}

impl Update for Scene {
    fn update(
        &mut self,
        args: &UpdateArgs,
        keyboard: &Keyboard,
        events: &mut Vec<ButtonArgs>,
    ) -> Option<StateChangeRequest> {
        use piston::input::{Button, ButtonState, Key};

        for event in events.drain(..) {
            if let ButtonArgs {
                state: ButtonState::Press,
                button: Button::Keyboard(Key::Escape),
                ..
            } = event
            {
                return Some(StateChangeRequest::MainMenu);
            }
        }
        None
    }
}
