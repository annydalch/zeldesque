extern crate glutin_window;
extern crate graphics;
extern crate image as im;
extern crate opengl_graphics;
extern crate piston;
extern crate rusttype;
extern crate texture;

mod world;
mod keyboard;
mod coordinates;
mod color;
mod asset_manager;
mod state;
mod characters;
mod other_entities;

use world::World;

fn main() {
    let game = World::new();
    game.run();
}
