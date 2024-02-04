use npc_engine_core::{impl_task_boxed_methods, AgentId, Context, ContextMut, Task, TaskDuration};

use crate::{domain::{DisplayAction, RPSBattleRoyaleDomain}, state::{Access, AccessMut, AgentType}, CONVERT_WEIGHT};

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Convert(pub AgentId);

impl Task<RPSBattleRoyaleDomain> for Convert {
    fn weight(&self, _ctx: Context<RPSBattleRoyaleDomain>) -> f32 {
        CONVERT_WEIGHT
    }

    fn duration(&self, _ctx: Context<RPSBattleRoyaleDomain>) -> TaskDuration {
        1
    }

    fn execute(&self, mut ctx: ContextMut<RPSBattleRoyaleDomain>) -> Option<Box<dyn Task<RPSBattleRoyaleDomain>>> {
        // change the type of the target agent and reset its conversions
        let this_agent = ctx.state_diff.get_agent_mut(ctx.agent).unwrap();
        let new_type = this_agent.ty;
        this_agent.conversions += 1;
        

        let target_agent = ctx.state_diff.get_agent_mut(self.0).unwrap();
        target_agent.ty = new_type;
        target_agent.conversions = -1; // give them incentive to not get converted :)

        None
    }

    fn is_valid(&self, ctx: Context<RPSBattleRoyaleDomain>) -> bool {
        // valid if we are on the same spot as our victim and it is of the correct type
        true
/*         let this_agent = ctx.state_diff.get_agent(ctx.agent).unwrap();
        let target_agent = ctx.state_diff.get_agent(self.0).unwrap();

         if target_agent.ty != match this_agent.ty {
            AgentType::Rock => AgentType::Scissors,
            AgentType::Paper => AgentType::Rock,
            AgentType::Scissors => AgentType::Paper,
        } { return false; }

        this_agent.location == target_agent.location */
    }

    fn display_action(&self) -> DisplayAction {
        DisplayAction::Convert(self.0)
    }

    impl_task_boxed_methods!(RPSBattleRoyaleDomain);
}