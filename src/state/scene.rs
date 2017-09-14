use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, UpdateArgs};
use graphics::math::Matrix2d;
use opengl_graphics::Texture;
use std::rc::Rc;

use coordinates::{MapCoord, Vec2};
use characters::Player;
use keyboard::Keyboard;
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

    pub fn new(textures: &mut ::asset_manager::TextureManager) -> Self {
        use asset_manager::TextureID::*;
        let player = Player::new(textures.get(PlayerSprite), Vec2 { x: 100.0, y: 100.0 });

        Scene {
            background: textures.get(Background),
            player,
            pos: MapCoord { x: 0, y: 0 },
        }
    }

    fn move_player(&mut self, dt: f64) {
        let player = &mut self.player;
        player.pos += player.vel * dt;
    }

    fn adjust_player_vel(&mut self, keys: &Keyboard) {
        if keys.w && keys.a && (!keys.s) && (!keys.d) {
            // up and left
            self.player.vel = self.player.adjust_speed() * (-1.0, -1.0);
        } else if (!keys.w) && keys.a && keys.s && (!keys.d) {
            // down and left
            self.player.vel = self.player.adjust_speed() * (-1.0, 1.0);
        } else if (!keys.w) && (!keys.a) && keys.s && keys.d {
            // down and right
            self.player.vel = self.player.adjust_speed();
        } else if keys.w && (!keys.a) && (!keys.s) && keys.d {
            // up and right
            self.player.vel = self.player.adjust_speed() * (1.0, -1.0);
        } else if keys.w && (!keys.s) {
            // up
            self.player.vel = Vec2 {
                x: 0.0,
                y: self.player.speed * -1.0,
            };
        } else if keys.a && (!keys.d) {
            // left
            self.player.vel = Vec2 {
                x: self.player.speed * -1.0,
                y: 0.0,
            };
        } else if keys.s && (!keys.w) {
            // down
            self.player.vel = Vec2 {
                x: 0.0,
                y: self.player.speed,
            };
        } else if keys.d && (!keys.a) {
            // right
            self.player.vel = Vec2 {
                x: self.player.speed,
                y: 0.0,
            };
        } else {
            // neutral
            self.player.vel = Vec2 { x: 0.0, y: 0.0 };
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
        self.adjust_player_vel(keyboard);
        self.move_player(args.dt);
        None
    }
}
