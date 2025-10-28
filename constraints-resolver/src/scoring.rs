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

pub fn calculate_hard_constraint_score(has_violations: bool) -> f64 {
    if has_violations {
        return 0.0;
    }
    return 20.0;
}

pub fn check_hard_constraints(
    placed_room: &Room,
    room_req: &RoomRequirement,
    already_placed: &[Room],
    boundary_width: f64,
    boundary_height: f64,
) -> (bool, Vec<String>) {
    let mut violations = Vec::new();
    let room_rect = Rectangle::from_room(placed_room);

    // Must be within boundary
    if !room_rect.is_within_boundary(boundary_width, boundary_height) {
        violations.push("Room is outside the boundary".to_string());
    }

    // Must not overlap with already placed rooms
    for placed in already_placed {
        if room_rect.overlaps_with(&Rectangle::from_room(placed)) {
            violations.push(format!("Room overlaps with already placed room: {}", placed.id));
        }
    }

    // Must touch the exterior wall
    if room_req.has_exterior_wall {
        if !room_rect.touches_exterior_wall(boundary_width, boundary_height) {
            violations.push("Room touches the exterior wall".to_string());
        }
    }

    // Must be adjacent to required rooms
    for adjacent in room_req.adjacent_to.iter() {
        let mut found_adjacent = false;
        for placed in already_placed {
            if placed.id == *adjacent {
                let existing_rect = Rectangle::from_room(placed);
                if room_rect.is_adjacent_to(&existing_rect) {
                    found_adjacent = true;
                    break;
                }
            }
        }

        if !found_adjacent {
            violations.push(format!("Room is not adjacent to required room: {}", adjacent));
        }
    }


    // Cannot be adjacent to forbidden rooms
    for forbidden in room_req.not_adjacent_to.iter() {
        for placed in already_placed {
            if placed.id == *forbidden {
                let existing_rect = Rectangle::from_room(placed);
                if room_rect.is_adjacent_to(&existing_rect) {
                    violations.push(format!("Room is adjacent to forbidden room: {}", forbidden));
                    break;
                }
            }
        }
    }

    (violations.is_empty(), violations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_violations_when_valid_position() {
        // Create a room at (1.0, 1.0) with size 3x3
        let placed_room = Room {
            id: "room1".to_string(),
            x: 1.0,
            y: 1.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room requirement with no special constraints
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        let already_placed = vec![];
        
        let (is_valid, violations) = check_hard_constraints(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,  // boundary width
            10.0,  // boundary height
        );
        
        assert!(is_valid);
        assert_eq!(violations.len(), 0);
    }
}