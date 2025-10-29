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

pub fn score_position(
    placed_room: &Room,
    room_req: &RoomRequirement,
    already_placed: &[Room],
    boundary_width: f64,
    boundary_height: f64,
) -> PositionScore {
    let (is_valid, violations) = check_hard_constraints(placed_room, room_req, already_placed, boundary_width, boundary_height);
    let has_violations = !is_valid;
    let hard_score = calculate_hard_constraint_score(has_violations);
    let mut soft_score = calculate_soft_preference_score(placed_room, room_req, already_placed, boundary_width, boundary_height);
    let mut efficiency_score = calculate_space_efficiency_score(placed_room, room_req);
    
    // If there are violations, zero out soft scores
    if has_violations {
        soft_score = 0.0;
        efficiency_score = 0.0;
    }

    let mut total_score = hard_score + soft_score + efficiency_score;

    if !has_violations {
        total_score += 5.0;
    }

    PositionScore {
        total_score: total_score,
        hard_constraint_score: hard_score,
        soft_preference_score: soft_score,
        space_efficiency_score: efficiency_score,
        has_violations: has_violations,
        violation_reasons: violations,
    }
}

fn calculate_space_efficiency_score(placed_room: &Room, room_req: &RoomRequirement) -> f64 {
    let actual_area = placed_room.width * placed_room.height;
    let min_area = room_req.min_area;
    let efficiency_ratio = min_area / actual_area;
    let score = efficiency_ratio * 10.0;
    score.min(10.0f64)
}

fn calculate_soft_preference_score(
    placed_room: &Room,
    room_req: &RoomRequirement,
    already_placed: &[Room],
    boundary_width: f64,
    boundary_height: f64,
) -> f64{
    let mut score: f64 = 0.0;
    let room_rect = Rectangle::from_room(placed_room);

    // +5 points for each satisfied adjacency preference
    for adjacent in room_req.adjacent_to.iter() {
        for placed in already_placed {
            if placed.id == *adjacent {
                let existing_rect = Rectangle::from_room(placed);
                if room_rect.is_adjacent_to(&existing_rect) {
                    score += 5.0;
                    break;
                }
            }
        }
    }

    // +3 points if touches external wall (bonus, even if not required)
    if room_rect.touches_exterior_wall(boundary_width, boundary_height) {
        score += 3.0;
    }

    score.min(15.0f64)
}

fn calculate_hard_constraint_score(has_violations: bool) -> f64 {
    if has_violations {
        return 0.0;
    }
    return 20.0;
}

