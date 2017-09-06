extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image as im;
extern crate texture;

mod keyboard;
mod coordinates;
mod color;
mod asset_manager;
mod state;
mod characters;

use self::state::State;
use self::state::scene::Scene;
use self::keyboard::Keyboard;
use self::coordinates::{Vec2, MapCoord};
use self::asset_manager::{TextureID, TextureManager};
use self::characters::Player;

use opengl_graphics::{GlGraphics, OpenGL, Texture};
use glutin_window::GlutinWindow;

use std::rc::{Weak, Rc};

const STARTING_SCREEN_WIDTH: f64 = 640.0;
const STARTING_SCREEN_HEIGHT: f64 = 480.0;
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

pub struct World {
    keyboard: Keyboard,
    window_size: Vec2,
    textures: TextureManager,
    state: State,
}

impl World {
    pub fn new() -> Self {
        use std::borrow::Borrow;
        
        let keyboard: Keyboard = Keyboard::new();
        let mut textures = TextureManager::new();

        let state = State::PreInit;
        World {
            keyboard,
            textures,
            state,
            window_size: Vec2 { x: STARTING_SCREEN_WIDTH, y: STARTING_SCREEN_HEIGHT },
        }
    }

    pub fn run(mut self) {
        use piston::event_loop::{Events, EventSettings};
        use piston::window::{WindowSettings};
        use std::borrow::Borrow;
        
        let mut window: GlutinWindow = WindowSettings::new(
            "zeldesque",
            (STARTING_SCREEN_WIDTH as _, STARTING_SCREEN_HEIGHT as _)
        )
            .exit_on_esc(true)
            .vsync(true)
            .resizable(false)
            .opengl(OPENGL_VERSION)
            .build()
            .unwrap();

        let mut gl = GlGraphics::new(OPENGL_VERSION);

        let mut events = Events::new(EventSettings::new());

        while let Some(event) = events.next(&mut window) {
            use piston::input::Event::*;

            match event {
                Input(input) => {
                    use piston::input::Input::*;
                    use piston::input::ButtonArgs;
                    use piston::input::ButtonState::*;
                    use piston::input::Key::*;
                    use piston::input::Button::*;

                    match input {
                        Button(args) => {
                            match args.button {
                                Keyboard(W) => {
                                    match args.state {
                                        Release => self.keyboard.w = false,
                                        Press => self.keyboard.w = true,
                                    }
                                },
                                Keyboard(A) => {
                                    match args.state {
                                        Release => self.keyboard.a = false,
                                        Press => self.keyboard.a = true,
                                    }
                                },
                                Keyboard(S) => {
                                    match args.state {
                                        Relese => self.keyboard.s = false,
                                        Press => self.keyboard.s = true,
                                    }
                                },
                                Keyboard(D) => {
                                    match args.state {
                                        Release => self.keyboard.d = false,
                                        Press => self.keyboard.d = true,
                                    }
                                },
                                _ => (),
                            }
                        },
                        _ => ()
                    }
                },
                Loop(loop_type) => {
                    use piston::input::Loop::*;
                    match loop_type {
                        Update(args) => {
                            match self.state {
                                State::PreInit => {
                                    let scene = Scene {
                                        pos: MapCoord { x: 0, y:0 },
                                        background: self.textures.get(TextureID::Background),
                                        player: Player {
                                            sprite: self.textures.get(TextureID::PlayerSprite),
                                            pos: Vec2 { x: 64.0, y: 64.0 },
                                            dimensions: Vec2 { x: 128.0, y: 128.0 },
                                            angle: 0.0,
                                        },
                                    };
                                    self.state = State::Gameplay(scene);
                                },
                                State::Gameplay(ref mut scene) => scene.update(&args, &self.keyboard),
                            }
                        },
                        Render(args) => {
                            gl.draw(args.viewport(), |ctx, gl| {
                                use graphics::{clear, image};
                                use self::color::*;

                                if let State::Gameplay(ref mut sc) = self.state {
                                    sc.draw(
                                        gl,
                                        ctx.transform
                                    );
                                }
                            });
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }
    }
}
