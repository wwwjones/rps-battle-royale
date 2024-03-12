use std::time::Duration;

pub const CONTESTANTS: u32 = 15;

// map generation
pub const MAP_HEIGHT: u32 = 25;
pub const MAP_WIDTH: u32 = 25; 

// planning parameters
pub const PLANNING_DURATION: u64 = 3;
pub const PLANNING_MINIMUM_VISITS: u32 = 5;
pub const PLANNING_DEPTH: u32 = 3; //keep relatively low, 5 seems good for now

// task weights (idle task has weight 1)
pub const MOVE_WEIGHT: f32 = 10.0;

pub const VISIBILITY_DISTANCE: i32 = 30; //higher seems to be better, though maybe try not a manhattan distance, but x closest considered

// execution paramters
pub const EXECUTION_STEP_DURATION: Duration = Duration::from_millis(0);
