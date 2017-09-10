extern crate opengl_graphics;
extern crate graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs, ButtonArgs};
use graphics::math::Matrix2d;
use opengl_graphics::Texture;
use std::rc::Rc;

use world::coordinates::MapCoord;
use world::characters::Player;
use world::keyboard::Keyboard;
use super::{Update, State};

pub struct Scene {
    pub background: Rc<Texture>,
    pub pos: MapCoord,
    pub player: Player,
}

impl Scene {
    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        use graphics::image;
        use world::color::*;
        use std::borrow::Borrow;

        image(self.background.borrow(), transform, gl);
        self.player.draw(gl, transform);
    }
}

impl Update for Scene {
    fn update(
        &mut self,
        args: &UpdateArgs,
        keyboard: &Keyboard,
        events: &mut Vec<ButtonArgs>
    ) -> Option<State> {
        None
    }
}
        
