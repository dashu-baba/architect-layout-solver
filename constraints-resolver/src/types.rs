pub struct Room {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct RoomRequirement {
    pub id: String,
    pub min_area: f64,
    pub adjacent_to: Vec<String>,
    pub not_adjacent_to: Vec<String>,
    pub has_exterior_wall: bool,   
}

