use npc_engine_core::{impl_task_boxed_methods, Context, ContextMut, Task, TaskDuration};
use npc_engine_utils::Direction;

use crate::{
    constants::MOVE_WEIGHT,
    domain::{DisplayAction, RPSBattleRoyaleDomain},
    map::DirConv,
    state::{Access, AccessMut},
};

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
        // get new position and the type of this agent
        let agent_state = ctx.state_diff.get_agent(ctx.agent).unwrap();
        let this_agent_type = agent_state.ty;

        let target_pos = DirConv::apply(self.0, agent_state.location);
        //let target_pos = DirConv::apply(self.0, target_pos);

        // if there is an agent at the target location: convert
        if let Some(agent_state) = ctx.state_diff.get_agent_at_mut(target_pos) {
            let target_state = agent_state.1;

            // if the agent is this agent's prey:
            if target_state.ty == this_agent_type.prey() {
                // turn the target agent into the same type as this agent and reset its points
                target_state.ty = this_agent_type;
                target_state.conversions = -1;

                // give this agent a point
                let agent_state = ctx.state_diff.get_agent_mut(ctx.agent).unwrap();
                agent_state.conversions += 1;
            }

            // if the agent is this agent's predator:
            else if target_state.ty == this_agent_type.predator() {
                // give the target agent a point
                target_state.conversions += 1;
                let target_agent_type = target_state.ty;

                // turn this agent into the same type as the target agent and reset its points
                let agent_state = ctx.state_diff.get_agent_mut(ctx.agent).unwrap();
                agent_state.ty = target_agent_type;
                agent_state.conversions -= 1;
            }
        }

        // finally, carry out the move
        let agent_state = ctx.state_diff.get_agent_mut(ctx.agent).unwrap();
        agent_state.location = target_pos;

        None
    }

    fn is_valid(&self, ctx: Context<RPSBattleRoyaleDomain>) -> bool {
        let agent_state = ctx.state_diff.get_agent(ctx.agent).unwrap();
        let target_pos = DirConv::apply(self.0, agent_state.location);
        //let target_pos = DirConv::apply(self.0, target_pos);

        // check if the location is valid
        ctx.state_diff.is_location_valid(target_pos)

/*         // if the move is out of bounds - invalid
        if !ctx.state_diff.is_location_valid(target_pos) {
            return false;
        }
        let necessary_type = match agent_state.ty {
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

#[cfg(test)]
mod tests {
    use npc_engine_core::{AgentId, Context, ContextMut, Task};
    use npc_engine_utils::{Coord2D, Direction};

    use crate::{domain::RPSBattleRoyaleDomain, state::{AgentType, Diff, GlobalState}, testing_factory::_create_test_state};

    use super::Move;

    fn create_state() -> GlobalState {
        let rock = (AgentType::Rock, Coord2D::new(0, 3));
        let paper = (AgentType::Paper, Coord2D::new(0,2));
        let scissors = (AgentType::Scissors, Coord2D::new(1, 3));

        let agents = vec![rock, paper, scissors];
        _create_test_state(agents)
    }

/*     fn world_step(mut ctx: ContextMut<RPSBattleRoyaleDomain>) {
        // go through the list of converted agents and convert them
        for agent in ctx.state_diff.diff.converted_agents.clone() {
            let mut agent = ctx.state_diff.get_agent_mut(agent).unwrap();
            agent.conversions = -1;
            agent.ty = match agent.ty {
                AgentType::Rock => AgentType::Paper,
                AgentType::Paper => AgentType::Scissors,
                AgentType::Scissors => AgentType::Rock,
            };
        }
    } */
    
    fn apply(global_state: &mut GlobalState, diff: &Diff) {
        // apply diff to state
        if !diff.agents.is_empty() {
            println!("{:?}", &diff.agents);
        }
        for (agent, agent_state) in diff.agents.clone() {
            global_state.agents.insert(agent, agent_state);
        }
    }

    #[test]
    fn test_out_of_bounds() {
        let state = create_state();
        let diff = Diff::default();
        let ctx: Context<'_, RPSBattleRoyaleDomain> = Context::with_state_and_diff(0, &state, &diff, AgentId(0));

        let move_task = Move(Direction::Left);
        assert_eq!(move_task.is_valid(ctx), false);
    }

    #[test]
    fn test_valid_move() {
        let mut state = create_state();
        let mut diff = Diff::default();

        let move_task = Move(Direction::Down);
        let ctx: Context<'_, RPSBattleRoyaleDomain> = Context::with_state_and_diff(0, &state, &diff, AgentId(0));
        assert_eq!(move_task.is_valid(ctx), true);

        let ctx: ContextMut<'_, RPSBattleRoyaleDomain> = ContextMut::with_state_and_diff(0, &state, &mut diff, AgentId(0));
        move_task.execute(ctx);
        //let ctx: ContextMut<'_, RPSBattleRoyaleDomain> = ContextMut::with_state_and_diff(0, &state, &mut diff, AgentId(0));
        apply(&mut state, &diff);

        println!("\n{:?}\n", &state);
    }

    #[test]
    fn test_valid_positive_conversion() {
        let mut state = create_state();
        let mut diff = Diff::default();

        let move_task = Move(Direction::Right);
        let ctx: Context<'_, RPSBattleRoyaleDomain> = Context::with_state_and_diff(0, &state, &diff, AgentId(0));
        assert_eq!(move_task.is_valid(ctx), true);

        let ctx: ContextMut<'_, RPSBattleRoyaleDomain> = ContextMut::with_state_and_diff(0, &state, &mut diff, AgentId(0));
        move_task.execute(ctx);
        //let ctx: ContextMut<'_, RPSBattleRoyaleDomain> = ContextMut::with_state_and_diff(0, &state, &mut diff, AgentId(0));
        apply(&mut state, &diff);

        println!("\n{:?}\n", &state);
    }

    #[test]
    fn test_valid_negative_conversion() {
        let mut state = create_state();
        let mut diff = Diff::default();

        let move_task = Move(Direction::Up);
        let ctx: Context<'_, RPSBattleRoyaleDomain> = Context::with_state_and_diff(0, &state, &diff, AgentId(0));
        assert_eq!(move_task.is_valid(ctx), true);

        let ctx: ContextMut<'_, RPSBattleRoyaleDomain> = ContextMut::with_state_and_diff(0, &state, &mut diff, AgentId(0));
        move_task.execute(ctx);
        //let ctx: ContextMut<'_, RPSBattleRoyaleDomain> = ContextMut::with_state_and_diff(0, &state, &mut diff, AgentId(0));
        apply(&mut state, &diff);

        println!("\n{:?}\n", &state);
    }
}