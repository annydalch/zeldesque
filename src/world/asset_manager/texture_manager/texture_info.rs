extern crate opengl_graphics;
extern crate image as im;
extern crate texture;

use std::rc::{Weak, Rc};
use std::path::Path;
use opengl_graphics::Texture;

#[derive(Debug)]
pub enum TextureStorage {
    FileStorage(&'static Path),
    Inline(&'static [u8]),
}

use self::TextureStorage::*;

pub struct TextureInfo {
    storage: TextureStorage,
    pub width: u32,
    pub height: u32,
    weak_ref: Weak<Texture>,
}

impl TextureInfo {
    pub fn from_path(path: &'static Path, width: u32, height: u32) -> Self {
        TextureInfo {
            storage: FileStorage(path),
            width,
            height,
            weak_ref: Weak::new(),
        }
    }

    pub fn from_data(data: &'static [u8], width: u32, height: u32) -> Self {
        TextureInfo {
            storage: Inline(data),
            width,
            height,
            weak_ref: Weak::new(),
        }
    }

    pub fn get(&mut self) -> Rc<Texture> {
        let opt_rc = self.weak_ref.upgrade();
        if let Some(rc) = opt_rc {
            rc
        } else {
            self.load()
        }
    }

    fn load(&mut self) -> Rc<Texture> {
        use im::ImageBuffer;
        use texture::TextureSettings;
        use self::TextureStorage::*;
        match self.storage {
            FileStorage(_) => {
                panic!("I haven't implemented that yet!");
            },
            Inline(data) => {
                let img = ImageBuffer::from_raw(
                    self.width, self.height,
                    data.to_vec()
                ).unwrap();
                let tex = Texture::from_image(
                    &img,
                    &TextureSettings::new()
                );
                let rc_tex = Rc::new(tex);
                self.weak_ref = Rc::downgrade(&rc_tex);
                rc_tex
            },
        }
    }
}
