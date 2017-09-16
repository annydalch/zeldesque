use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<Self> for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<(f64, f64)> for Vec2 {
    type Output = Self;
    fn mul(self, other: (f64, f64)) -> Self {
        Self {
            x: self.x * other.0,
            y: self.y * other.1,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MapCoord {
    pub x: usize,
    pub y: usize,
}

impl Add for MapCoord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Self> for MapCoord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn get_pos(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
    pub fn get_dimensions(&self) -> Vec2 {
        Vec2 {
            x: self.width,
            y: self.height,
        }
    }
    pub fn check_collision(a: Self, b: Self) -> bool {
        !(
            (a.x > (b.x + b.width)) ||
                ((a.x + a.width) < b.x) ||
                (a.y > (b.y + b.height)) ||
                ((a.y + a.height) < b.y)
        )
    }
}

pub trait Collides {
    fn rectangle(&self) -> Rectangle;

    fn collides_with<T: Collides>(&self, other: T) -> bool {
        Rectangle::check_collision(self.rectangle(), other.rectangle())
    }
}

