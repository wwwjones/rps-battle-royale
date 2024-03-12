use std::collections::HashSet;

use npc_engine_core::{ActiveTask, ActiveTasks, AgentId, IdleTask, MCTS};
use npc_engine_utils::{plot_tree_in_tmp_with_task_name, Coord2D};

use crate::{domain::RPSBattleRoyaleDomain, executor::{ExecutorState, ExecutorStateGlobal}, map::Map, state::{AgentState, AgentType, Agents, GlobalState}, CONTESTANTS, MAP_HEIGHT, MAP_WIDTH, PLANNING_DURATION, PLANNING_MINIMUM_VISITS};

#[derive(Debug)]
pub struct RPSBattleRoyaleExecutorState {
    rock_count: u32,
    paper_count: u32,
    scissors_count: u32,
    mcts_visits: Vec<u32>,
}

impl Default for RPSBattleRoyaleExecutorState {
    fn default() -> Self {
        let rock_count = CONTESTANTS/3;
        let paper_count = CONTESTANTS/3;
        let scissors_count = CONTESTANTS - rock_count - paper_count;
        Self { rock_count, paper_count, scissors_count, mcts_visits: vec![0; CONTESTANTS as usize] }
    }
}

impl ExecutorStateGlobal<RPSBattleRoyaleDomain> for RPSBattleRoyaleExecutorState {
    const MINIMUM_VISITS: u32 = PLANNING_MINIMUM_VISITS;

    fn create_initial_state(&self) -> GlobalState {
        // create map with size from constants 
        // spawn in contestants, sampling their locations from a disc, want it denser near the center
        // make sure the spawn points are far enough away at the beginning
        // first, just spawn them all in randomly, easy enough

        let map = Map::new(MAP_HEIGHT, MAP_WIDTH);
        let map_size = map.size();

        let mut agents = Agents::new();
        let mut used_poses = HashSet::new();
        let mut agent_id = 0;
        let mut add_contestants = |ty, count| {
            for _i in 0..count {
                loop {
                    let location = Coord2D::rand_uniform(map_size);
                    if !used_poses.contains(&location) {
                        used_poses.insert(location);
                        agents.insert(
                            AgentId(agent_id),
                            AgentState{
                                ty,
                                location,
                                conversions: 0,
                            }
                        );
                        agent_id += 1;
                        break;
                    }
                }
            }
        };

        let rocks = CONTESTANTS/3;
        let papers = CONTESTANTS/3;
        let scissors = CONTESTANTS - rocks - papers;
        add_contestants(AgentType::Rock, rocks);
        add_contestants(AgentType::Paper, papers);
        add_contestants(AgentType::Scissors, scissors);
      
        GlobalState { map, agents }
    }

    fn init_task_queue(&self, state: &GlobalState) -> ActiveTasks<RPSBattleRoyaleDomain> {
        let groups = PLANNING_DURATION + 1;

        state
            .agents
            .iter()
            .map(|(id, _)| ActiveTask::new_with_end(0, id.0 as u64 % groups, *id, Box::new(IdleTask)))
            .collect()
    }

    fn keep_execution(&self, _tick: u64, _queue: &ActiveTasks<RPSBattleRoyaleDomain>, _state: &GlobalState) -> bool {
        !(self.rock_count == CONTESTANTS || self.paper_count == CONTESTANTS || self.scissors_count == CONTESTANTS)
    }

    fn post_step_hook(&mut self, tick: u64, state: &mut GlobalState) {
        //print!("\x1B[H {:?}", state.agents);

        // update the type counts and print the screen
        let mut rock_count = 0;
        let mut paper_count = 0;
        let mut scissors_count = 0;
        for (_, agent) in &state.agents {
            match agent.ty {
                AgentType::Rock => rock_count += 1 ,
                AgentType::Paper => paper_count += 1,
                AgentType::Scissors => scissors_count += 1,
            };
        }

        self.rock_count = rock_count;
        self.paper_count = paper_count;
        self.scissors_count = scissors_count;

        print!(
            "\x1B[H\
            T{tick}   🗿:{}  📄:{}  ✂️:{}\n\
            {}\n",
            self.rock_count,
            self.paper_count,
            self.scissors_count,
            *state,
        );
        
        if self.rock_count == CONTESTANTS {println!("Rock wins!\n {:?}", state.agents)} 
        else if self.paper_count == CONTESTANTS {println!("Paper wins!\n {:?}", state.agents)}
        else if self.scissors_count == CONTESTANTS {println!("Scissors win!\n {:?}", state.agents)}

    }
}

impl ExecutorState<RPSBattleRoyaleDomain> for RPSBattleRoyaleExecutorState {
    fn post_mcts_run_hook(&mut self, mcts: &MCTS<RPSBattleRoyaleDomain>, last_active_task: &ActiveTask<RPSBattleRoyaleDomain>) {
        if let Err(e) = plot_tree_in_tmp_with_task_name(mcts, "rps", last_active_task) {
            println!("Cannot write search tree: {e}");
        }
        if let Some(entry) = self.mcts_visits.get_mut(mcts.agent().0 as usize) {
            *entry += mcts.node_count() as u32;
        }
    }
}