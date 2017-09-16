use std::rc::Rc;
use coordinates::{Vec2, Rectangle, Collides};
use opengl_graphics::Texture;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;

#[derive(Clone)]
pub struct Player {
    pub sprite: Rc<Texture>,
    pub rect: Rectangle,
    pub vel: Vec2,
    pub speed: f64,
}

impl Player {
    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        use graphics::{image, Transformed};
        use std::borrow::Borrow;

        let my_trans = transform
            .clone()
            .trans(self.rect.x, self.rect.y)
            .trans(self.rect.width / -2.0, self.rect.height / -2.0);

        image(self.sprite.borrow(), my_trans, gl);
    }

    pub fn adjust_speed(&self) -> Vec2 {
        use std::f64::consts::SQRT_2;
        let speed: f64 = self.speed * SQRT_2 / 2.0;
        Vec2 { x: speed, y: speed }
    }

    pub fn new(sprite: Rc<Texture>, pos: Vec2) -> Self {
        use texture::ImageSize;
        use std::borrow::Borrow;

        let rect = {
            let texture: &Texture = sprite.borrow();
            let (width, height): (u32, u32) = texture.get_size();
            Rectangle {
                width: width as _,
                height: height as _,
                x: pos.x,
                y: pos.y,
            }
        };

        Player {
            sprite,
            rect,
            vel: Vec2 { x: 0.0, y: 0.0 },
            speed: 30.0,
        }
    }
}

impl Collides for Player {
    fn rectangle(&self) -> Rectangle {
        self.rect
    }
}
