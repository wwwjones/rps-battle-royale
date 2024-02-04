use std::{collections::HashMap, fmt::Display};

use npc_engine_core::{AgentId, StateDiffRef, StateDiffRefMut};
use npc_engine_utils::{keep_second_mut, Coord2D};

use crate::{domain::RPSBattleRoyaleDomain, map::Map};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AgentType {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct AgentState {
    pub ty: AgentType,
    pub location: Coord2D,
    pub conversions: i64,
    pub converted: bool,
}

pub type Agents = HashMap<AgentId, AgentState>;

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub map: Map,
    pub agents: Agents,
}

impl Display for GlobalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

/*         for (id, state) in &self.agents {
            writeln!(f, "{id}, {}", state.location)?;
        } */

        // collect agents into a map indexed by locations
        // go through each map coordinate from top left to bottom right and print 
        // a background for every coord, putting an agent on top if necessary
        let mut agents_map = HashMap::new();
        for (_, agent_state) in self.agents.iter() {
            agents_map
                .entry(agent_state.location)
                .or_insert_with(|| agent_state.ty);
        }
        let width = self.map.width();
        let height = self.map.height();
        for y in 0..height {
            for x in 0..width {
                use ansi_term::Colour::Fixed;
                let background = Fixed(241);
                let location = Coord2D::new(x as i32, y as i32);
                let text = if let Some(agent) = agents_map.get(&location) {
                    match agent {
                        AgentType::Rock => "🪨",
                        AgentType::Paper => "📄",
                        AgentType::Scissors => "✂️",
                    }
                } else {
                    "  "
                };
                write!(f, "{}", Fixed(0).on(background).paint(text))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct Diff {
    pub agents: Vec<(AgentId, AgentState)>,
}
impl Diff {
    fn has_agent(&self, agent: AgentId) -> bool {
        self.get_agent(agent).is_some()
    }
    fn get_agent(&self, agent: AgentId) -> Option<&AgentState> {
        self.agents
            .iter()
            .find_map(|(id, state)| if *id == agent { Some(state) } else { None })
    }
    fn get_agent_mut(&mut self, agent: AgentId) -> Option<&mut AgentState> {
        self.agents
            .iter_mut()
            .find_map(|(id, state)| if *id == agent { Some(state) } else { None })
    }
    #[allow(dead_code)]
    pub(crate) fn get_agent_ids(&self) -> impl Iterator<Item = AgentId> + '_ {
        self.agents.iter().map(|(agent, _)| *agent)
    }
}

pub trait Access {
    fn map_height(&self) -> u32;
    fn map_width(&self) -> u32;
    fn list_agents(&self) -> Vec<AgentId>;
    fn get_agent(&self, agent: AgentId) -> Option<&AgentState>;
    fn get_agent_at(&self, location: Coord2D) -> Option<(AgentId, &AgentState)>;
    fn agent_type_at_location(&self, location: Coord2D) -> Option<AgentType>;
    fn is_location_valid(&self, location: Coord2D) -> bool;
}

pub trait AccessMut: std::ops::Deref
where
    Self::Target: Access,
{
    fn get_agent_mut(&mut self, agent: AgentId) -> Option<&mut AgentState>;
    fn get_agent_at_mut(&mut self, location: Coord2D) -> Option<(AgentId, &mut AgentState)> {
        let agent = self.get_agent_at(location)?.0;
        self.get_agent_mut(agent)
            .map(|agent_state| (agent, agent_state))
    }
}

impl Access for StateDiffRef<'_, RPSBattleRoyaleDomain> {
    fn map_height(&self) -> u32 {
        self.initial_state.map.height()
    }

    fn map_width(&self) -> u32 {
        self.initial_state.map.width()
    }

    fn list_agents(&self) -> Vec<AgentId> {
        self.initial_state.agents.keys().copied().collect()
    }

    fn get_agent(&self, agent: AgentId) -> Option<&AgentState> {
        self.diff
            .get_agent(agent)
            .or_else(|| self.initial_state.agents.get(&agent))
    }

    fn get_agent_at(&self, location: Coord2D) -> Option<(AgentId, &AgentState)> {
        fn filter_location(
            location: Coord2D,
            id: AgentId,
            state: &AgentState,
        ) -> Option<(AgentId, &AgentState)> {
            if state.location == location {
                Some((id, state))
            } else {
                None
            }
        }
        fn get_agent_at_location(
            location: Coord2D,
            candidates: &[(AgentId, AgentState)],
        ) -> Option<(AgentId, &'_ AgentState)> {
            candidates
                .iter()
                .find_map(|(id, state)| filter_location(location, *id, state))
        }
        get_agent_at_location(location, &self.diff.agents)
            .or_else(|| {
                self.initial_state.agents.iter().find_map(|(id, state)| {
                    if self.diff.get_agent(*id).is_some() {
                        None
                    } else {
                        filter_location(location, *id, state)
                    }
                })
            })
    }

    fn agent_type_at_location(&self, location: Coord2D) -> Option<AgentType> {
        if let Some((_, agent)) = self.get_agent_at(location) {
            Some(agent.ty)
        } else {
            None
        }
    }

    fn is_location_valid(&self, location: Coord2D) -> bool {
        !self.initial_state.map.out_of_bounds(location)
    }
}

impl AccessMut for StateDiffRefMut<'_, RPSBattleRoyaleDomain> {
    fn get_agent_mut(&mut self, agent: AgentId) -> Option<&mut AgentState> {
        if self.diff.has_agent(agent) {
            self.diff.get_agent_mut(agent)
        } else {
            self.initial_state
                .agents
                .get(&agent)
                .and_then(|agent_state| {
                    self.diff.agents.push((agent, agent_state.clone()));
                    self.diff.agents.last_mut().map(keep_second_mut)
                })
        }
    }
}