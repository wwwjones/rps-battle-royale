use std::num::NonZeroU64;

use npc_engine_core::MCTSConfiguration;
use npc_engine_utils::run_threaded_executor;

use constants::*;

mod constants;
mod executor_state;
mod map;
mod domain;
mod state;

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

    // First clear the screen.
    // clearscreen::clear().unwrap();

    // State of the execution.
    let mut executor_state = RPSBattleRoyaleExecutorState::default();

    // Run as long as there is at least one agent alive.
    run_threaded_executor(&mcts_config, &mut executor_state, EXECUTION_STEP_DURATION);
}
