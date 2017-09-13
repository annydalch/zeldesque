mod keyboard;
mod coordinates;
mod color;
mod asset_manager;
mod state;
mod characters;

use self::state::State;
use self::keyboard::Keyboard;
use self::coordinates::Vec2;
use self::asset_manager::TextureManager;
use self::characters::Player;

use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow;
use texture::TextureSettings;
use piston::input::{ButtonArgs, UpdateArgs};

static MENU_FONT: &[u8] = include_bytes!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/font/Megrim.ttf"
    )
);
pub const MENU_FONT_SIZE: u32 = 24;

const STARTING_SCREEN_WIDTH: f64 = 640.0;
const STARTING_SCREEN_HEIGHT: f64 = 480.0;
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

pub struct World {
    keyboard: Keyboard,
    window_size: Vec2,
    textures: TextureManager,
    state: State,
    button_events: Vec<ButtonArgs>,
}

impl World {
    pub fn new() -> Self {
        let keyboard: Keyboard = Keyboard::new();
        let textures = TextureManager::new();

        let state = State::PreInit;
        World {
            keyboard,
            textures,
            state,
            window_size: Vec2 { x: STARTING_SCREEN_WIDTH, y: STARTING_SCREEN_HEIGHT },
            button_events: Vec::new(),
        }
    }

    pub fn update_state(&mut self, args: &UpdateArgs) {
        use self::state::Update;
        use self::state::StateChangeRequest;
        use self::state::menu::Menu;
        use self::state::scene::Scene;
        
        let state_request = match self.state {
            State::Gameplay(ref mut sc) => sc.update(args, &self.keyboard, &mut self.button_events),
            State::MainMenu(ref mut menu) => menu.update(args, &self.keyboard, &mut self.button_events),
            State::PreInit => Some(StateChangeRequest::MainMenu),
            _ => panic!("Bad state!"),
        };
        
        if let Some(target_state) = state_request {
            self.state = match target_state {
                StateChangeRequest::NewGame => State::Gameplay(Scene::new(&mut self.textures)),
                StateChangeRequest::LoadGame => panic!("Not yet implemented"),
                StateChangeRequest::Quit => ::std::process::exit(0),
                StateChangeRequest::MainMenu => State::MainMenu(Menu::new(MENU_FONT_SIZE)),
                _ => panic!("bad state request"),
            }
        }
    }
            

    pub fn run(mut self) {
        use piston::event_loop::{Events, EventSettings};
        use piston::window::{WindowSettings};
        use opengl_graphics::glyph_cache::GlyphCache;
        
        let mut window: GlutinWindow = WindowSettings::new(
            "zeldesque",
            (STARTING_SCREEN_WIDTH as _, STARTING_SCREEN_HEIGHT as _)
        )
            .vsync(true)
            .resizable(false)
            .opengl(OPENGL_VERSION)
            .build()
            .unwrap();

        let mut gl = GlGraphics::new(OPENGL_VERSION);

        let mut events = Events::new(EventSettings::new());

        let mut menu_font = GlyphCache::from_bytes(
            MENU_FONT,
            TextureSettings::new()
        ).unwrap();
        menu_font.preload_printable_ascii(MENU_FONT_SIZE);

        while let Some(event) = events.next(&mut window) {
            use piston::input::Event::*;

            match event {
                Input(input) => {
                    use piston::input::Input::*;

                    match input {
                        Button(args) => {
                            self.keyboard.handle_keypress(&args);
                            self.button_events.push(args);
                        },
                        _ => ()
                    }
                },
                Loop(loop_type) => {
                    use piston::input::Loop::*;
                    match loop_type {
                        Update(args) => self.update_state(&args),
                        Render(args) => {
                            gl.draw(args.viewport(), |ctx, gl| {
                                use graphics::clear;
                                use world::color::*;

                                clear(with_opacity(BLACK, OPAQUE), gl);
                                match self.state {
                                    State::Gameplay(ref mut sc) => sc.draw(gl, ctx.transform),
                                    State::MainMenu(ref mut menu) => menu.draw(&mut menu_font, gl, ctx.transform),
                                    _ => (),
                                }
                            });
                        },
                        _ => (),
                    }
                },
                _ => ()
            }
        }
    }
}
