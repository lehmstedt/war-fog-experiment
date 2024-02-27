use crate::character;
use crate::vec2d;

pub struct Scout {
    character: character::Character,
    is_going_to_player: bool,
    player_position: vec2d::Vec2D
}

pub fn new() -> Scout {
    Scout {
        character: character::new(),
        is_going_to_player: false,
        player_position: vec2d::new()
    }

}

impl Scout {

    pub fn update_position(&mut self, dt: &f64){

        let is_target_set = self.character.is_target_set;
        self.character.update_position(&dt);
        
        if is_target_set && !self.character.is_target_set && !self.is_going_to_player {
            self.is_going_to_player = true;
            self.character.set_target(&self.player_position);
        }
    }

    pub fn set_target(&mut self, target_position: &vec2d::Vec2D, player_position: &vec2d::Vec2D){

        self.is_going_to_player = false;
        self.player_position = *player_position;
        self.character.set_target(target_position);
    }

    pub fn is_target_set(&mut self) -> bool{
        self.character.is_target_set
    }

    pub fn get_position(&mut self) -> &vec2d::Vec2D {
        &self.character.position
    }
}