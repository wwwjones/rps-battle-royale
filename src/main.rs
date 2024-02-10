use std::num::NonZeroU64;

use executor_state::RPSBattleRoyaleExecutorState;
use npc_engine_core::MCTSConfiguration;
use npc_engine_utils::run_threaded_executor;

use constants::*;

mod constants;
mod executor_state;
mod map;
mod domain;
mod state;
mod tasks;
mod behavior;
mod testing_factory;
mod planner;

fn main() {
    // These parameters control the MCTS algorithm.
    let mcts_config = MCTSConfiguration {
        allow_invalid_tasks: false,
        visits: PLANNING_VISITS,
        depth: PLANNING_DEPTH,
        exploration: PLANNING_EXPLORATION,
        discount_hl: PLANNING_DISCOUNT_HL,
        seed: None,
        planning_task_duration: Some(NonZeroU64::new(PLANNING_DURATION).unwrap()),
    };

    env_logger::init();

    // First clear the screen.
    print!("{}[2J", 27 as char);

    // State of the execution.
    let mut executor_state = RPSBattleRoyaleExecutorState::default();

    run_threaded_executor(&mcts_config, &mut executor_state, EXECUTION_STEP_DURATION);
}
