//! # WebAssembly Bindings
//!
//! Provides JavaScript-compatible bindings for the constraint solver.
//!
//! This module exposes the Rust solver functionality to WebAssembly/JavaScript
//! environments with proper serialization, error handling, and performance timing.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::solver::solve_layout as solve_layout_internal;
use crate::types::RoomRequirement;
use instant::Instant;

/// JavaScript-compatible input structure for room requirements.
///
/// This structure is deserialized from JavaScript objects and converted
/// to internal `RoomRequirement` types.
#[derive(Serialize, Deserialize)]
pub struct RoomInput {
    pub id: String,
    pub min_area: f64,
    pub adjacent_to: Vec<String>,
    pub not_adjacent_to: Vec<String>,
    pub has_exterior_wall: bool,
}

/// JavaScript-compatible output structure for layout solutions.
///
/// Contains the solved room placements, total quality score, and
/// computation time in milliseconds.
#[derive(Serialize, Deserialize)]
pub struct SolutionOutput {
    pub rooms: Vec<PlacedRoomOutput>,
    pub score: f64,
    pub computation_time_ms: u64,
}

/// JavaScript-compatible structure for a placed room.
///
/// Represents a room with its final position and dimensions.
#[derive(Serialize, Deserialize)]
pub struct PlacedRoomOutput {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Solves the room layout problem from JavaScript.
///
/// This is the main WASM entry point for the solver. It accepts JavaScript
/// objects, runs the constraint solver, and returns a solution with timing data.
///
/// # Arguments
///
/// * `rooms_json` - JavaScript array of room requirement objects
/// * `boundary_width` - Width of the layout boundary in meters
/// * `boundary_height` - Height of the layout boundary in meters
///
/// # Returns
///
/// A JavaScript object containing:
/// - `rooms`: Array of placed rooms with positions and dimensions
/// - `score`: Total quality score of the solution
/// - `computation_time_ms`: Time taken to solve in milliseconds
///
/// # Errors
///
/// Returns a JavaScript error if:
/// - Input cannot be parsed
/// - No valid solution exists
/// - Output cannot be serialized
///
/// # JavaScript Example
///
/// ```javascript
/// import init, { init_logging, solve_layout } from './pkg/constraints_resolver.js';
///
/// async function solvePlan() {
///     await init();
///     init_logging();
///
///     const rooms = [
///         {
///             id: "living_room",
///             min_area: 20.0,
///             adjacent_to: ["kitchen"],
///             not_adjacent_to: [],
///             has_exterior_wall: true
///         },
///         {
///             id: "kitchen",
///             min_area: 12.0,
///             adjacent_to: [],
///             not_adjacent_to: [],
///             has_exterior_wall: false
///         }
///     ];
///
///     try {
///         const solution = solve_layout(rooms, 10.0, 10.0);
///         console.log(`Solved in ${solution.computation_time_ms}ms`);
///         console.log(`Total score: ${solution.score}`);
///         solution.rooms.forEach(room => {
///             console.log(`${room.id}: (${room.x}, ${room.y}) ${room.width}×${room.height}`);
///         });
///     } catch (error) {
///         console.error('Solver failed:', error);
///     }
/// }
/// ```
#[wasm_bindgen]
pub fn solve_layout(
    rooms_json: JsValue,
    boundary_width: f64,
    boundary_height: f64,
) -> Result<JsValue, JsValue> {
    // 1. Deserialize input from JS
    let room_inputs: Vec<RoomInput> = serde_wasm_bindgen::from_value(rooms_json)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;
    
    // 2. Convert to your internal RoomRequirement types
    let rooms: Vec<RoomRequirement> = room_inputs.iter().map(|input| {
        RoomRequirement {
            id: input.id.clone(),
            min_area: input.min_area,
            adjacent_to: input.adjacent_to.clone(),
            not_adjacent_to: input.not_adjacent_to.clone(),
            has_exterior_wall: input.has_exterior_wall,
        }
    }).collect();
    
    // 3. Call your existing solver
    let start = Instant::now();
    let solution = solve_layout_internal(rooms, boundary_width, boundary_height)
        .map_err(|e| JsValue::from_str(&format!("Solver error: {:?}", e)))?;
    let elapsed = start.elapsed().as_millis() as u64;
    
    // 4. Convert solution to JS-friendly format
    let output = SolutionOutput {
        rooms: solution.rooms.iter().map(|room| {
            PlacedRoomOutput {
                id: room.id.clone(),
                x: room.x,
                y: room.y,
                width: room.width,
                height: room.height,
            }
        }).collect(),
        score: solution.total_score,
        computation_time_ms: elapsed,
    };
    
    // 5. Serialize back to JS
    serde_wasm_bindgen::to_value(&output)
        .map_err(|e| JsValue::from_str(&format!("Serialize error: {}", e)))
}