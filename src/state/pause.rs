use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::GlGraphics;
use piston::input::{ButtonArgs, Key, UpdateArgs};
use graphics::math::Matrix2d;
use piston::input::Key::*;
use piston::input::Button::*;
use piston::input::ButtonState::*;

use keyboard::Keyboard;
use super::{StateChangeRequest, Update};

const MENU_MARGINS: (f64, f64) = (100.0, 100.0);
const MENU_SEPERATION: f64 = 50.0;
const PAUSE_MENU_OPACITY: f32 = 0.5;
const MENU_DIMENSIONS: (f64, f64) = (300.0, 300.0);

#[derive(Debug, Copy, Clone)]
pub enum PauseMenuEntry {
    Resume,
    QuitToMainMenu,
    Save,
}

#[derive(Clone)]
pub struct PauseMenu {
    entries: Vec<PauseMenuEntry>,
    pos: usize,
    font_size: u32,
}

fn get_text_from_menu_entry(entry: &PauseMenuEntry) -> &str {
    use self::PauseMenuEntry::*;
    match *entry {
        Resume => "Resume",
        QuitToMainMenu => "Quit to Main Menu",
        Save => "Save Game",
        _ => panic!("Bad menu entry!"),
    }
}

impl PauseMenu {
    pub fn new(font_size: u32) -> Self {
        use self::PauseMenuEntry::*;

        Self {
            entries: vec![Resume, Save, QuitToMainMenu],
            pos: 0,
            font_size,
        }
    }

    pub fn draw(&self, font: &mut GlyphCache, gl: &mut GlGraphics, mut transform: Matrix2d) {
        use color::*;
        use graphics::{text, rectangle};
        use graphics::Transformed;

        transform = transform.trans(MENU_MARGINS.0, MENU_MARGINS.1);
        rectangle(
            with_opacity(BLACK, PAUSE_MENU_OPACITY),
            [0.0, 0.0, MENU_DIMENSIONS.0, MENU_DIMENSIONS.1],
            transform,
            gl
        );

        for (ct, entry) in self.entries.iter().enumerate() {
            transform = transform.trans(0.0, MENU_SEPERATION);
            if ct == self.pos {
                text(
                    with_opacity(WHITE, OPAQUE),
                    self.font_size,
                    get_text_from_menu_entry(entry),
                    font,
                    transform,
                    gl,
                );
            } else {
                text(
                    with_opacity(BLUE, OPAQUE),
                    self.font_size,
                    get_text_from_menu_entry(entry),
                    font,
                    transform,
                    gl
                );
            }
        }
    }

    fn activate_menu_option(&self) -> StateChangeRequest {
        let selected_option = self.entries[self.pos];
        use self::PauseMenuEntry::*;

        match selected_option {
            Resume => StateChangeRequest::Continue,
            QuitToMainMenu => StateChangeRequest::MainMenu,
            Save => StateChangeRequest::SaveGame,
            _ => panic!("Bad menu entry!"),
        }
    }

    fn key_pressed(&mut self, key: Key) -> Option<StateChangeRequest> {
        match key {
            Return => Some(self.activate_menu_option()),
            W | Up => {
                if self.pos == 0 {
                    self.pos = self.entries.len() - 1;
                } else {
                    self.pos -= 1;
                }
                None
            }
            S | Down => {
                if self.pos == self.entries.len() - 1 {
                    self.pos = 0;
                } else {
                    self.pos += 1;
                }
                None
            }
            _ => None,
        }
    }
}

impl Update for PauseMenu {
    fn update(
        &mut self,
        _: &UpdateArgs,
        _: &Keyboard,
        events: &mut Vec<ButtonArgs>,
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
