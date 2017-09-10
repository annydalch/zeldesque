use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::GlGraphics;
use piston::input::{UpdateArgs, ButtonArgs};
use graphics::math::Matrix2d;

use world::keyboard::Keyboard;
use super::{Update, State};

const MENU_SEPERATION: f64 = 40.0;
const MENU_MARGINS: (f64, f64) = (100.0, 100.0);

pub enum MenuEntry {
    NewGame,
    Quit,
}

pub struct Menu {
    entries: Vec<MenuEntry>,
    pos: usize,
    size: u32,
}

impl Menu {
    pub fn new(size: u32) -> Self {
        use self::MenuEntry::*;

        Menu {
            entries: vec![NewGame, Quit],
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
            text(with_opacity(WHITE, OPAQUE), self.size, "sick, brah", font, transform, gl);
        }
    }
}

impl Update for Menu {
    fn update(
        &mut self,
        args: &UpdateArgs,
        keyboard: &Keyboard,
        events: &mut Vec<ButtonArgs>
    ) -> Option<State> {
        None
    }
}
