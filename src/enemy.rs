use crate::character::{Character, CharacterStatus};
use crate::vec2d;

pub trait Enemy {
    fn new() -> Self;
    fn update(&mut self, dt: &f64);
}

impl Enemy for Character {

    fn new() -> Self {
        Character::new()
    }

    fn update(&mut self, _dt: &f64){
        if *self.get_status() == CharacterStatus::Idle && *self.get_health() > 90.0 {
            self.set_target(&vec2d::Vec2D{ x: (rand::random::<f64>() -0.5) * 1000.0, y: (rand::random::<f64>() - 0.5) * 1000.0});
        }
    }
}