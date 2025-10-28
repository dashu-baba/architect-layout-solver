use crate::types::{Room, RoomRequirement};
use crate::geometry::Rectangle;

#[derive(Debug, Clone)]
pub struct PositionScore {
    pub total_score: f64,
    pub hard_constraint_score: f64,
    pub soft_preference_score: f64,
    pub space_efficiency_score: f64,
    pub has_violations: bool,
    pub violation_reasons: Vec<String>,
}