use crate::{
    candidate_generation::generate_candidate_positions,
    room_ordering::order_rooms_by_constraints,
    scoring::score_position,
    types::{Room, RoomRequirement},
};

pub struct LayoutSolution {
    pub rooms: Vec<Room>,
    pub total_score: f64,
    pub is_valid: bool,
}

#[derive(Debug, Clone)]
pub enum SolverError {
    NoSolutionFound(String),
}

pub fn solve_layout(
    room_requirements: Vec<RoomRequirement>,
    boundary_width: f64,
    boundary_height: f64,
) -> Result<LayoutSolution, SolverError> {
    let ordered_rooms = order_rooms_by_constraints(room_requirements);

    let already_placed: Vec<Room> = Vec::new();

    let solution = solve_recursive(
        &ordered_rooms,
        already_placed,
        boundary_width,
        boundary_height,
    );

    match solution {
        Some(placed_rooms) => {
            // Calculate total score by scoring each placed room
            let mut total_score = 0.0;

            for (i, room) in placed_rooms.iter().enumerate() {
                let room_req = ordered_rooms
                    .iter()
                    .find(|r| r.id == room.id)
                    .expect(&format!(
                        "Room requirement not found for placed room: {}",
                        room.id
                    ));

                // Get all rooms placed before this one
                let already_placed_before = placed_rooms[..i].to_vec();

                let score = score_position(
                    room,
                    room_req,
                    &already_placed_before,
                    boundary_width,
                    boundary_height,
                );

                total_score += score.total_score;
            }

            Ok(LayoutSolution {
                rooms: placed_rooms,
                total_score: total_score,
                is_valid: true,
            })
        }
        None => Err(SolverError::NoSolutionFound(
            "No solution found".to_string(),
        )),
    }
}

