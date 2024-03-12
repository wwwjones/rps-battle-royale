use std::num::NonZeroU64;

use executor_state::RPSBattleRoyaleExecutorState;

use constants::*;

use executor::run_threaded_executor;

use planner::brain::BRAINConfiguration;

mod constants;
mod executor_state;
mod map;
mod domain;
mod state;
mod tasks;
mod behavior;
mod testing_factory;
mod planner;
mod executor;

fn main() {

    //These parameters control the BRAIN
    let brain_config = BRAINConfiguration {
        depth: PLANNING_DEPTH,
        planning_task_duration: Some(NonZeroU64::new(PLANNING_DURATION).unwrap()),
    };

    env_logger::init();

    // First clear the screen.
    print!("{}[2J", 27 as char);

    // State of the execution.
    let mut executor_state = RPSBattleRoyaleExecutorState::default();

    run_threaded_executor(&brain_config, &mut executor_state, EXECUTION_STEP_DURATION);
}
