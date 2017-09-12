use piston::input::{ButtonArgs, Key};
use piston::input::Key::*;
use piston::input::Button::*;
use piston::input::ButtonState::*;

pub struct Keyboard {
    // true = pressed
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            w: false,
            a: false,
            s: false,
            d: false,
        }
    }

    pub fn handle_keypress(&mut self, args: &ButtonArgs) {
        if let Keyboard(key) = args.button {
            match args.state {
                Release => self.key_released(key),
                Press => self.key_pressed(key),
            }
        }
    }

    fn key_pressed(&mut self, key: Key) {
        match key {
            W => self.w = true,
            A => self.a = true,
            S => self.s = true,
            D => self.d = true,
            _ => (),
        }
    }

    fn key_released(&mut self, key: Key) {
        match key {
            W => self.w = false,
            A => self.a = false,
            S => self.s = false,
            D => self.d = false,
            _ => (),
        }
    }
}
