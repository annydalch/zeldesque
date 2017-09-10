extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image as im;
extern crate texture;
extern crate graphics;
extern crate rusttype;

mod world;

use world::World;

fn main() {
    let mut game = World::new();
    game.run();
}