fn check_hard_constraints(
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

    // Must be adjacent to required rooms (only check if those rooms are already placed)
    for adjacent in room_req.adjacent_to.iter() {
        // Check if the required adjacent room has been placed
        let required_room_placed = already_placed.iter().find(|r| r.id == *adjacent);
        
        if let Some(required_room) = required_room_placed {
            // Room has been placed, so check adjacency
            let existing_rect = Rectangle::from_room(required_room);
            if !room_rect.is_adjacent_to(&existing_rect) {
                violations.push(format!("Room is not adjacent to required room: {}", adjacent));
            }
        }
        // If the required room hasn't been placed yet, skip this check
        // (it will be validated when that room is placed later)
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
    // Test 1: test_no_violations_when_valid_position
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

    // Test 2: test_violation_when_outside_boundary
    #[test]
    fn test_violation_when_outside_boundary() {
        // Place a room at (8.0, 8.0) with size 5x5
        let placed_room = Room {
            id: "room1".to_string(),
            x: 8.0,
            y: 8.0,
            width: 5.0,
            height: 5.0,
        };
        
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 25.0,
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
        
        assert!(!is_valid);
        assert!(violations.iter().any(|v| v.contains("boundary")));
    }

    // Test 3: test_violation_when_overlapping
    #[test]
    fn test_violation_when_overlapping() {
        // Place room1 at (2.0, 2.0) size 4x4
        let placed_room = Room {
            id: "room1".to_string(),
            x: 2.0,
            y: 2.0,
            width: 4.0,
            height: 4.0,
        };
        
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 16.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: room2 at (4.0, 4.0) size 4x4 (these overlap!)
        let already_placed = vec![
            Room {
                id: "room2".to_string(),
                x: 4.0,
                y: 4.0,
                width: 4.0,
                height: 4.0,
            }
        ];
        
        let (is_valid, violations) = check_hard_constraints(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,
            10.0,
        );
        
        assert!(!is_valid);
        assert!(violations.iter().any(|v| v.contains("overlap")));
    }

    // Test 4: test_violation_when_missing_required_adjacency
    #[test]
    fn test_violation_when_missing_required_adjacency() {
        // Place room1 at (0.0, 0.0) size 3x3
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room requirement: must be adjacent to "room2"
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["room2".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: room2 at (5.0, 5.0) size 3x3 (not adjacent!)
        let already_placed = vec![
            Room {
                id: "room2".to_string(),
                x: 5.0,
                y: 5.0,
                width: 3.0,
                height: 3.0,
            }
        ];
        
        let (is_valid, violations) = check_hard_constraints(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,
            10.0,
        );
        
        assert!(!is_valid);
        assert!(violations.iter().any(|v| v.contains("room2")));
    }

    // Test 5: test_violation_when_adjacent_to_forbidden_room
    #[test]
    fn test_violation_when_adjacent_to_forbidden_room() {
        // Place room1 at (0.0, 0.0) size 3x3
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room requirement: cannot be adjacent to "room2"
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec!["room2".to_string()],
            has_exterior_wall: false,
        };
        
        // Already placed: room2 at (3.0, 0.0) size 3x3 (they ARE adjacent!)
        let already_placed = vec![
            Room {
                id: "room2".to_string(),
                x: 3.0,
                y: 0.0,
                width: 3.0,
                height: 3.0,
            }
        ];
        
        let (is_valid, violations) = check_hard_constraints(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,
            10.0,
        );
        
        assert!(!is_valid);
        assert!(violations.iter().any(|v| v.contains("forbidden")));
    }

    // Test 6: test_hard_constraint_score_zero_when_violations
    #[test]
    fn test_hard_constraint_score_zero_when_violations() {
        let score = calculate_hard_constraint_score(true);
        assert_eq!(score, 0.0);
    }

    // Test 7: test_hard_constraint_score_full_when_no_violations
    #[test]
    fn test_hard_constraint_score_full_when_no_violations() {
        let score = calculate_hard_constraint_score(false);
        assert_eq!(score, 20.0);
    }

    // Test 8: test_soft_score_bonus_for_satisfied_adjacency
    #[test]
    fn test_soft_score_bonus_for_satisfied_adjacency() {
        // Place room1 at (0.0, 0.0) size 3x3
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room requirement: adjacent_to = ["room2"]
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["room2".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: room2 at (3.0, 0.0) size 3x3 (adjacent!)
        let already_placed = vec![
            Room {
                id: "room2".to_string(),
                x: 3.0,
                y: 0.0,
                width: 3.0,
                height: 3.0,
            }
        ];
        
        let score = calculate_soft_preference_score(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,
            10.0,
        );
        
        assert!(score >= 5.0, "Expected score >= 5.0, got {}", score);
    }

    // Test 9: test_soft_score_bonus_for_external_wall
    #[test]
    fn test_soft_score_bonus_for_external_wall() {
        // Place room at (0.0, 2.0) size 3x3 (touches left wall)
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 2.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room requirement: adjacent_to = [] (no adjacency requirements)
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: empty
        let already_placed = vec![];
        
        let score = calculate_soft_preference_score(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,
            10.0,
        );
        
        assert_eq!(score, 3.0, "Expected score 3.0 for external wall bonus, got {}", score);
    }

    // Test 10: test_soft_score_capped_at_15
    #[test]
    fn test_soft_score_capped_at_15() {
        // Place room at (0.0, 0.0) size 3x3
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room requirement: adjacent_to = ["room2", "room3", "room4"] (many adjacencies)
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["room2".to_string(), "room3".to_string(), "room4".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: room2 at (3.0, 0.0), room3 at (0.0, 3.0), room4 at (3.0, 3.0) (all adjacent)
        let already_placed = vec![
            Room {
                id: "room2".to_string(),
                x: 3.0,
                y: 0.0,
                width: 3.0,
                height: 3.0,
            },
            Room {
                id: "room3".to_string(),
                x: 0.0,
                y: 3.0,
                width: 3.0,
                height: 3.0,
            },
        ];
        
        let score = calculate_soft_preference_score(
            &placed_room,
            &room_req,
            &already_placed,
            10.0,
            10.0,
        );
        
        assert!(score <= 15.0, "Expected score <= 15.0 (capped), got {}", score);
    }

    // Test 11: test_space_efficiency_perfect_when_exact_area
    #[test]
    fn test_space_efficiency_perfect_when_exact_area() {
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 4.0,
            height: 5.0,  // area = 20.0
        };
        
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        let score = calculate_space_efficiency_score(&placed_room, &room_req);
        assert_eq!(score, 10.0);  // 20/20 * 10 = 10.0
    }

    // Test 12: test_space_efficiency_lower_when_oversized
    #[test]
    fn test_space_efficiency_lower_when_oversized() {
        // Room: 5.0 x 5.0 (area = 25.0)
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 5.0,  // area = 25.0
        };
        
        // Min area: 20.0
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Score should be: 20.0 / 25.0 * 10.0 = 8.0
        let score = calculate_space_efficiency_score(&placed_room, &room_req);
        assert_eq!(score, 8.0, "Expected score 8.0, got {}", score);
    }

    // Test 13: test_score_position_perfect_score
    #[test]
    fn test_score_position_perfect_score() {
        // Room at (0.0, 0.0) size 4.0 x 5.0 (area = 20.0)
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 4.0,
            height: 5.0,
        };
        
        // Room req: min_area = 20.0, has_exterior_wall = true, no adjacency requirements
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: true,
        };
        
        // Already placed: empty
        let already_placed = vec![];
        
        // Boundary: 10x10
        let score = score_position(&placed_room, &room_req, &already_placed, 10.0, 10.0);
        
        // Expected scores:
        // Hard: 20.0 (no violations)
        // Soft: 3.0 (external wall bonus)
        // Efficiency: 10.0 (perfect area match)
        // Base: 5.0 (no violations)
        // Total: 38.0
        assert_eq!(score.total_score, 38.0, "Expected total score 38.0, got {}", score.total_score);
        assert!(!score.has_violations);
        assert!(score.violation_reasons.is_empty());
    }

    // Test 14: test_score_position_zero_when_violations
    #[test]
    fn test_score_position_zero_when_violations() {
        // Room at (8.0, 8.0) size 5.0 x 5.0 (exceeds boundary)
        let placed_room = Room {
            id: "room1".to_string(),
            x: 8.0,
            y: 8.0,
            width: 5.0,
            height: 5.0,
        };
        
        // Room req: min_area = 25.0
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 25.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: empty
        let already_placed = vec![];
        
        // Boundary: 10x10
        let score = score_position(&placed_room, &room_req, &already_placed, 10.0, 10.0);
        
        // Expected:
        // Hard: 0.0 (violations)
        // Soft: 0.0 (zeroed due to violations)
        // Efficiency: 0.0 (zeroed due to violations)
        // Total: 0.0
        assert_eq!(score.total_score, 0.0, "Expected total score 0.0, got {}", score.total_score);
        assert!(score.has_violations);
        assert!(score.violation_reasons.len() > 0);
    }

    // Test 15: test_score_position_with_adjacency_bonus
    #[test]
    fn test_score_position_with_adjacency_bonus() {
        // Room1 at (0.0, 0.0) size 3.0 x 3.0
        let placed_room = Room {
            id: "room1".to_string(),
            x: 0.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
        };
        
        // Room req: min_area = 9.0, adjacent_to = ["room2"]
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["room2".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: room2 at (3.0, 0.0) size 3.0 x 3.0 (adjacent!)
        let already_placed = vec![
            Room {
                id: "room2".to_string(),
                x: 3.0,
                y: 0.0,
                width: 3.0,
                height: 3.0,
            }
        ];
        
        // Boundary: 10x10
        let score = score_position(&placed_room, &room_req, &already_placed, 10.0, 10.0);
        
        // Expected:
        // Hard: 20.0
        // Soft: 8.0 (5 for adjacency + 3 for external wall)
        // Efficiency: 10.0 (exact area)
        // Base: 5.0
        // Total: 43.0
        assert_eq!(score.total_score, 43.0, "Expected total score 43.0, got {}", score.total_score);
        assert!(score.soft_preference_score >= 5.0, "Expected soft score >= 5.0, got {}", score.soft_preference_score);
    }

    // Test 16: test_score_position_lower_efficiency_when_oversized
    #[test]
    fn test_score_position_lower_efficiency_when_oversized() {
        // Room at (1.0, 1.0) size 5.0 x 5.0 (area = 25.0)
        let placed_room = Room {
            id: "room1".to_string(),
            x: 1.0,
            y: 1.0,
            width: 5.0,
            height: 5.0,
        };
        
        // Room req: min_area = 20.0
        let room_req = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // Already placed: empty
        let already_placed = vec![];
        
        // Boundary: 10x10
        let score = score_position(&placed_room, &room_req, &already_placed, 10.0, 10.0);
        
        // Expected efficiency: 20.0 / 25.0 * 10.0 = 8.0
        assert_eq!(score.space_efficiency_score, 8.0, "Expected efficiency score 8.0, got {}", score.space_efficiency_score);
        // Total: 20.0 (hard) + 0.0 (soft) + 8.0 (efficiency) + 5.0 (base) = 33.0
        assert_eq!(score.total_score, 33.0, "Expected total score 33.0, got {}", score.total_score);
    }
}