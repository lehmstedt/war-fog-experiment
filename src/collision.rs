use crate::vec2d;

const TOUCH_RADIUS: f64 = 10.0;
const VIEW_RADIUS: f64 = 200.0;
const FIGHT_RADIUS: f64 = 100.0;

pub enum CollisionType {
    Touch,
    View,
    Fight
}

pub fn are_positions_colliding(position1: &vec2d::Vec2D, position2: &vec2d::Vec2D, collision_type: CollisionType) -> bool {
    let radius = match collision_type {
        CollisionType::Touch => TOUCH_RADIUS,
        CollisionType::View => VIEW_RADIUS,
        CollisionType::Fight => FIGHT_RADIUS
    };
    (position1.x - position2.x).abs() < radius && (position1.y - position2.y).abs() < radius
}