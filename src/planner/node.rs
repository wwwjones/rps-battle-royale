use std::{fmt, sync::{Arc, Mutex}};

use npc_engine_core::{AgentValue, Domain};

pub type Node<D> = Arc<Mutex<NodeInner<D>>>;

#[derive(Clone)]
pub struct NodeInner<D: Domain> {
    // the cumulative changes at this node
    diff: D::Diff,
    // the score at this node
    score: AgentValue,
    // pointer to the parent node, or None if this is the root
    parent: Option<*mut Node<D>>,
    // vector of children nodes, with their respective tasks
    children: Vec<(D::DisplayAction, *mut Node<D>)>,
}

impl <D: Domain> NodeInner<D> {
    pub fn new(diff: D::Diff, score: AgentValue, parent: Option<*mut Node<D>>, children: Vec<(D::DisplayAction, *mut Node<D>)>) -> Self {
        Self {
            diff,
            score,
            parent,
            children,
        }
    }
    pub fn add_children(&mut self, children: Vec<(D::DisplayAction, *mut Node<D>)>) {
        self.children = children;
    }
    pub fn diff(&self) -> &D::Diff {
        &self.diff
    }
    pub fn score(&self) -> AgentValue {
        self.score
    }
    pub fn children(&self) -> &Vec<(D::DisplayAction, *mut Node<D>)> {
        &self.children
    }
}

impl<D: Domain> fmt::Debug for NodeInner<D> {
    fn fmt(&self, f: &'_ mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NodeInner")
            .field("\ndiff", &self.diff)
            .field("\nscore", &self.score)
            .field("\nparent", &self.parent)
            .field("\nchildren", &self.children)
            .finish()
    }
}