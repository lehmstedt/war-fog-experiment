use crate::character;
use crate::scout;
use crate::scout::ScoutStatus;
use crate::vec2d;
use crate::collision;
use crate::character::CharacterStatus;

pub struct Game {
    player: character::Character,
    scout: scout::Scout,
    enemy: character::Character
}

pub fn new() -> Game{
    let mut game = Game {
        player: character::new(),
        scout: scout::new(),
        enemy: character::new()
    };

    game.enemy.set_position(&vec2d::Vec2D{ x: (rand::random::<f64>() -0.5) * 1000.0, y: (rand::random::<f64>() - 0.5) * 1000.0});

    return game;   
}

impl Game {
    pub fn update(&mut self, dt: &f64){
        self.player.update_position(dt);
        self.scout.update_position(dt);
        self.enemy.update_position(dt);


        // enemy behaviour
        if *self.enemy.get_status() != CharacterStatus::Moving {
            self.enemy.set_target(&vec2d::Vec2D{ x: (rand::random::<f64>() -0.5) * 1000.0, y: (rand::random::<f64>() - 0.5) * 1000.0});
        }

        // player/scout interaction
        if collision::are_positions_colliding(self.player.get_position(),self.scout.get_position(),collision::CollisionType::View){
            match *self.scout.get_status() {
                ScoutStatus::GoingToPlayer => {
                    self.scout.set_target(self.player.get_position());
                },
                _ => ()
            }
            
        }

        if collision::are_positions_colliding(self.player.get_position(),self.scout.get_position(),collision::CollisionType::Touch){
            match *self.scout.get_status() {
                ScoutStatus::GoingToPlayer => {
                    self.scout.set_idle();

                    if self.scout.has_enemy_position_to_deliver(){
                        let enemy_position = self.scout.deliver_enemy_position();
                        self.player.discover_enemy(enemy_position);
                    }
                },
                _ => ()
            }
            
        }

        let is_scout_visible = collision::are_positions_colliding(self.player.get_position(), self.scout.get_position(), collision::CollisionType::View);
        self.scout.set_visible(is_scout_visible);

        

        let is_enemy_visible = collision::are_positions_colliding(self.player.get_position(), self.enemy.get_position(), collision::CollisionType::View);
        self.enemy.set_visible(is_enemy_visible);
        if is_enemy_visible {
            self.player.discover_enemy(self.enemy.get_position());
        }

        if collision::are_positions_colliding(self.scout.get_position(), self.enemy.get_position(), collision::CollisionType::View){
            self.scout.discover_enemy(self.enemy.get_position());
        }

        // player/enemy interaction

        if collision::are_positions_colliding(self.player.get_position(), self.enemy.get_position(), collision::CollisionType::Fight){
            if *self.player.get_status() == CharacterStatus::Idle {
                self.enemy.hurt(dt); 
            }
            if *self.enemy.get_status() == CharacterStatus::Idle {
                self.player.hurt(dt);
            }
            
            
        }
        
    }

    pub fn get_player_position(&self) -> &vec2d::Vec2D{
        self.player.get_position()
    }

    pub fn get_player_target_position(&self) -> &vec2d::Vec2D{
        self.player.get_target_position()
    }

    pub fn get_enemy_position(&self) -> &vec2d::Vec2D{
        self.enemy.get_position()
    }

    pub fn set_player_target(&mut self, target_position: &vec2d::Vec2D){
        self.player.set_target(target_position)
    }

    pub fn set_scout_mission(&mut self, target_position: &vec2d::Vec2D){
        if *self.scout.get_status() == ScoutStatus::Idle {
            self.scout.set_status(ScoutStatus::GoingToTarget);
            self.scout.set_position(self.player.get_position());
            self.scout.set_mission(target_position, self.player.get_position());
        }
    }

    pub fn is_enemy_visible(&self) -> bool {
        self.enemy.is_visible()
    }

    pub fn is_enemy_discovered(&self) -> bool {
        self.player.has_discovered_enemy()
    }

    pub fn get_discovered_enemy_position(&self) -> &vec2d::Vec2D {
        self.player.get_known_enemy_position()
    }

    pub fn get_player(&self) -> &character::Character {
        &self.player
    }

    pub fn get_enemy(&self) -> &character::Character {
        &self.enemy
    }

    pub fn get_scout(&self) -> &scout::Scout {
        &self.scout
    }


}