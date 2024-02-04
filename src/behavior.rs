use npc_engine_core::{Behavior, Context, Task};
use npc_engine_utils::DIRECTIONS;

use crate::{domain::RPSBattleRoyaleDomain, tasks::r#move::Move};

pub struct Contestant;

impl Behavior<RPSBattleRoyaleDomain> for Contestant {
    fn is_valid(&self, _ctx: Context<RPSBattleRoyaleDomain>) -> bool {
        true
    }

    fn add_own_tasks(&self, ctx: Context<RPSBattleRoyaleDomain>, tasks: &mut Vec<Box<dyn Task<RPSBattleRoyaleDomain>>>) {
        for direction in DIRECTIONS {
            let task = Move(direction);
            if task.is_valid(ctx) {
                tasks.push(Box::new(task));
            }
        }

        //tasks.push(Box::new(IdleTask));
    }
}