#[derive(Copy, Clone)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

pub fn new() -> Vec2D{
    Vec2D {
        x: 0.0,
        y: 0.0
    }
}
