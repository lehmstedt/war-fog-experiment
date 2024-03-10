use crate::vec2d;
use crate::collision;

#[derive(PartialEq)]
pub enum CharacterStatus {
    Idle,
    Moving,
    Fighting
}

pub struct Character {
    position: vec2d::Vec2D,
    speed: vec2d::Vec2D,
    target_position: vec2d::Vec2D,
    max_speed: f64,
    is_visible: bool,
    known_enemy_position: vec2d::Vec2D,
    has_discovered_enemy: bool,
    health: f64,
    status: CharacterStatus
}

pub fn new() -> Character{
    Character {
        position: vec2d::new(),
        speed: vec2d::new(),
        target_position: vec2d::new(),
        max_speed: 50.0,
        is_visible: false,
        known_enemy_position: vec2d::new(),
        has_discovered_enemy: false,
        health: 100.0,
        status: CharacterStatus::Idle
    }
}

impl Character {

    pub fn update_position(&mut self, dt: &f64){
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;

        match self.status {
            CharacterStatus::Idle => self.health = num::clamp(self.health + dt, 0.0, 100.0),
            CharacterStatus::Moving | CharacterStatus::Fighting => self.health = num::clamp(self.health - dt, 0.0, 100.0),
        }

        if collision::are_positions_colliding(&self.position, &self.known_enemy_position, collision::CollisionType::View){
            self.has_discovered_enemy = false;
        }

        if self.has_reached_target(){
            self.rest();
        }
    }

    pub fn has_reached_target(&mut self) -> bool{
        collision::are_positions_colliding(&self.position, &self.target_position, collision::CollisionType::Touch)
    }

    pub fn rest(&mut self){
        self.status = CharacterStatus::Idle;
        self.speed.x = 0.0;
        self.speed.y = 0.0;
    }

    pub fn set_target(&mut self, target_position: &vec2d::Vec2D){
        self.status = CharacterStatus::Moving;
        self.target_position.x = target_position.x;
        self.target_position.y = target_position.y;

        let length = ((self.target_position.x - self.position.x).powi(2) + (self.target_position.y - self.position.y).powi(2)).sqrt();
        self.speed.x = (self.target_position.x - self.position.x) / length * self.max_speed;
        self.speed.y = (self.target_position.y - self.position.y) / length * self.max_speed;
    }

    pub fn get_position(&self) -> &vec2d::Vec2D {
        &self.position
    }

    pub fn get_target_position(&self) -> &vec2d::Vec2D {
        &self.target_position
    }

    pub fn set_position(&mut self, position: &vec2d::Vec2D) {
        self.position = *position;
    }

    pub fn set_visible(&mut self, visible: bool){
        self.is_visible = visible
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn get_known_enemy_position(&self) -> &vec2d::Vec2D {
        &self.known_enemy_position
    }

    pub fn discover_enemy(&mut self, enemy_position: &vec2d::Vec2D){
        self.known_enemy_position = *enemy_position;
        self.has_discovered_enemy = true;
    }

    pub fn has_discovered_enemy(&self) -> bool {
        self.has_discovered_enemy
    }

    pub fn hurt(&mut self, health_loss: f64) {
        self.health -= health_loss;
    }

    pub fn get_health(&self) -> &f64{
        &self.health
    }

    pub fn get_status(&self) -> &CharacterStatus {
        &self.status
    }

    pub fn fight(&mut self) {
        self.rest();
        self.status = CharacterStatus::Fighting;
    }
}