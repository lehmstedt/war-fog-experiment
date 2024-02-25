use crate::vec2d;

pub struct Character {
    pub position: vec2d::Vec2D,
    speed: vec2d::Vec2D,
    pub is_target_set: bool,
    pub target_position: vec2d::Vec2D,
    max_speed: f64
}

pub fn new() -> Character{
    Character {
        position: vec2d::new(),
        speed: vec2d::new(),
        is_target_set: false,
        target_position: vec2d::new(),
        max_speed: 50.0
    }
}

impl Character {

    pub fn update_position(&mut self, dt: &f64){
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;

        if self.is_target_set && (self.position.x - self.target_position.x).abs() < 0.25 && (self.position.y - self.target_position.y).abs() < 0.25 {
            self.unset_target();
        }
    }

    pub fn unset_target(&mut self){
        self.is_target_set = false;
        self.speed.x = 0.0;
        self.speed.y = 0.0;
    }

    pub fn set_target(&mut self, target_position: &vec2d::Vec2D){
        self.is_target_set = true;
        self.target_position.x = target_position.x;
        self.target_position.y = target_position.y;

        let length = ((self.target_position.x - self.position.x).powi(2) + (self.target_position.y - self.position.y).powi(2)).sqrt();
        self.speed.x = (self.target_position.x - self.position.x) / length * self.max_speed;
        self.speed.y = (self.target_position.y - self.position.y) / length * self.max_speed;
    }
}