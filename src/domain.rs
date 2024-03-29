use std::collections::BTreeSet;

use npc_engine_core::AgentId;
use npc_engine_core::AgentValue;
use npc_engine_core::Behavior;
use npc_engine_core::Context;
use npc_engine_core::Domain;
use npc_engine_core::DomainWithPlanningTask;
use npc_engine_core::StateDiffRef;
use npc_engine_utils::Coord2D;
use npc_engine_utils::Direction;
use npc_engine_utils::GlobalDomain;

use crate::behavior::Contestant;
use crate::state::*;
use crate::VISIBILITY_DISTANCE;


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum DisplayAction {
    #[default]
    Idle,
    Plan,
    Move(Direction),
}

pub struct RPSBattleRoyaleDomain;

impl Domain for RPSBattleRoyaleDomain {
    type State = GlobalState;
    type Diff = Diff;
    type DisplayAction = DisplayAction;

    fn list_behaviors() -> &'static [&'static dyn Behavior<Self>] {
        &[&Contestant]
    }

    fn get_current_value(_tick: u64, state_diff: StateDiffRef<Self>, agent: AgentId) -> AgentValue {
        let agent = state_diff.get_agent(agent).unwrap();
        let point_value = state_diff.initial_state.map.longest_dist();
        let conversions = (agent.conversions as f32) * point_value;
        let distance = 3. * state_diff.initial_state.map.distance_points(agent.location);
        AgentValue::new(conversions + distance).unwrap()
    }

    /// update agents to include all other agents within a given manhattan distance
    fn update_visible_agents(_start_tick: u64, ctx: Context<Self>, agents: &mut BTreeSet<AgentId>) {
        // clear the list
        agents.clear();
        // add all agents from the state
        // TODO: limit the horizon to not all of the dudes...
        // sort the list of agents by increasing distance, pick the first x
        // maybe make sure we include 1 or 2 of each type?
        // or just purely do it with distance --> either change the project so we use local state again or just do it here
        // yeah, just manhattan distance of x
        let root_agent = ctx.agent;
        let agent_loc = ctx.state_diff.get_agent(root_agent).unwrap().location;
        for (id, state) in &ctx.state_diff.initial_state.agents {
            if Coord2D::manhattan_dist(&state.location, agent_loc) < VISIBILITY_DISTANCE && *id != root_agent {
                agents.insert(*id);
            }
        }
        //agents.extend(ctx.state_diff.initial_state.agents.keys());
    }

    fn display_action_task_planning() -> Self::DisplayAction {
        DisplayAction::Plan
    }
}

impl GlobalDomain for RPSBattleRoyaleDomain {
    type GlobalState = GlobalState;

    fn derive_local_state(global_state: &Self::GlobalState, _agent: AgentId) -> Self::State {
        // right now, consider all agents, local state is global state
        global_state.clone()
    }

    fn apply(global_state: &mut Self::GlobalState, _local_state: &Self::State, diff: &Self::Diff) {
        // as the map is the same, simply update the agents states
        for (agent, agent_state) in diff.agents.clone() {
            global_state.agents.insert(agent, agent_state);
        }
    }
}

impl DomainWithPlanningTask for RPSBattleRoyaleDomain {}