fn solve_recursive(
    remaining_rooms: &[RoomRequirement],
    already_placed: Vec<Room>,
    boundary_width: f64,
    boundary_height: f64,
) -> Option<Vec<Room>> {
    // BASE CASE: No more rooms to place
    if remaining_rooms.is_empty() {
        return Some(already_placed);
    }

    // RECURSIVE CASE: Place the next room
    let current_room = &remaining_rooms[0];
    let remaining_rooms = &remaining_rooms[1..];

    // Generate all candidate positions for current room
    let candidates = generate_candidate_positions(current_room, boundary_width, boundary_height);

    // Score and sort candidates (best first)
    let mut scored_candidates = Vec::new();
    for candidate in candidates {
        let score = score_position(
            &candidate,
            current_room,
            &already_placed,
            boundary_width,
            boundary_height,
        );

        if !score.has_violations {
            scored_candidates.push((score, candidate));
        }
    }

    // Sort by total_score descending
    scored_candidates.sort_by(|a, b| {
        b.0.total_score
            .partial_cmp(&a.0.total_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Try each valid candidate (best first)
    for (_score, candidate) in scored_candidates {
        // Make a new placement list with this candidate
        let mut new_already_placed = already_placed.clone();
        new_already_placed.push(candidate);

        // Recursively try to place remaining rooms
        let result = solve_recursive(
            remaining_rooms,
            new_already_placed,
            boundary_width,
            boundary_height,
        );

        // If successful, return the solution
        if result.is_some() {
            return result;
        }

        // Otherwise, backtrack and try next candidate
    }

    // If no valid candidate found, return None
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rectangle;

    // Test 1: test_solve_simple_layout_two_rooms
    #[test]
    fn test_solve_simple_layout_two_rooms() {
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        let room2 = RoomRequirement {
            id: "room2".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        let result = solve_layout(
            vec![room1, room2],
            10.0, // boundary width
            10.0, // boundary height
        );

        assert!(result.is_ok());
        let solution = result.unwrap();
        assert_eq!(solution.rooms.len(), 2);
        assert!(solution.is_valid);
    }

    // Test 2: test_solve_fails_when_impossible
    #[test]
    fn test_solve_fails_when_impossible() {
        // Create 2 rooms, each needing min_area = 60.0
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 60.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        let room2 = RoomRequirement {
            id: "room2".to_string(),
            min_area: 60.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        // Boundary: 10.0 × 10.0 (total area = 100, but can't fit both efficiently)
        let result = solve_layout(vec![room1, room2], 10.0, 10.0);

        assert!(result.is_err());
    }

    // Test 3: test_solve_with_adjacency_requirement
    #[test]
    fn test_solve_with_adjacency_requirement() {
        // Room1: min_area = 9.0, adjacent_to = ["room2"]
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["room2".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        // Room2: min_area = 9.0, no requirements
        let room2 = RoomRequirement {
            id: "room2".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        // Boundary: 10.0 × 10.0
        let result = solve_layout(vec![room1, room2], 10.0, 10.0);

        assert!(result.is_ok());
        let solution = result.unwrap();
        assert_eq!(solution.rooms.len(), 2);

        // Verify room1 and room2 are actually adjacent
        let room1_placed = solution.rooms.iter().find(|r| r.id == "room1").unwrap();
        let room2_placed = solution.rooms.iter().find(|r| r.id == "room2").unwrap();

        let rect1 = Rectangle::from_room(room1_placed);
        let rect2 = Rectangle::from_room(room2_placed);

        assert!(
            rect1.is_adjacent_to(&rect2),
            "room1 and room2 should be adjacent"
        );
    }

    // Test 4: test_solve_respects_must_have_external_wall
    #[test]
    fn test_solve_respects_must_have_external_wall() {
        // Room1: min_area = 16.0, has_exterior_wall = true
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 16.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: true,
        };

        // Boundary: 10.0 × 10.0
        let result = solve_layout(vec![room1], 10.0, 10.0);

        assert!(result.is_ok());
        let solution = result.unwrap();

        // Assert: solution.rooms[0] touches external wall
        let placed_room = &solution.rooms[0];
        let rect = Rectangle::from_room(placed_room);

        assert!(
            rect.touches_exterior_wall(10.0, 10.0),
            "Room with has_exterior_wall=true should touch external wall"
        );
    }

    // Test 5: test_solve_orders_by_most_constrained_first
    #[test]
    fn test_solve_orders_by_most_constrained_first() {
        // Room1: min_area = 9.0, no constraints (ID: "simple")
        let room1 = RoomRequirement {
            id: "simple".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        // Room2: min_area = 9.0, adjacent_to = ["simple"], has_exterior_wall = true (ID: "complex")
        let room2 = RoomRequirement {
            id: "complex".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["simple".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: true,
        };

        // Boundary: 10.0 × 10.0
        let result = solve_layout(vec![room1, room2], 10.0, 10.0);

        assert!(result.is_ok());
        let solution = result.unwrap();

        // The solver should place "complex" first (more constrained)
        // We can verify this succeeded by checking both rooms are placed
        assert_eq!(solution.rooms.len(), 2);
        assert!(solution.rooms.iter().any(|r| r.id == "simple"));
        assert!(solution.rooms.iter().any(|r| r.id == "complex"));
    }

    // Test 6: test_solution_has_valid_total_score
    #[test]
    fn test_solution_has_valid_total_score() {
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 9.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: true, // Should get bonus points
        };

        let room2 = RoomRequirement {
            id: "room2".to_string(),
            min_area: 9.0,
            adjacent_to: vec!["room1".to_string()], // Should get adjacency bonus
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        let result = solve_layout(vec![room1, room2], 10.0, 10.0);

        assert!(result.is_ok());
        let solution = result.unwrap();

        // Total score should be positive and reasonable
        assert!(solution.total_score > 0.0);
        println!("Total score: {}", solution.total_score);

        // Each room should score at least base points (20 hard + 5 no violations = 25)
        // With 2 rooms, minimum should be around 50
        assert!(solution.total_score >= 50.0);
    }

    // Test 7: test_solve_complex_layout_with_all_constraints
    #[test]
    fn test_solve_complex_layout_with_all_constraints() {
        // Living room - must have external wall, most constrained
        let living = RoomRequirement {
            id: "living".to_string(),
            min_area: 20.0,
            adjacent_to: vec!["kitchen".to_string()],
            not_adjacent_to: vec!["bathroom".to_string()],
            has_exterior_wall: true,
        };

        // Kitchen - adjacent to living
        let kitchen = RoomRequirement {
            id: "kitchen".to_string(),
            min_area: 12.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        // Bathroom - cannot be adjacent to living
        let bathroom = RoomRequirement {
            id: "bathroom".to_string(),
            min_area: 6.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };

        let result = solve_layout(
            vec![living, kitchen, bathroom],
            15.0, // Larger boundary for 3 rooms
            15.0,
        );

        assert!(result.is_ok());
        let solution = result.unwrap();

        // Verify all rooms placed
        assert_eq!(solution.rooms.len(), 3);
        assert!(solution.is_valid);
        assert!(solution.total_score > 0.0);

        // Find each room
        let living_room = solution.rooms.iter().find(|r| r.id == "living").unwrap();
        let kitchen_room = solution.rooms.iter().find(|r| r.id == "kitchen").unwrap();
        let bathroom_room = solution.rooms.iter().find(|r| r.id == "bathroom").unwrap();

        // Verify constraints
        // Living room has external wall
        let living_rect = Rectangle::from_room(living_room);
        assert!(living_rect.touches_exterior_wall(15.0, 15.0));

        // Living and kitchen are adjacent
        let kitchen_rect = Rectangle::from_room(kitchen_room);
        assert!(living_rect.is_adjacent_to(&kitchen_rect));

        // Living and bathroom are NOT adjacent
        let bathroom_rect = Rectangle::from_room(bathroom_room);
        assert!(!living_rect.is_adjacent_to(&bathroom_rect));

        println!("✅ Complex layout solved successfully!");
        println!("Total score: {}", solution.total_score);
        println!(
            "Living room: ({}, {}) {}x{}",
            living_room.x, living_room.y, living_room.width, living_room.height
        );
        println!(
            "Kitchen: ({}, {}) {}x{}",
            kitchen_room.x, kitchen_room.y, kitchen_room.width, kitchen_room.height
        );
        println!(
            "Bathroom: ({}, {}) {}x{}",
            bathroom_room.x, bathroom_room.y, bathroom_room.width, bathroom_room.height
        );
    }
}
