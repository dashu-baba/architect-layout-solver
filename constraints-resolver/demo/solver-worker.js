// Web Worker for running the solver asynchronously
import init, { solve_layout } from '../pkg/constraints_resolver.js';

let initialized = false;

self.onmessage = async function(e) {
    const { rooms, boundaryWidth, boundaryHeight } = e.data;
    
    try {
        // Initialize WASM module once
        if (!initialized) {
            await init();
            initialized = true;
        }
        
        // Run the solver
        const startTime = Date.now();
        const solution = solve_layout(rooms, boundaryWidth, boundaryHeight);
        const elapsed = Date.now() - startTime;
        
        // Send success result back to main thread
        self.postMessage({
            success: true,
            solution: solution,
            elapsed: elapsed
        });
    } catch (error) {
        // Send error back to main thread
        self.postMessage({
            success: false,
            error: error.message || String(error)
        });
    }
};

