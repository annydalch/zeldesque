extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image as im;
extern crate texture;

mod keyboard;
mod coordinates;
mod color;

use self::keyboard::Keyboard;
use self::coordinates::Vec2;

use opengl_graphics::{GlGraphics, OpenGL, Texture};
use glutin_window::GlutinWindow;

use std::rc::{Weak, Rc};

const STARTING_SCREEN_WIDTH: f64 = 640.0;
const STARTING_SCREEN_HEIGHT: f64 = 480.0;
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

static BACKGROUND_SPRITE: &[u8] = include_bytes!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/test_3.data"
    )
);
// To export these files from GIMP, Export As => .data, select RGBX

pub struct World {
    keyboard: Keyboard,
    window_size: Vec2,
    background_texture: Weak<Texture>,
}

pub enum TextureID {
    Background,
    Character,
}

impl World {
    fn load_texture(&mut self, texture_id: TextureID) -> Rc<Texture> {
        use im::ImageBuffer;
        use texture::TextureSettings;
        use self::TextureID::*;
        match texture_id {
            Background => {
                let opt_rc = self.background_texture.upgrade();
                if let Some(rc) = opt_rc {
                    rc
                } else {
                    let background_sprite = ImageBuffer::from_raw(
                        600, 600,
                        BACKGROUND_SPRITE.to_vec()
                    ).unwrap();

                    let background_texture = Texture::from_image(
                        &background_sprite,
                        &TextureSettings::new()
                    );
                    let bg_texture = Rc::new(background_texture);
                    self.background_texture = Rc::downgrade(&bg_texture);
                    bg_texture
                }
            },
            _ => panic!("Undefined texture"),
        }
    }
    
    fn get_texture<'a>(&'a mut self, texture_id: TextureID) -> Rc<Texture> {
        use self::TextureID::*;
        match texture_id {
            Background => {
                let opt_rc = self.background_texture.upgrade();
                if let Some(rc) = opt_rc {
                    rc
                } else {
                    self.load_texture(Background)
                }
            },
            _ => {
                panic!("Undefined texture");
            }
        }

    }

    pub fn new() -> Self {
        let keyboard: Keyboard = Keyboard::new();

        World {
            keyboard,
            background_texture: Weak::new(),
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

        let bg_texture_rc = self.get_texture(TextureID::Background);

        let bg_texture = bg_texture_rc.borrow();

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
                        Button(ButtonArgs { state: Press, button: Keyboard(Q), .. }) => {
                            println!("Sick keyinput, dude!");
                        },
                        _ => ()
                    }
                },
                Loop(loop_type) => {
                    use piston::input::Loop::*;
                    match loop_type {
                        Render(args) => {
                            gl.draw(args.viewport(), |ctx, gl| {
                                use graphics::{clear, image};
                                use self::color::*;
                                                                
                                clear(with_opacity(WHITE, OPAQUE), gl);
                                image(bg_texture, ctx.transform, gl);
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
