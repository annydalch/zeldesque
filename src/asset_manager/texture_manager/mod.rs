extern crate image as im;
extern crate opengl_graphics;
extern crate texture;

mod texture_info;

use std::rc::Rc;
use opengl_graphics::Texture;
use self::texture_info::TextureInfo;

static BACKGROUND_SPRITE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/test_3.data"
));
// To export these files from GIMP, Export As => .data, select RGBX

static PLAYER_SPRITE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/test_pc.data"
));

#[derive(Copy, Clone, Debug)]
pub enum TextureID {
    Background,
    PlayerSprite,
    DummyTexture,
}

const NUMBER_OF_TEXTURES: usize = TextureID::DummyTexture as usize + 1;

pub struct TextureManager {
    textures: [TextureInfo; NUMBER_OF_TEXTURES],
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: [
                TextureInfo::from_data(BACKGROUND_SPRITE, 600, 600),
                TextureInfo::from_data(PLAYER_SPRITE, 128, 128),
                TextureInfo::from_data(&[], 0, 0),
            ],
        }
    }

    pub fn get(&mut self, texture_id: TextureID) -> Rc<Texture> {
        self.textures[texture_id as usize].get()
    }
}
