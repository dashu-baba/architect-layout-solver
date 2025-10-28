/// A room with a position and size.
#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// A requirement for a room.
pub struct RoomRequirement {
    pub id: String,
    pub min_area: f64,
    pub adjacent_to: Vec<String>,
    pub not_adjacent_to: Vec<String>,
    pub has_exterior_wall: bool,   
}

/// A rectangle with a position and size.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}