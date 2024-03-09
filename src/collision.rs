use crate::vec2d;

const TARGET_REACHING_RADIUS: f64 = 1.0;
const RETRIEVE_SCOUT_RADIUS: f64 = 50.0;
const VIEW_RADIUS: f64 = 200.0;
const FIGHT_RADIUS: f64 = 100.0;

pub enum CollisionType {
    TargetReaching,
    ScoutRetrieving,
    View,
    Fight
}

pub fn are_positions_colliding(position1: &vec2d::Vec2D, position2: &vec2d::Vec2D, collision_type: CollisionType) -> bool {
    let radius = match collision_type {
        CollisionType::TargetReaching => TARGET_REACHING_RADIUS,
        CollisionType::ScoutRetrieving => RETRIEVE_SCOUT_RADIUS,
        CollisionType::View => VIEW_RADIUS,
        CollisionType::Fight => FIGHT_RADIUS
    };
    (position1.x - position2.x).abs() < radius && (position1.y - position2.y).abs() < radius
}