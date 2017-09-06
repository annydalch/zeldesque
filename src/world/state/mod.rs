pub mod scene;

use self::scene::Scene;

pub enum State {
    Gameplay(Scene),
    PreInit,
}
