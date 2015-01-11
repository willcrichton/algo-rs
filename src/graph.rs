//! Custom graph structure/implementation. Required for use with graph algorithms.

use std::collections::HashMap;
use std::default::Default;
use std::cmp::Ordering;

use FnvMap;

pub trait Graph {
    type NodeValue;
    type EdgeValue;
    type NodeIndex: Copy;

    /// Returns a vector of all nodes in the graph.
    fn nodes(&self) -> Vec<Self::NodeIndex>;

    /// Returns a vector of all edges in the form (from, to).
    fn edges(&self) -> Vec<(Self::NodeIndex, Self::NodeIndex, &Self::EdgeValue)>;

    /// Adds a new node to the graph and returns its index.
    fn add_node(&mut self, Self::NodeValue) -> Self::NodeIndex;

    /// Adds a new edge to the graph.
    fn add_edge(&mut self, Self::NodeIndex, Self::NodeIndex, Self::EdgeValue);

    /// Returns a map of the out-neighbors of a node to the weights of the edges from the node to the out-neighbors.
    fn neighbors(&self, Self::NodeIndex) -> Vec<Self::NodeIndex>;

    /// Returns list of edges on an arbitrary path from source to sink.
    fn find_path(&self, Self::NodeIndex, Self::NodeIndex) ->
        Option<Vec<(Self::NodeIndex, Self::NodeIndex)>>;
}

type NodeIndex = usize;

// TODO: write tests for this
#[derive(Eq, PartialEq)]
pub struct AdjacencyList<N, E> {
    edges: FnvMap<NodeIndex, FnvMap<NodeIndex, E>>,
    nodes: FnvMap<NodeIndex, N>,
    counter: NodeIndex,
}

impl<N, E> AdjacencyList<N, E> {
    pub fn new() -> Self {
        AdjacencyList {
            edges: HashMap::with_hash_state(Default::default()),
            nodes: HashMap::with_hash_state(Default::default()),
            counter: 0us,
        }
    }

    fn find_path_helper(&self, source: NodeIndex, sink: NodeIndex, path: Vec<(NodeIndex, NodeIndex)>) -> Option<Vec<(NodeIndex, NodeIndex)>> {
        if source == sink { Some(path) }
        else {
            for neighbor in self.neighbors(source).into_iter() {
                let edge = (source.clone(), neighbor.clone());
                if path.contains(&edge) { continue; }
                let mut new_path = path.clone();
                new_path.push(edge);

                match self.find_path_helper(neighbor, sink, new_path) {
                    Some(p) => { return Some(p) },
                    None => {}
                }
            }

            None
        }
    }
}

impl<N, E> Graph for AdjacencyList<N, E> {
    type NodeValue = N;
    type EdgeValue = E;
    type NodeIndex = NodeIndex;

    fn nodes(&self) -> Vec<NodeIndex> {
        self.nodes.keys().map(|x| *x).collect()
    }

    fn edges(&self) -> Vec<(NodeIndex, NodeIndex, &E)> {
        self.edges.iter()
            .flat_map(|(from, neighbors)| {
                neighbors.iter().map(move |(to, value)| (*from, *to, value))
            })
            .collect()
    }

    fn add_node(&mut self, value: N) -> NodeIndex {
        let index = self.counter;
        self.nodes.insert(index, value);
        self.counter += 1;
        return index;
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: E) {
        if !self.edges.contains_key(&from) {
            self.edges.insert(from, HashMap::with_hash_state(Default::default()));
        }

        let mut list = self.edges.get_mut(&from).unwrap();
        list.insert(to, weight);
    }

    fn neighbors(&self, node: NodeIndex) -> Vec<NodeIndex> {
        match self.edges.get(&node) {
            Some(map) => map.keys().map(|x| *x).collect(),
            None => panic!("node {} does not exist", node)
        }
    }

    fn find_path(&self, source: NodeIndex, sink: NodeIndex) -> Option<Vec<(NodeIndex, NodeIndex)>> {
        self.find_path_helper(source, sink, Vec::new())
    }
}

// HeapEdge is used for creating a min-heap over edges of the Graph
// in conjunction with std::collections::BinaryHeap

#[derive(PartialEq, Eq)]
pub struct HeapEdge<G: Graph>(pub (G::NodeIndex, G::NodeIndex), pub G::EdgeValue) where G::EdgeValue: Eq, G::NodeIndex: Eq;

impl<G: Graph> Ord for HeapEdge<G> where G: Eq, G::NodeIndex: Eq, G::EdgeValue: Ord {
    fn cmp(&self, other: &HeapEdge<G>) -> Ordering {
        use std::cmp::Ordering::{Less, Greater, Equal};
        let (&HeapEdge(_, ref me), &HeapEdge(_, ref other)) = (self, other);
        match me.cmp(other) {
            Less => Greater,
            Equal => Equal,
            Greater => Less
        }
    }
}

impl<G: Graph> PartialOrd for HeapEdge<G> where G: Eq, G::NodeIndex: Eq, G::EdgeValue: Ord {
    fn partial_cmp(&self, other: &HeapEdge<G>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}