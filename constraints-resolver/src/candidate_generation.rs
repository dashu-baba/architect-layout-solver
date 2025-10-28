/// Generate aspect ratio candidates for a given minimum area.
pub fn generate_aspect_ratio_candidates(min_area: f64) -> Vec<(f64, f64)> {
    let mut candidates = Vec::new();
    let ratios = [0.5, 0.67, 0.8, 1.0, 1.2, 1.5, 2.0];

    for ratio in ratios {
        let width = (min_area / ratio).sqrt();
        let height = min_area / width;

        // round to nearest 0.5
        let rounded_width = round_to_nearest_0_5(width);
        let rounded_height = round_to_nearest_0_5(height);

        // check if the rounded width and height are equal to the min area
        if rounded_width * rounded_height == min_area {
            candidates.push((rounded_width, rounded_height));
        }
    }
    candidates
}

fn round_to_nearest_0_5(value: f64) -> f64 {
    (value * 2.0).round() / 2.0
}

/// Generate grid positions for a given room size and boundary size.
pub fn generate_grid_positions(
    room_width: f64,
    room_height: f64,
    boundary_width: f64,
    boundary_height: f64,
) -> Vec<(f64, f64)>{
    let mut positions = Vec::new();
    let grid_size = 0.5;

    let mut x = 0.0;
    while (x + room_width) <= boundary_width {
        let mut y = 0.0;
        while y + room_height <= boundary_height {
            positions.push((x, y));
            y += grid_size;
        }
        x += grid_size;
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    // Test 1: test_generate_aspect_ratio_candidates_returns_multiple_candidates
    fn test_generate_aspect_ratio_candidates_returns_multiple_candidates() {
        let candidates = generate_aspect_ratio_candidates(20.0);
        assert!(candidates.len() > 0);
    }

    #[test]
    // Test 2: test_all_candidates_meet_minimum_area
    fn test_all_candidates_meet_minimum_area() {
        let min_area = 20.0;
        let candidates = generate_aspect_ratio_candidates(min_area);
        
        for (width, height) in candidates {
            assert!(width * height >= min_area);
        }
    }

    #[test]
    // Test 3: test_dimensions_rounded_to_half_meter
    fn test_dimensions_rounded_to_half_meter() {
        let candidates = generate_aspect_ratio_candidates(20.0);
        
        for (width, height) in candidates {
            // Check width is rounded to 0.5m
            assert_eq!(width, round_to_nearest_0_5(width));
            // Check height is rounded to 0.5m
            assert_eq!(height, round_to_nearest_0_5(height));
        }
    }

    #[test]
    // Test 4: test_generate_grid_positions_returns_valid_positions
    fn test_generate_grid_positions_returns_valid_positions( ) {
        let positions = generate_grid_positions(2.0, 2.0,5.0, 5.0);
        assert!(positions.len() > 0);
    }

    #[test]
    // Test 5: test_grid_positions_respect_boundary
    fn test_grid_positions_respect_boundary(){
        let positions = generate_grid_positions(3.0, 2.0,5.0, 4.0);
        for (x, y) in positions {
            assert!(x + 3.0 <= 5.0);
            assert!(y + 2.0 <= 4.0);
        }
    }

    // Test 6: test_grid_positions_use_half_meter_steps
    #[test]
    fn test_grid_positions_use_half_meter_steps() {
        let positions = generate_grid_positions(2.0, 2.0, 3.0, 3.0);
        
        // Expected positions: (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (1.0, 0.0), (1.0, 0.5), (1.0, 1.0)
        assert_eq!(positions.len(), 9);
        
        // Check that (0.5, 0.5) is in the positions
        assert!(positions.contains(&(0.5, 0.5)));
        
        // Verify all expected positions
        let expected = vec![
            (0.0, 0.0), (0.0, 0.5), (0.0, 1.0),
            (0.5, 0.0), (0.5, 0.5), (0.5, 1.0),
            (1.0, 0.0), (1.0, 0.5), (1.0, 1.0),
        ];
        for pos in expected {
            assert!(positions.contains(&pos), "Missing position: {:?}", pos);
        }
    }

    // Test 7: test_grid_positions_excludes_positions_that_exceed_boundary
    #[test]
    fn test_grid_positions_excludes_positions_that_exceed_boundary() {
        let positions = generate_grid_positions(3.0, 2.0, 4.0, 3.0);
        
        // Position (2.0, 1.5) should NOT be in results (because 2.0 + 3.0 = 5.0 > 4.0)
        assert!(!positions.contains(&(2.0, 1.5)), "Position (2.0, 1.5) should be excluded");
        
        // Verify that all positions respect the boundary
        for (x, y) in &positions {
            assert!(x + 3.0 <= 4.0, "Position ({}, {}) exceeds width boundary", x, y);
            assert!(y + 2.0 <= 3.0, "Position ({}, {}) exceeds height boundary", x, y);
        }
    }
}