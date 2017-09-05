pub type Color = [f32; 4];
pub type RawColor = [f32; 3];

pub fn with_opacity(color: RawColor, opacity: f32) -> Color {
    [color[0], color[1], color[2], opacity]
}

pub const OPAQUE: f32 = 1.0;

pub const RED: RawColor = [1.0, 0.0, 0.0];
pub const BLUE: RawColor = [0.0, 1.0, 0.0];
pub const GREEN: RawColor = [0.0, 0.0, 1.0];
pub const BLACK: RawColor = [0.0, 0.0, 0.0];
pub const WHITE: RawColor = [1.0, 1.0, 1.0];
