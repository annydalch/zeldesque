use std::rc::Rc;
use world::coordinates::Vec2;
use opengl_graphics::Texture;
use graphics::math::Matrix2d;
use piston::input::UpdateArgs;
use opengl_graphics::GlGraphics;
use world::keyboard::Keyboard;

pub struct Player {
    pub sprite: Rc<Texture>,
    pub pos: Vec2,
    pub dimensions: Vec2,
}

impl Player {
    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        use graphics::{image, Transformed};
        use std::borrow::Borrow;

        let my_trans = transform.clone()
            .trans(self.pos.x, self.pos.y)
            .trans(self.dimensions.x / -2.0, self.dimensions.y / -2.0);

        image(self.sprite.borrow(), my_trans, gl);
    }

    pub fn new(sprite: Rc<Texture>, pos: Vec2) -> Self {
        use texture::ImageSize;
        use std::borrow::Borrow;
        
        let dimensions = {
            let texture: &Texture = sprite.borrow();
            let (width, height): (u32, u32) = texture.get_size();
            Vec2 { x: width as _, y: height as _ }
        };

        Player {
            sprite,
            pos,
            dimensions,
        }
    }
    
    pub fn update(&mut self, _: &UpdateArgs, _: &Keyboard) {
    }
}
