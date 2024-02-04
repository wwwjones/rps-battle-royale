use npc_engine_core::{impl_task_boxed_methods, Context, ContextMut, Task, TaskDuration};
use npc_engine_utils::Direction;

use crate::{
    constants::MOVE_WEIGHT,
    domain::{DisplayAction, RPSBattleRoyaleDomain},
    map::DirConv,
    state::{Access, AccessMut, AgentType},
};

use super::convert::Convert;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Move(pub Direction);

impl Task<RPSBattleRoyaleDomain> for Move {
    fn weight(&self, _ctx: Context<RPSBattleRoyaleDomain>) -> f32 {
        MOVE_WEIGHT
    }

    fn duration(&self, _ctx: Context<RPSBattleRoyaleDomain>) -> TaskDuration {
        0
    }

    fn execute(
        &self,
        mut ctx: ContextMut<RPSBattleRoyaleDomain>,
    ) -> Option<Box<dyn Task<RPSBattleRoyaleDomain>>> {


        let agent_state = ctx.state_diff.get_agent(ctx.agent).unwrap();
        let necessary_type = match agent_state.ty {
            AgentType::Rock => AgentType::Scissors,
            AgentType::Paper => AgentType::Rock,
            AgentType::Scissors => AgentType::Paper,
        };
        let target_pos = DirConv::apply(self.0, agent_state.location);
        let agent_state = ctx.state_diff.get_agent_mut(ctx.agent).unwrap();
        agent_state.location = target_pos;

        if let Some((id, state)) = ctx.state_diff.get_agent_at(target_pos) {
            if state.ty == necessary_type {
                Some(Box::new(Convert(id)))
            } else {
                None
            }
        } else {
            None
        }

    }

    fn is_valid(&self, ctx: Context<RPSBattleRoyaleDomain>) -> bool {
        let agent_state = ctx.state_diff.get_agent(ctx.agent).unwrap();
        let target_pos = DirConv::apply(self.0, agent_state.location);
        ctx.state_diff.is_location_valid(target_pos)
/*         let necessary_type = match agent_state.ty {
            AgentType::Rock => AgentType::Scissors,
            AgentType::Paper => AgentType::Rock,
            AgentType::Scissors => AgentType::Paper,
        };
        // valid if move is onto an empty spot or a spot occupied by the target type
        let target_type = ctx.state_diff.agent_type_at_location(target_pos);
        target_type == Some(necessary_type) || target_type == None */
    }

    fn display_action(&self) -> DisplayAction {
        DisplayAction::Move(self.0)
    }

    impl_task_boxed_methods!(RPSBattleRoyaleDomain);
}
