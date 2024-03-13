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

use core::fmt;
use std::{borrow::BorrowMut, collections::BTreeSet, num::NonZeroU64};

use npc_engine_core::{AgentId, AgentValue, Context, ContextMut, Domain, StateDiffRef, Task};

use super::node::Node;

#[derive(Clone, Debug, Default)]
pub struct BRAINConfiguration {
    pub depth: u32,
    pub planning_task_duration: Option<NonZeroU64>,
}

/// Pinky and the
#[derive(Clone)]
pub struct Brain<D: Domain> {
    // how many steps do we want to simulate
    depth: u32,
    // agent for whom we are planning
    root_agent: AgentId,
    // root node
    //nodes: Vec<(D::DisplayAction, Node<D>)>,
    nodes: Vec<Node<D>>,
    //
    // current scores for each edge from the root node
    scores: Vec<AgentValue>,
    // initial state before plannning
    initial_state: D::State,
    // starting tick
    start_tick: u64,
    // best task candidates
    candidates: Vec<Box<dyn Task<D>>>,
    // best task
    best_task: Option<Box<dyn Task<D>>>,
}

impl<D: Domain> fmt::Debug for Brain<D> {
    fn fmt(&self, f: &'_ mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Brain")
            .field("\ndepth", &self.depth)
            .field("\nroot agent", &self.root_agent)
            .field("\nnodes", &self.nodes)
            .field("\nbest_task", &self.best_task)
            .finish()
    }
}

impl <D: Domain> Brain<D> {
    pub fn _print(&self) {
        println!("Brain:");
        println!("  root agent: {}", self.root_agent);
        println!("  depth: {}", self.depth);
        println!("  best task: {:?}", self.best_task.as_ref().unwrap());
        println!("  candidates: {:?}", self.candidates);
        println!("  scores: {:?}", self.scores);
        println!("  nodes:");

        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            println!("    {i}:");
            println!("    diff: {:?}", node.diff());
            println!("    score: {}", node.score());
            println!("    parent: {:?}", node.parent());
            println!("    og parent: {:?}", node.og_parent());
            println!("    children: {:?}\n", node.children());
        }
    }

    pub fn best_task(&self) -> Option<Box<dyn Task<D>>> {
        self.best_task.clone()
    }

    pub fn new(root_agent: AgentId, initial_state: D::State, start_tick: u64, depth: u32) -> Self {
        // some helpers
        let diff = D::Diff::default();
        let zero_score = AgentValue::new(0.0).unwrap();

        // create the root node
        let root_node: Node<D> = Node::new(diff.clone(), zero_score, None, None, Vec::new());

        // create the nodes vector
        //let mut nodes = vec![(D::display_action_task_idle(), root_node)];
        let mut nodes = vec![root_node];
        let mut new_nodes = Vec::new();
        let mut children = Vec::new();

        // create children nodes by creating a list of valid tasks, executing them, and adding a node with a new diff for each
        let ctx = Context::with_state_and_diff(start_tick, &initial_state, &diff, root_agent);
        let candidates: Vec<Box<dyn Task<D>>> = D::get_tasks(ctx);
        let mut index = 1;
        for task in candidates.iter() {
            let mut diff = D::Diff::default();
            let ctx: ContextMut<'_, D> = ContextMut::with_state_and_diff(start_tick, &initial_state, &mut diff, root_agent);
            task.execute(ctx); // this changes the diff
            let score = D::get_current_value(start_tick, StateDiffRef::new(&initial_state, &diff), root_agent);
            let new_node: Node<D> = Node::new(diff, score, Some(0), Some(index), Vec::new());
            //new_nodes.push((task.display_action(), new_node));
            new_nodes.push(new_node);
            children.push(index);
            index += 1;
        }

        // set scores slots
        let scores = vec![zero_score; new_nodes.len()];

        // set root node's children to point to the newly created nodes
        let root_node = nodes.first_mut().unwrap();
        root_node.add_children(children);

        // add the new nodes to our list
        nodes.append(&mut new_nodes);

        Self {
            depth,
            root_agent,
            nodes,
            scores,
            initial_state,
            start_tick,
            candidates,
            best_task: None,
        }


    }

    // move right seems to actually be moving it right and down -- double check which diffs are being used and where execution is happening
    // maybe also add tick to the node - for debugging

    // creation already kind of does one step...
    // maybe change creation to not include children yet and change run to start with the root agent's turn, makes more sense

    pub fn run(&mut self) {
        // other agent move, this agent's move, loop
        // we start with a root node with a child node for each valid move
        // want to go breadth first - increase the tick and iterate through the most recently added nodes
        let mut end_idx = 1; // initialize this to 1, so start_idx is correct

        // loop node creation until we have reach the desired depth
        for level in 1..=self.depth {
            let tick = self.start_tick + (level as u64);

            let start_idx = end_idx;
            end_idx = self.nodes.len();
            let mut index = end_idx;
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
                let start_node = self.nodes[i].borrow_mut();
                let mut diff = start_node.diff().clone();
                let ctx = Context::with_state_and_diff(tick, &self.initial_state, &diff, self.root_agent);
                let mut agents: BTreeSet<AgentId> = BTreeSet::new();
                D::update_visible_agents(tick, ctx, &mut agents);

                // agents is the list of agents whose tasks we now have to pick
                // for each agent, go through each task and pick the one with the best immediate score and execute it, adding it to the diff for the next agent
                for agent in agents {
                    let ctx = Context::with_state_and_diff(tick, &self.initial_state, &diff, agent);
                    let valid_tasks = D::get_tasks(ctx);
                    let mut highscore: (AgentValue, D::DisplayAction, D::Diff) = (AgentValue::new(0.0).unwrap(), D::display_action_task_idle(), D::Diff::default()); // top choice of task
                    // loop through the tasks and find the best one
                    for task in valid_tasks.iter() {
                        let mut temp_diff = diff.clone();
                        let ctx: ContextMut<'_, D> = ContextMut::with_state_and_diff(tick, &self.initial_state, &mut temp_diff, agent);
                        task.execute(ctx); // this changes the diff
                        let score = D::get_current_value(tick, StateDiffRef::new(&self.initial_state, &temp_diff), agent);
                        if score > highscore.0 {
                            highscore = (score, task.display_action(), temp_diff.clone());
                        }
                    }
                    diff = highscore.2.clone();
                }
                // diff should now hold the moves of all the other visible agents
                // we now create a node for each currently valid task and connect them to our parent node
                let mut new_nodes = Vec::new();
                let mut children = Vec::new();

                let ctx = Context::with_state_and_diff(tick, &self.initial_state, &diff, self.root_agent);
                let valid_tasks = D::get_tasks(ctx);
                let mut high_score = AgentValue::new(f32::MIN).unwrap();
                let og_parent = start_node.og_parent().unwrap();
                for task in valid_tasks.iter() {
                    let mut new_diff = diff.clone();
                    let ctx = ContextMut::with_state_and_diff(tick, &self.initial_state, &mut new_diff, self.root_agent);
                    task.execute(ctx);
                    let score = D::get_current_value(tick, StateDiffRef::new(&self.initial_state, &new_diff), self.root_agent);
                    if score > high_score {
                        high_score = score;
                    }
                    let new_node: Node<D> = Node::new(new_diff, score, Some(i), Some(og_parent), Vec::new());
                    new_nodes.push(new_node);
                    children.push(index);
                    index += 1;
                }

                // update the scores list to contain the highest score of the new level
                self.scores[og_parent-1] = high_score;

                start_node.add_children(children);
                self.nodes.append(&mut new_nodes);
            }
            // here, each node from this level has added its children            
        }
        // here, the tree should be fully created up to the specified level
        // the highest scores at the deepest child node are stored in self.scores at the index of the first child that leads to that path
        // return the task that corresponds to the path with the highest score

        let mut best_task = (AgentValue::new(f32::MIN).unwrap(), 0);
        for i in 0..self.scores.len() {
            if self.scores[i] > best_task.0 {
                best_task = (self.scores[i], i);
            }
        }

        self.best_task = self.candidates.get(best_task.1).cloned();
    }
}

