use crate::types::Room;


/// A rectangle with a position and size.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {

    /// Create a rectangle from a room.
    pub fn from_room(room: &Room) -> Self {
        Self {
            x: room.x,
            y: room.y,
            width: room.width,
            height: room.height,
        }
    }

    /// Check if two rectangles overlap using "NOT separated" logic
    /// Two rectangles are separated if:
    /// - One is completely to the left of the other, OR
    /// - One is completely to the right of the other, OR
    /// - One is completely above the other, OR
    /// - One is completely below the other
    /// Returns true if rectangles overlap (share area, not just touch)
    pub fn overlaps_with(&self, other: &Rectangle) -> bool {
        let x_separated = self.x + self.width <= other.x || other.x + other.width <= self.x;
        let y_separated = self.y + self.height <= other.y || other.y + other.height <= self.y;

        !(x_separated || y_separated)
    }

    /// Check if two rectangles are adjacent (share an edge with range overlap).
    /// Not just corner touching - must share a full edge segment
    pub fn is_adjacent_to(&self, other: &Rectangle) -> bool {
        // Check vertical edge sharing (left-right adjacency)
        let vertical_edge_touching = self.x == other.x + other.width || self.x + self.width == other.x;

        // check vertical range overlap
        let vertical_range_overlap = self.y < other.y + other.height && self.y + self.height > other.y;
        
        // Check horizontal edge sharing (top-bottom adjacency)
        let horizontal_edge_touching = self.y == other.y + other.height || self.y + self.height == other.y;

        // check horizontal range overlap
        let horizontal_range_overlap = self.x < other.x + other.width && self.x + self.width > other.x;

        (vertical_edge_touching && vertical_range_overlap) || (horizontal_edge_touching && horizontal_range_overlap)
    }

    // Check if a rectangle is fully inside boundary.
    pub fn is_within_boundary(&self, boundary_width: f64, boundary_height: f64) -> bool {
        self.x >= 0.0 && (self.x + self.width) <= boundary_width &&
        self.y >= 0.0 && (self.y + self.height) <= boundary_height
    }

    // Check if a rectangle touches the exterior wall of the boundary.
    pub fn touches_exterior_wall(&self, boundary_width: f64, boundary_height: f64) -> bool {
        self.x == 0.0 || (self.x + self.width) == boundary_width ||
        self.y == 0.0 || (self.y + self.height) == boundary_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test 1: test_overlaps_when_overlapping
    // Checks if two rectangles overlap when they share an area.
    #[test]
    fn test_overlaps_when_overlapping() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 };
        let rectangle2 = Rectangle { x: 5.0, y: 5.0, width: 10.0, height: 10.0 };
        assert!(rectangle1.overlaps_with(&rectangle2));
    }

    // Test 2: test_no_overlap_when_separated
    // Checks if two rectangles do not overlap when they are completely separated.
    #[test]
    fn test_no_overlap_when_separated() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 };
        let rectangle2 = Rectangle { x: 15.0, y: 5.0, width: 10.0, height: 10.0 };
        assert!(!rectangle1.overlaps_with(&rectangle2));
    }

    // Test 3: test_no_overlap_when_touching_edges
    // Checks if two rectangles do not overlap when they are touching at the edges.
    #[test]
    fn test_no_overlap_when_touching_edges() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let rectangle2 = Rectangle { x: 5.0, y: 0.0, width: 5.0, height: 5.0 };
        assert!(!rectangle1.overlaps_with(&rectangle2));
    }

    // Test 4: test_adjacent_when_sharing_vertical_edge
    // Checks if two rectangles are adjacent when they share a vertical edge.
    #[test]
    fn test_adjacent_when_sharing_vertical_edge() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let rectangle2 = Rectangle { x: 5.0, y: 2.0, width: 10.0, height: 10.0 };
        assert!(rectangle1.is_adjacent_to(&rectangle2));
    }

    // Test 5: test_not_adjacent_when_corner_touch_only
    // Checks if two rectangles are not adjacent when they are touching at the corners.
    #[test]
    fn test_not_adjacent_when_corner_touch_only() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let rectangle2 = Rectangle { x: 5.0, y: 5.0, width: 5.0, height: 5.0 };
        assert!(!rectangle1.is_adjacent_to(&rectangle2));
    }

    // Test 6: test_adjacent_when_sharing_horizontal_edge
    // Checks if two rectangles are adjacent when they share a horizontal edge.
    #[test]
    fn test_adjacent_when_sharing_horizontal_edge() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let rectangle2 = Rectangle { x: 2.0, y: 5.0, width: 5.0, height: 3.0 };
        assert!(rectangle1.is_adjacent_to(&rectangle2));
    }

    // Test 7: test_not_adjacent_when_separated
    // Checks if two rectangles are not adjacent when they are completely separated.
    #[test]
    fn test_not_adjacent_when_separated() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let rectangle2 = Rectangle { x: 10.0, y: 0.0, width: 5.0, height: 5.0 };
        assert!(!rectangle1.is_adjacent_to(&rectangle2));
    }

    // Test 8: test_not_adjacent_when_edges_align_but_no_range_overlap
    // Checks if two rectangles are not adjacent when they are touching at the edges but do not share a range.
    #[test]
    fn test_not_adjacent_when_edges_align_but_no_range_overlap() {
        let rectangle1 = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
        let rectangle2 = Rectangle { x: 5.0, y: 10.0, width: 5.0, height: 5.0 };
        assert!(!rectangle1.is_adjacent_to(&rectangle2));
    }

    // Test 9: test_within_boundary_when_inside
    // Checks if a rectangle is within the boundary.
    #[test]
    fn test_within_boundary_when_inside() {
        let rectangle = Rectangle { x: 1.0, y: 1.0, width: 3.0, height: 3.0 };
        assert!(rectangle.is_within_boundary(10.0, 10.0));
    }

    // Test 10: test_not_within_boundary_when_exceeds_right
    // Checks if a rectangle is not within the boundary when it exceeds the right edge.
    #[test]
    fn test_not_within_boundary_when_exceeds_right() {
        let rectangle = Rectangle { x: 8.0, y: 1.0, width: 5.0, height: 3.0 };
        assert!(!rectangle.is_within_boundary(10.0, 10.0));
    }

    // Test 11: test_not_within_boundary_when_exceeds_bottom
    // Checks if a rectangle is not within the boundary when it exceeds the bottom edge.
    #[test]
    fn test_not_within_boundary_when_exceeds_bottom() {
        let rectangle = Rectangle { x: 1.0, y: 8.0, width: 3.0, height: 5.0 };
        assert!(!rectangle.is_within_boundary(10.0, 10.0));
    }

    // Test 12: test_not_within_boundary_when_negative_position
    // Checks if a rectangle is not within the boundary when it has a negative position.
    #[test]
    fn test_not_within_boundary_when_negative_position() {
        let rectangle = Rectangle { x: -1.0, y: 0.0, width: 5.0, height: 5.0 };
        assert!(!rectangle.is_within_boundary(10.0, 10.0));
    }

    // Test 13: test_touches_external_wall_left
    // Checks if a rectangle touches the exterior wall when it is on the left edge.
    #[test]
    fn test_touches_external_wall_left() {
        let rectangle = Rectangle { x: 0.0, y: 2.0, width: 5.0, height: 5.0 };
        assert!(rectangle.touches_exterior_wall(10.0, 10.0));
    }

    // Test 14: test_touches_external_wall_top
    // Checks if a rectangle touches the exterior wall when it is on the top edge.
    #[test]
    fn test_touches_external_wall_top() {
        let rectangle = Rectangle { x: 2.0, y: 0.0, width: 5.0, height: 5.0 };
        assert!(rectangle.touches_exterior_wall(10.0, 10.0));
    }

    // Test 15: test_touches_external_wall_right
    // Checks if a rectangle touches the exterior wall when it is on the right edge.
    #[test]
    fn test_touches_external_wall_right() {
        let rectangle = Rectangle { x: 5.0, y: 2.0, width: 5.0, height: 5.0 };
        assert!(rectangle.touches_exterior_wall(10.0, 10.0));
    }

    // Test 16: test_touches_external_wall_bottom
    // Checks if a rectangle touches the exterior wall when it is on the bottom edge.
    #[test]
    fn test_touches_external_wall_bottom() {
        let rectangle = Rectangle { x: 2.0, y: 5.0, width: 5.0, height: 5.0 };
        assert!(rectangle.touches_exterior_wall(10.0, 10.0));
    }

    // Test 17: test_not_touches_external_wall_when_interior
    // Checks if a rectangle does not touch the exterior wall when it is inside the boundary.
    #[test]
    fn test_not_touches_external_wall_when_interior() {
        let rectangle = Rectangle { x: 2.0, y: 2.0, width: 3.0, height: 3.0 };
        assert!(!rectangle.touches_exterior_wall(10.0, 10.0));
    }

    // Test 18: test_from_room
    // Checks if a rectangle is created from a room.
    #[test]
    fn test_from_room() {
        let room = Room {
            id: "test".to_string(),
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        };
        let rectangle = Rectangle::from_room(&room);
        assert_eq!(rectangle.x, 1.0);
        assert_eq!(rectangle.y, 2.0);
        assert_eq!(rectangle.width, 3.0);
        assert_eq!(rectangle.height, 4.0);
    }
    
}

