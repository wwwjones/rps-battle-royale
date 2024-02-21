// This is a custom planning algorithm to replace the NPC Engine mcts algorithm.
// The mcts has proven to be unpredictable in simple situations and difficult to debug, and more importantly is too slow for a larger 
// number of agents.

// initially, this was a linked list, but rust hates the classics, so now this is a vector approach

// Where the mcts algorithm employs dynamic exploration and exploitation, this planner is built to try ALL possibilities for one agent
// for a few steps, while greedily sampling actions of  other agents to attempt a similar simulation-based approach.
// This planner is therefore more myopic in its approach, but its purpose is not to show great intelligence for a few agents,
// but rather middle-of-the-road intelligence for many.
// It uses the same node and edge datastructures as the mcts algorithm, as edges are also tasks, and this is implemented
// very well in the mcts approach.

// Edges is the 4 movement tasks when expanding for the tree root agent, and we force one edge - the greedily evaluated next move - when
// expanding for the other agents

// how to make breadth first - how to do backprop

use std::{borrow::BorrowMut, collections::BTreeSet, fmt};

use npc_engine_core::{AgentId, AgentValue, Context, ContextMut, Domain, StateDiffRef, Task};

use super::node::Node;

/// Pinky and the
struct Brain<D: Domain> {
    // how many steps do we want to simulate
    depth: u64,
    // agent for whom we are planning
    root_agent: AgentId,
    // root node
    nodes: Vec<(D::DisplayAction, Node<D>)>,
    // current scores for each edge from the root node
    scores: Vec<AgentValue>,
    // initial state before plannning
    initial_state: D::State,
    // starting tick
    start_tick: u64,
    // best task
    best_task: Option<Box<dyn Task<D>>>,
}

impl <D: Domain> Brain<D> {
    pub fn new(root_agent: AgentId, initial_state: D::State, start_tick: u64, depth: u64) -> Self {
        // some helpers
        let diff = D::Diff::default();
        let zero_score = AgentValue::new(0.0).unwrap();

        // create the root node
        let root_node: Node<D> = Node::new(diff.clone(), zero_score, None, Vec::new());

        // create the nodes vector
        let mut nodes = vec![(D::display_action_task_idle(), root_node)];
        let mut new_nodes = Vec::new();
        let mut children = Vec::new();

        // create children nodes by creating a list of valid tasks, executing them, and adding a node with a new diff for each
        let ctx = Context::with_state_and_diff(start_tick, &initial_state, &diff, root_agent);
        let valid_tasks = D::get_tasks(ctx);
        let mut index = 1;
        for task in valid_tasks.iter() {
            let mut diff = D::Diff::default();
            let ctx: ContextMut<'_, D> = ContextMut::with_state_and_diff(start_tick, &initial_state, &mut diff, root_agent);
            task.execute(ctx); // this changes the diff
            let score = D::get_current_value(start_tick, StateDiffRef::new(&initial_state, &diff), root_agent);
            let new_node: Node<D> = Node::new(diff, score, Some(0), Vec::new());
            new_nodes.push((task.display_action(), new_node));
            children.push((task.display_action(), index));
            index += 1;
        }

        // set scores slots
        let scores = vec![zero_score; new_nodes.len()];

        // set root node's children to point to the newly created nodes
        let root_node = nodes.first_mut().unwrap();
        root_node.1.add_children(children);

        // add the new nodes to our list
        nodes.append(&mut new_nodes);

        Self {
            depth,
            root_agent,
            nodes,
            scores,
            initial_state,
            start_tick,
            best_task: None,
        }


    }

    pub fn run(&mut self) {
        // other agent move, this agent's move, loop
        // we start with a root node with a child node for each valid move
        // want to go breadth first - increase the tick and iterate through the most recently added nodes
        let mut start_idx = 1; // we start with the first child
        let mut end_idx = self.nodes.len();

        // loop node creation until we have reach the desired depth
        for level in 0..self.depth {
            let tick = self.start_tick + level;

            for i in start_idx..end_idx {
                // for each node, we get a list of the other agents
                // for each agent, we get its possible tasks and execute each
                // each agent applies the best task to the diff
                // this diff is then used to find the next valid tasks for our root agent
                // for each valid task, we execute it and add a new node as in initialization
                // when each node has added its children, backpropogate the scores to the root
                // change the start and end indices and repeat for the next level
                // get the other visible agents

                //get the current node
                let start_node = self.nodes[i].1.borrow_mut();
                let diff = start_node.diff();
                let ctx = Context::with_state_and_diff(tick, &self.initial_state, diff, self.root_agent);
                let mut agents: BTreeSet<AgentId> = BTreeSet::new();
                D::update_visible_agents(tick, ctx, &mut agents);
            }
        }

        // once the tree has been fully created, simply pick the best action and return it
    }
}

impl<D: Domain> fmt::Debug for Brain<D> {
    fn fmt(&self, f: &'_ mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Brain")
            .field("\ndepth", &self.depth)
            .field("\nroot_agent", &self.root_agent)
            .field("\nnodes", &self.nodes)
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
        //println!("{:?}", brain);

        // try and get children
        let root_node = &brain.nodes.first().unwrap().1;

        let children = root_node.children();
        let child_1_idx = children[0].1;
        let child_2_idx = children[1].1;
        let child_3_idx = children[2].1;

        let child_1 = &brain.nodes[child_1_idx].1;
        let child_2 = &brain.nodes[child_2_idx].1;
        let child_3 = &brain.nodes[child_3_idx].1;

        println!("\nRoot node children:\n\n{:?}\n\n{:?}\n\n{:?}", child_1, child_2, child_3);

    }
}