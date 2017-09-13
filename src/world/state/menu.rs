use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::GlGraphics;
use piston::input::{UpdateArgs, ButtonArgs, Key};
use graphics::math::Matrix2d;
use piston::input::Key::*;
use piston::input::Button::*;
use piston::input::ButtonState::*;

use world::keyboard::Keyboard;
use super::{Update, StateChangeRequest};

const MENU_SEPERATION: f64 = 40.0;
const MENU_MARGINS: (f64, f64) = (100.0, 100.0);

#[derive(Copy, Clone, Debug)]
pub enum MenuEntry {
    NewGame,
    LoadGame,
    Quit,
}

pub struct Menu {
    entries: Vec<MenuEntry>,
    pos: usize,
    size: u32,
}

fn get_text_from_menu_entry(entry: &MenuEntry) -> &str {
    use self::MenuEntry::*;
    match *entry {
        NewGame => "New Game",
        LoadGame => "Load Game",
        Quit => "Quit",
        _ => panic!("Bad menu entry!"),
    }
}

impl Menu {
    pub fn new(size: u32) -> Self {
        use self::MenuEntry::*;

        Menu {
            entries: vec![NewGame, LoadGame, Quit],
            pos: 0,
            size,
        }
    }

    pub fn draw(&self, font: &mut GlyphCache, gl: &mut GlGraphics, mut transform: Matrix2d) {
        use world::color::*;
        use graphics::text;
        use graphics::Transformed;

        transform = transform.trans(MENU_MARGINS.0, MENU_MARGINS.1);

        for (ct, entry) in self.entries.iter().enumerate() {
            transform = transform.trans(0.0, MENU_SEPERATION);
            if ct == self.pos {
                text(with_opacity(WHITE, OPAQUE), self.size, get_text_from_menu_entry(entry), font, transform, gl);
            } else {
                text(with_opacity(BLUE, OPAQUE), self.size, get_text_from_menu_entry(entry), font, transform, gl);
            }
        }
    }
    
    fn activate_menu_option(&self) -> StateChangeRequest {
        let selected_option = self.entries[self.pos];
        use self::MenuEntry::*;
        
        match selected_option {
            NewGame => StateChangeRequest::NewGame,
            LoadGame => StateChangeRequest::LoadGame,
            Quit => StateChangeRequest::Quit,
            _ => panic!("invalid menu option"),
        }
    }
    
    fn key_pressed(&mut self, key: Key) -> Option<StateChangeRequest> {
        match key {
            Return => {
                Some(self.activate_menu_option())
            },
            W | Up => {
                if self.pos == 0 {
                    self.pos = self.entries.len() - 1;
                } else {
                    self.pos -= 1;
                }
                None
            },
            S | Down => {
                if self.pos == self.entries.len() - 1 {
                    self.pos = 0;
                } else {
                    self.pos += 1;
                }
                None
            },
            _ => None,
        }
    }
}

impl Update for Menu {
    fn update(
        &mut self,
        _: &UpdateArgs,
        _: &Keyboard,
        events: &mut Vec<ButtonArgs>
    ) -> Option<StateChangeRequest> {
        for event in events.drain(..) {
            if let Keyboard(key) = event.button {
                match event.state {
                    Press => if let Some(s) = self.key_pressed(key) {
                        return Some(s);
                    },
                    _ => (),
                }
            }
        }
        None
    }
}
