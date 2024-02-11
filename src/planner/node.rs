use std::fmt;

use npc_engine_core::{AgentValue, Domain};

#[derive(Clone)]
pub struct Node<D: Domain> {
    // the cumulative changes at this node
    diff: D::Diff,
    // the score at this node
    score: AgentValue,
    // pointer to the parent node, or None if this is the root
    parent: Option<usize>,
    // vector of children nodes, with their respective tasks
    children: Vec<(D::DisplayAction, usize)>,
}

impl <D: Domain> Node<D> {
    pub fn new(diff: D::Diff, score: AgentValue, parent: Option<usize>, children: Vec<(D::DisplayAction, usize)>) -> Self {
        Self {
            diff,
            score,
            parent,
            children,
        }
    }
    pub fn add_children(&mut self, children: Vec<(D::DisplayAction, usize)>) {
        self.children = children;
    }
    pub fn diff(&self) -> &D::Diff {
        &self.diff
    }
    pub fn score(&self) -> AgentValue {
        self.score
    }
    pub fn children(&self) -> &Vec<(D::DisplayAction, usize)> {
        &self.children
    }
}

impl<D: Domain> fmt::Debug for Node<D> {
    fn fmt(&self, f: &'_ mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("\ndiff", &self.diff)
            .field("\nscore", &self.score)
            .field("\nparent", &self.parent)
            .field("\nchildren", &self.children)
            .finish()
    }
}