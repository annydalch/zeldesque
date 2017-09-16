use std::rc::Rc;
use coordinates::{Vec2, Rectangle, Collides};
use opengl_graphics::Texture;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;

struct Wall {
    rect: Rectangle,
    sprite: Rc<Texture>,
}

impl Wall {
    pub fn new(sprite: Rc<Texture>, pos: Vec2) -> Self {
        use texture::ImageSize;
        use std::borrow::Borrow;

        let rect = {
            let texture: &Texture = sprite.borrow();
            let (width, height) = texture.get_size();
            Rectangle {
                width: width as _,
                height: height as _,
                x: pos.x,
                y: pos.y,
            }
        };

        Wall {
            sprite,
            rect,
        }
    }
}

impl Collides for Wall {
    fn rectangle(&self) -> Rectangle {
        self.rect
    }
}
