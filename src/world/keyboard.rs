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
}
