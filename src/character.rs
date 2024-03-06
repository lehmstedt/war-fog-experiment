use crate::vec2d;
use crate::collision;

pub struct Character {
    position: vec2d::Vec2D,
    speed: vec2d::Vec2D,
    is_target_set: bool,
    target_position: vec2d::Vec2D,
    max_speed: f64,
    is_visible: bool
}

pub fn new() -> Character{
    Character {
        position: vec2d::new(),
        speed: vec2d::new(),
        is_target_set: false,
        target_position: vec2d::new(),
        max_speed: 50.0,
        is_visible: false
    }
}

impl Character {

    pub fn update_position(&mut self, dt: &f64){
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;

        if self.has_reached_target(){
            self.unset_target();
        }
    }

    pub fn has_reached_target(&mut self) -> bool{
        collision::are_positions_colliding(&self.position, &self.target_position, collision::CollisionType::TargetReaching)
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

    pub fn get_position(&mut self) -> &vec2d::Vec2D {
        &self.position
    }

    pub fn get_target_position(&mut self) -> &vec2d::Vec2D {
        &self.target_position
    }

    pub fn is_target_set(&mut self) -> bool {
        self.is_target_set
    }

    pub fn set_position(&mut self, position: &vec2d::Vec2D) {
        self.position = *position;
    }

    pub fn set_visible(&mut self, visible: bool){
        self.is_visible = visible
    }

    pub fn is_visible(&mut self) -> bool {
        self.is_visible
    }
}