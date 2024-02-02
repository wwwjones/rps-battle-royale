use std::collections::BTreeSet;

use npc_engine_core::AgentId;
use npc_engine_core::AgentValue;
use npc_engine_core::Behavior;
use npc_engine_core::Context;
use npc_engine_core::Domain;
use npc_engine_core::DomainWithPlanningTask;
use npc_engine_core::StateDiffRef;
use npc_engine_utils::Direction;
use npc_engine_utils::GlobalDomain;

use crate::state::*;


#[derive(Debug, Default)]
pub enum DisplayAction {
    #[default]
    Idle,
    Plan,
    Move(Direction),
    Convert,
}

pub struct RPSBattleRoyaleDomain;

impl Domain for RPSBattleRoyaleDomain {
    type State = GlobalState;
    type Diff = Diff;
    type DisplayAction = DisplayAction;

    fn list_behaviors() -> &'static [&'static dyn Behavior<Self>] {
        todo!()
    }

    fn get_current_value(tick: u64, state_diff: StateDiffRef<Self>, agent: AgentId) -> AgentValue {
        AgentValue::new(state_diff.get_agent(agent).unwrap().conversions as f32).unwrap()
    }

    fn update_visible_agents(start_tick: u64, ctx: Context<Self>, agents: &mut BTreeSet<AgentId>) {
        // clear the list
        agents.clear();
        // add all agents from the state
        agents.extend(ctx.state_diff.initial_state.agents.keys());   
    }

    fn display_action_task_planning() -> Self::DisplayAction {
        DisplayAction::Plan
    }
}

impl GlobalDomain for RPSBattleRoyaleDomain {
    type GlobalState = GlobalState;

    fn derive_local_state(global_state: &Self::GlobalState, agent: AgentId) -> Self::State {
        // right now, consider all agents, local state is global state
        global_state.clone()
    }

    fn apply(global_state: &mut Self::GlobalState, local_state: &Self::State, diff: &Self::Diff) {
        // as the map is the same, simply update the agents states
        for (agent, agent_state) in diff.agents.clone() {
            global_state.agents.insert(agent, agent_state);
        }
    }
}

impl DomainWithPlanningTask for RPSBattleRoyaleDomain {}
