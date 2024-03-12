use npc_engine_core::{Behavior, Context, IdleTask, Task};
use npc_engine_utils::{Direction, DIRECTIONS};

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{domain::RPSBattleRoyaleDomain, tasks::r#move::Move};

pub struct Contestant;

impl Behavior<RPSBattleRoyaleDomain> for Contestant {
    fn is_valid(&self, _ctx: Context<RPSBattleRoyaleDomain>) -> bool {
        true
    }

    fn add_own_tasks(&self, ctx: Context<RPSBattleRoyaleDomain>, tasks: &mut Vec<Box<dyn Task<RPSBattleRoyaleDomain>>>) {
        let mut directions = vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ];
        directions.shuffle(&mut thread_rng());
        for direction in directions {
            let task = Move(direction);
            if task.is_valid(ctx) {
                tasks.push(Box::new(task));
            }
        }
        if tasks.is_empty() {
            log::error!("Couldnt add any movement tasks\n")
        }
        //tasks.push(Box::new(IdleTask));
        
    }
}