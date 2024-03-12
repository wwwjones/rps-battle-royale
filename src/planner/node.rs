use std::fmt;

use npc_engine_core::{AgentValue, Domain};

#[derive(Clone)]
pub struct Node<D: Domain> {
    // the cumulative changes at this node
    diff: D::Diff,
    // the score at this node
    score: AgentValue,
    // index of the parent node, or None if this is the root
    parent: Option<usize>,
    // index of the earliest parent node in the tree
    og_parent: Option<usize>,
    // vector of children nodes, with their respective tasks
    children: Vec<(D::DisplayAction, usize)>,
}

impl <D: Domain> Node<D> {
    pub fn new(diff: D::Diff, score: AgentValue, parent: Option<usize>, og_parent: Option<usize>, children: Vec<(D::DisplayAction, usize)>) -> Self {
        Self {
            diff,
            score,
            parent,
            og_parent,
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
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }
    /// return the index of the earliest parent in the tree (just don't call this on the root node please)
    pub fn og_parent(&self) -> Option<usize> {
        self.og_parent
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
            .field("\nOG parent", &self.og_parent)
            .field("\nchildren", &self.children)
            .finish()
    }
}