use crate::types::{Rectangle, Room};

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
        let x_separated = self.x + self.width < other.x || other.x + other.width < self.x;
        let y_separated = self.y + self.height < other.y || other.y + other.height < self.y;

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

