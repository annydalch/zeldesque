extern crate opengl_graphics;
extern crate graphics;
extern crate piston;

use std::rc::Rc;
use world::coordinates::Vec2;
use opengl_graphics::Texture;
use graphics::math::Matrix2d;
use piston::input::UpdateArgs;
use opengl_graphics::GlGraphics;
use world::keyboard::Keyboard;

const UP: f64 = 0.0;
const RIGHT: f64 = 90.0;
const DOWN: f64 = 180.0;
const LEFT: f64 = 270.0;

pub struct Player {
    pub sprite: Rc<Texture>,
    pub pos: Vec2,
    pub dimensions: Vec2,
    pub angle: f64,
}

impl Player {
    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        use graphics::{image, Transformed};
        use std::borrow::Borrow;

        let my_trans = transform.clone()
            .trans(self.pos.x, self.pos.y)
            .rot_deg(self.angle)
            .trans(self.dimensions.x / -2.0, self.dimensions.y / -2.0);

        image(self.sprite.borrow(), my_trans, gl);
    }
    
    pub fn update(&mut self, args: &UpdateArgs, keyboard: &Keyboard) {
    }
}
