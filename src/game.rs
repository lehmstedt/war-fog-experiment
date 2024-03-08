use crate::character;
use crate::scout;
use crate::vec2d;
use crate::collision;

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

    game.enemy.set_position(&vec2d::Vec2D{ x: (rand::random::<f64>() -0.5) * 500.0, y: (rand::random::<f64>() - 0.5) * 500.0});

    return game;   
}

impl Game {
    pub fn update(&mut self, dt: &f64){
        self.player.update_position(dt);
        self.scout.update_position(dt);

        if !self.scout.is_target_set()
            && !self.scout.is_idle()
            && collision::are_positions_colliding(
                self.player.get_position(),
                self.scout.get_position(),
                collision::CollisionType::ScoutRetrieving,
            )
        {
            self.scout.set_idle();
        }

        let is_scout_visible = collision::are_positions_colliding(self.player.get_position(), self.scout.get_position(), collision::CollisionType::View);
        self.scout.set_visible(is_scout_visible);

        if self.scout.has_enemy_position_to_deliver() && collision::are_positions_colliding( self.player.get_position(), self.scout.get_position(), collision::CollisionType::TargetReaching){
            let enemy_position = self.scout.deliver_enemy_position();
            self.player.discover_enemy(enemy_position);
        }
        self.scout.set_visible(is_scout_visible);

        let is_enemy_visible = collision::are_positions_colliding(self.player.get_position(), self.enemy.get_position(), collision::CollisionType::View);
        self.enemy.set_visible(is_enemy_visible);

        if collision::are_positions_colliding(self.scout.get_position(), self.enemy.get_position(), collision::CollisionType::View){
            self.scout.discover_enemy(self.enemy.get_position());
        }
        
    }

    pub fn get_player_position(&self) -> &vec2d::Vec2D{
        self.player.get_position()
    }

    pub fn get_player_target_position(&self) -> &vec2d::Vec2D{
        self.player.get_target_position()
    }

    pub fn get_scout_position(&self) -> &vec2d::Vec2D{
        self.scout.get_position()
    }

    pub fn get_enemy_position(&self) -> &vec2d::Vec2D{
        self.enemy.get_position()
    }

    pub fn is_scout_visible(&self) -> bool {
        !self.scout.is_idle() && self.scout.is_visible()
    }

    pub fn set_player_target(&mut self, target_position: &vec2d::Vec2D){
        self.player.set_target(target_position)
    }

    pub fn set_scout_target(&mut self, target_position: &vec2d::Vec2D){
        if self.scout.is_idle() {
            self.scout.set_position(self.player.get_position());
            self.scout
                .set_target(target_position, self.player.get_position());
        }
    }

    pub fn is_player_target_visible(&self) -> bool {
        self.player.is_target_set()
    }

    pub fn is_enemy_visible(&self) -> bool {
        self.enemy.is_visible()
    }

    pub fn is_enemy_discovered(&self) -> bool {
        self.player.has_discovered_enemy()
    }


}