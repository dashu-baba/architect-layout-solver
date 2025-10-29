use crate::types::RoomRequirement;

pub fn order_rooms_by_constraints(mut rooms_reqs: Vec<RoomRequirement>) -> Vec<RoomRequirement> {
    rooms_reqs.sort_by(|a,b| {
        let count_a = count_constraints(a);
        let count_b = count_constraints(b);
        count_b.cmp(&count_a)
    });
    rooms_reqs
}

fn count_constraints(room_req: &RoomRequirement) -> usize {
    let mut count = 0;
    count += room_req.adjacent_to.len();
    count += room_req.not_adjacent_to.len();
    count += if room_req.has_exterior_wall { 1 } else { 0 };
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: test_count_constraints_with_multiple_requirements
    #[test]
    fn test_count_constraints_with_multiple_requirements() {
        let room = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec!["room2".to_string(), "room3".to_string()],
            not_adjacent_to: vec!["room4".to_string()],
            has_exterior_wall: true,
        };
        
        let count = count_constraints(&room);
        assert_eq!(count, 4);  // 2 adjacent + 1 not_adjacent + 1 exterior wall
    }

    // Test 2: test_count_constraints_with_no_requirements
    #[test]
    fn test_count_constraints_with_no_requirements() {
        let room = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec![],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        let count = count_constraints(&room);
        assert_eq!(count, 0);
    }

    // Test 3: test_count_constraints_only_adjacency
    #[test]
    fn test_count_constraints_only_adjacency() {
        let room = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec!["room2".to_string(), "room3".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        let count = count_constraints(&room);
        assert_eq!(count, 2);
    }

    // Test 4: test_order_rooms_most_constrained_first
    #[test]
    fn test_order_rooms_most_constrained_first() {
        // room1: 4 constraints (adjacent_to = 2, not_adjacent_to = 1, exterior wall = true)
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec!["room2".to_string(), "room3".to_string()],
            not_adjacent_to: vec!["room4".to_string()],
            has_exterior_wall: true,
        };
        
        // room2: 1 constraint (adjacent_to = 1)
        let room2 = RoomRequirement {
            id: "room2".to_string(),
            min_area: 15.0,
            adjacent_to: vec!["room1".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        // room3: 2 constraints (adjacent_to = 2)
        let room3 = RoomRequirement {
            id: "room3".to_string(),
            min_area: 18.0,
            adjacent_to: vec!["room1".to_string(), "room2".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        let rooms = vec![room1, room2, room3];
        let ordered = order_rooms_by_constraints(rooms);
        
        assert_eq!(ordered[0].id, "room1");  // 4 constraints first
        assert_eq!(ordered[1].id, "room3");  // 2 constraints second
        assert_eq!(ordered[2].id, "room2");  // 1 constraint last
    }

    // Test 5: test_order_rooms_preserves_equal_constraints
    #[test]
    fn test_order_rooms_preserves_equal_constraints() {
        // Create 2 rooms with same constraint count (2 each)
        let room1 = RoomRequirement {
            id: "room1".to_string(),
            min_area: 20.0,
            adjacent_to: vec!["room2".to_string(), "room3".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: false,
        };
        
        let room2 = RoomRequirement {
            id: "room2".to_string(),
            min_area: 15.0,
            adjacent_to: vec!["room1".to_string()],
            not_adjacent_to: vec![],
            has_exterior_wall: true,
        };
        
        let rooms = vec![room1, room2];
        let ordered = order_rooms_by_constraints(rooms);
        
        // Assert both rooms still in result (length = 2)
        assert_eq!(ordered.len(), 2);
        // Both have 2 constraints, so order doesn't matter but both should be present
        assert!(ordered.iter().any(|r| r.id == "room1"));
        assert!(ordered.iter().any(|r| r.id == "room2"));
    }
}