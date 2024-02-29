use crate::character;
use crate::vec2d;
use crate::collision;

pub struct Scout {
    character: character::Character,
    player_position: vec2d::Vec2D,
    is_idle: bool
}

pub fn new() -> Scout {
    Scout {
        character: character::new(),
        is_idle: true,
        player_position: vec2d::new()
    }

}

impl Scout {

    pub fn update_position(&mut self, dt: &f64){

        self.character.update_position(&dt);

        if self.character.has_reached_target(){
            self.character.set_target(&self.player_position);
        }

        if collision::are_positions_colliding(self.character.get_position(), &self.player_position, collision::CollisionType::TargetReaching){
            self.character.unset_target();
        }
    }

    pub fn set_target(&mut self, target_position: &vec2d::Vec2D, player_position: &vec2d::Vec2D){

        self.is_idle = false;
        self.player_position = *player_position;
        self.character.set_target(target_position);
    }

    pub fn get_position(&mut self) -> &vec2d::Vec2D {
        &self.character.get_position()
    }

    pub fn is_idle(&mut self) -> bool {
        self.is_idle
    }

    pub fn set_position(&mut self, position: &vec2d::Vec2D){
        self.character.set_position(position);
    }

    pub fn set_idle(&mut self) {
        self.is_idle = true;
    }

    pub fn is_target_set(&mut self) -> bool {
        self.character.is_target_set()
    }
}