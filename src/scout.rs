use crate::character;
use crate::vec2d;

const SCOUT_SPEED: f64 = 75.0;

#[derive(PartialEq, Debug)]
pub enum ScoutStatus {
    Idle,
    GoingToTarget,
    GoingToPlayer
}

pub struct Scout {
    character: character::Character,
    player_position: vec2d::Vec2D,
    has_enemy_position_to_deliver: bool,
    status: ScoutStatus
}

impl Scout {

    pub fn new() -> Self {
        let mut new_scout = Self {
            character: character::Character::new(),
            player_position: vec2d::new(),
            has_enemy_position_to_deliver: false,
            status: ScoutStatus::Idle
        };

        new_scout.character.set_max_speed(SCOUT_SPEED);

        new_scout
    
    }

    pub fn update(&mut self, dt: &f64){

        self.character.update(&dt);

        if self.character.has_reached_target(){

            match self.status {
                ScoutStatus::GoingToTarget => {
                    self.character.set_target(&self.player_position);
                    self.status = ScoutStatus::GoingToPlayer;
                },
                ScoutStatus::GoingToPlayer => {
                    self.character.rest();
                },
                _ => ()
            }
            
        }
    }

    pub fn set_mission(&mut self, target_position: &vec2d::Vec2D, player_position: &vec2d::Vec2D){
        self.player_position = *player_position;
        self.character.set_target(target_position);
    }

    pub fn set_target(&mut self, target_position: &vec2d::Vec2D){
        self.character.set_target(target_position);
    }

    pub fn get_position(&self) -> &vec2d::Vec2D {
        &self.character.get_position()
    }

    pub fn set_position(&mut self, position: &vec2d::Vec2D){
        self.character.set_position(position);
    }

    pub fn set_idle(&mut self) {
        self.status = ScoutStatus::Idle;
        self.character.rest();
    }

    pub fn set_visible(&mut self, visible: bool){
        self.character.set_visible(visible);
    }

    pub fn is_visible(&self) -> bool{
        self.character.is_visible()
    }

    pub fn discover_enemy(&mut self, enemy_position: &vec2d::Vec2D){
        self.character.discover_enemy(enemy_position);
        self.has_enemy_position_to_deliver = true;
        self.character.set_target(&self.player_position);
    }

    pub fn has_enemy_position_to_deliver(&self) -> bool {
        self.has_enemy_position_to_deliver
    }

    pub fn deliver_enemy_position(&mut self) -> &vec2d::Vec2D {
        self.has_enemy_position_to_deliver = false;
        &self.character.get_known_enemy_position()
    }

    pub fn get_status(&self) -> &ScoutStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: ScoutStatus) {
        self.status = status;
    } 
}