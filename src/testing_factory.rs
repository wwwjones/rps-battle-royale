use npc_engine_core::AgentId;
use npc_engine_utils::Coord2D;

use crate::{map::Map, state::{AgentState, AgentType, Agents, GlobalState}};

pub fn _create_test_state(agent_info: Vec<(AgentType, Coord2D)>) -> GlobalState {
    let map = Map::new(10, 10);
    let mut agents = Agents::new();
    let mut id = 1;

    for (ty, location) in &agent_info {
        agents.insert(
            AgentId(id),
            AgentState{ 
                ty: *ty, 
                location: *location, 
                conversions: 0, 
            }
        );
        id += 1;
    }

    GlobalState { map, agents }
}