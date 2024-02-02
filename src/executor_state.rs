use npc_engine_core::{ActiveTask, ActiveTasks, IdleTask};
use npc_engine_utils::ExecutorStateGlobal;

use crate::{domain::RPSBattleRoyaleDomain, state::GlobalState, CONTESTANTS, PLANNING_MINIMUM_VISITS};

#[derive(Debug, Default)]
pub struct RPSBattleRoyaleExecutorState {
    rock_count: u32,
    paper_count: u32,
    scissors_count: u32,
}

impl ExecutorStateGlobal<RPSBattleRoyaleDomain> for RPSBattleRoyaleExecutorState {
    const MINIMUM_VISITS: u32 = PLANNING_MINIMUM_VISITS;

    fn create_initial_state(&self) -> GlobalState {
        // create map with size from constants 
        // spawn in contestants, sampling their locations from a disc, want it denser near the center
        // make sure the spawn points are far enough away at the beginning
        // GlobalState::from_map_and_agents(map, agents)
        todo!()
    }

    fn init_task_queue(&self, state: &GlobalState) -> ActiveTasks<RPSBattleRoyaleDomain> {
        state
            .agents
            .iter()
            .map(|(id, _)| ActiveTask::new_with_end(0, 0, *id, Box::new(IdleTask)))
            .collect()
    }

    fn keep_execution(&self, _tick: u64, _queue: &ActiveTasks<RPSBattleRoyaleDomain>, _state: &GlobalState) -> bool {
        self.rock_count == CONTESTANTS || self.paper_count == CONTESTANTS || self.scissors_count == CONTESTANTS
    }

    fn post_step_hook(&mut self, _tick: u64, _state: &mut GlobalState) {
        //print the screen here - add format to global state, its just printing it here
    }

}