#[cfg(test)]
mod tests {
    use npc_engine_core::AgentId;
    use npc_engine_utils::Coord2D;

    use crate::{domain::RPSBattleRoyaleDomain, state::AgentType, testing_factory::_create_test_state};

    use super::Brain;

    fn create_brain() -> Brain<RPSBattleRoyaleDomain> {
        let agent_info = vec![(AgentType::Rock, Coord2D::new(10, 10)), (AgentType::Scissors, Coord2D::new(10, 0))];
        let root_agent = AgentId(1);
        let start_tick = 0;
        let depth = 1;
        let initial_state = _create_test_state(agent_info);
        let brain: Brain<RPSBattleRoyaleDomain> = Brain::new(root_agent, initial_state, start_tick, depth);
        brain
    }

    #[test]
    fn test_brain_creation() {
        let brain: Brain<RPSBattleRoyaleDomain> = create_brain();
        //println!("{:?}", brain);

        // try and get children
        let root_node = &brain.nodes.first().unwrap();

        let children = root_node.children();
        let child_1_idx = children[0];
        let child_2_idx = children[1];
        let child_3_idx = children[2];

        let child_1 = &brain.nodes[child_1_idx];
        let child_2 = &brain.nodes[child_2_idx];
        let child_3 = &brain.nodes[child_3_idx];

        println!("\nRoot node children:\n\n{:?}\n\n{:?}\n\n{:?}", child_1, child_2, child_3);
    }

    #[test]
    fn think() {
        let mut brain = create_brain();

        brain.run();

        brain._print();
    }
}