// This is a custom planning algorithm to replace the NPC Engine mcts algorithm.
// The mcts has proven to be unpredictable in simple situations and difficult to debug, and more importantly is too slow for a larger 
// number of agents.
// This will be a similar tree-style data-structure, but it's rollout is a more lightweight, static approach.
// Where the mcts algorithm employs dynamic exploration and exploitation, this planner is built to try ALL possibilities for one agent
// for a few steps, while greedily sampling actions of  other agents to attempt a similar simulation-based approach.
// This planner is therefore more myopic in its approach, but its purpose is not to show great intelligence for a few agents,
// but rather middle-of-the-road intelligence for many.
// It uses the same node and edge datastructures as the mcts algorithm, as edges are also tasks, and this is implemented
// very well in the mcts approach.

// Edges is the 4 movement tasks when expanding for the tree root agent, and we force one edge - the greedily evaluated next move - when
// expanding for the other agents

// how to make breadth first - how to do backprop

use std::{fmt, sync::Mutex};

use npc_engine_core::{AgentId, AgentValue, Context, ContextMut, Domain, StateDiffRef, Task};

use super::node::{Node, NodeInner};

/// Pinky and the
struct Brain<D: Domain> {
    // how many steps do we want to simulate
    depth: u32,
    // agent for whom we are planning
    root_agent: AgentId,
    // root node
    root_node: Node<D>,
    // current scores for each edge from the root node
    scores: Vec<AgentValue>,
    // initial state before plannning
    initial_state: D::State,
    // best task
    best_task: Option<Box<dyn Task<D>>>,
}

impl <D: Domain> Brain<D> {
    pub fn new(root_agent: AgentId, initial_state: D::State, start_tick: u64, depth: u32) -> Self {
        // create a new, empty diff for the node
        let diff = D::Diff::default();
        // create root node with no children
        let zero_score = AgentValue::new(0.0).unwrap();
        let mut root_node: Node<D> = Node::new(Mutex::new(NodeInner::new(diff.clone(), zero_score, None, Vec::new())));
        let mut nodes: Vec<(D::DisplayAction, *mut Node<D>)> = Vec::new();
        // create children nodes by creating a list of valid tasks, executing them, and adding a node with a new diff for each
        let ctx = Context::with_state_and_diff(start_tick, &initial_state, &diff, root_agent);
        let valid_tasks = D::get_tasks(ctx);
        for task in valid_tasks.iter() {
            let mut diff = D::Diff::default();
            let ctx: ContextMut<'_, D> = ContextMut::with_state_and_diff(start_tick, &initial_state, &mut diff, root_agent);
            task.execute(ctx);
            println!("{:?}", &diff);
            let state_diff = StateDiffRef::new(&initial_state, &diff);
            let score = D::get_current_value(start_tick, state_diff, root_agent);
            nodes.push((task.display_action(), &mut Node::new(Mutex::new(NodeInner::new(diff, score, Some(&mut root_node), Vec::new())))));
        }
        // set scores slots
        let scores = vec![AgentValue::new(0.0).unwrap(); nodes.len()];
        // set root node's children to point to the newly created nodes
        root_node.lock().unwrap().add_children(nodes);

        Self {
            depth,
            root_agent,
            root_node,
            scores,
            initial_state,
            best_task: None,
        }


    }

    pub fn run(&mut self) {
        // when adding a new node:
        // expand the edge - execute the task of the root agent, add effect to diff
        // for each visible agent, execute its available tasks and see which one has the best immediate effect and store it in a list of activetasks

        // other agent move, this agent's move, loop

    }
}

impl<D: Domain> fmt::Debug for Brain<D> {
    fn fmt(&self, f: &'_ mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Brain")
            .field("\ndepth", &self.depth)
            .field("\nroot_agent", &self.root_agent)
            .field("\nroot_node", &self.root_node)
            .field("\nscores", &self.scores)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use npc_engine_core::AgentId;
    use npc_engine_utils::Coord2D;

    use crate::{domain::RPSBattleRoyaleDomain, state::AgentType, testing_factory::_create_test_state};

    use super::Brain;

    #[test]
    fn create_brain() {
        let agent_info = vec![(AgentType::Rock, Coord2D::new(0, 0)), (AgentType::Scissors, Coord2D::new(0, 1))];
        let root_agent = AgentId(1);
        let start_tick = 0;
        let depth = 5;
        let initial_state = _create_test_state(agent_info);
        let brain: Brain<RPSBattleRoyaleDomain> = Brain::new(root_agent, initial_state, start_tick, depth);
/*         println!("{:?}", brain);

        // try and access a child
        let root_binding = brain.root_node.lock().unwrap();
        let child_ptr = root_binding.children().first().unwrap().1;
        let child_binding = unsafe { child_ptr.read() };
        let child = child_binding.lock().unwrap();
        println!("\n{:?}", child.score()); */
    }
}