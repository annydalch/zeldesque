extern crate opengl_graphics;
extern crate graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::Graphics;
use graphics::math::Matrix2d;
use opengl_graphics::Texture;
use world::asset_manager::TextureManager;
use world::coordinates::{Vec2, MapCoord};
use std::rc::Rc;

use world::characters::Player;
use world::keyboard::Keyboard;



pub struct Scene {
    pub background: Rc<Texture>,
    pub pos: MapCoord,
    pub player: Player,
}

impl Scene {
    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        use graphics::{image, clear};
        use world::color::*;
        use std::borrow::Borrow;

        clear(with_opacity(BLACK, OPAQUE), gl);
        image(self.background.borrow(), transform, gl);
        self.player.draw(gl, transform);
    }
    
    pub fn update(&mut self, args: &UpdateArgs, keyboard: &Keyboard) {
        self.player.update(args, keyboard);
    }
}
        
