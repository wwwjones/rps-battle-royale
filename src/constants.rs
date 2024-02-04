use std::time::Duration;

pub const CONTESTANTS: u32 = 10;

// map generation
pub const MAP_HEIGHT: u32 = 10;
pub const MAP_WIDTH: u32 = 10; 

// planning parameters
pub const PLANNING_DURATION: u64 = 5;
pub const PLANNING_VISITS: u32 = 1000;
pub const PLANNING_MINIMUM_VISITS: u32 = 50;
pub const PLANNING_DEPTH: u32 = 200;
pub const PLANNING_DISCOUNT_HL: f32 = PLANNING_DEPTH as f32 / 3.0;
pub const PLANNING_EXPLORATION: f32 = 1.414;

// task weights (idle task has weight 1)
pub const CONVERT_WEIGHT: f32 = 1.0;
pub const MOVE_WEIGHT: f32 = 5.0;

// execution paramters
pub const EXECUTION_STEP_DURATION: Duration = Duration::from_millis(0);
