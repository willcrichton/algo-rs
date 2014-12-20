use rustc::util::nodemap::FnvHasher;
use std::collections::HashMap;
use FnvMap;

pub type NodeIndex = uint;
pub type Edge = (NodeIndex, NodeIndex);

// TODO: write tests for Graph
pub struct Graph<N, E> {
    pub edges: FnvMap<NodeIndex, FnvMap<NodeIndex, E>>,
    pub nodes: FnvMap<NodeIndex, N>,
    counter: NodeIndex,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph {
            edges: HashMap::with_hasher(FnvHasher),
            nodes: HashMap::with_hasher(FnvHasher),
            counter: 0u,
        }
    }

    pub fn add_node(&mut self, value: N) -> NodeIndex {
        let index = self.counter;
        self.nodes.insert(index, value);
        self.counter += 1;
        return index;
    }

    pub fn add_edge(&mut self, (from, to): Edge, weight: E) {
        if !self.edges.contains_key(&from) {
            self.edges.insert(from, HashMap::with_hasher(FnvHasher));
        }

        let mut list = self.edges.get_mut(&from).unwrap();
        list.insert(to, weight);
    }

    pub fn neighbors(&self, node: &NodeIndex) -> &FnvMap<NodeIndex, E> {
        match self.edges.find(node) {
            Some(map) => map,
            None => panic!("node {} does not exist", node)
        }
    }

    pub fn find_path(&self, source: &NodeIndex, sink: &NodeIndex) -> Option<Vec<Edge>> {
        self.find_path_helper(source, sink, Vec::new())
    }

    fn find_path_helper(&self, source: &NodeIndex, sink: &NodeIndex, path: Vec<Edge>) -> Option<Vec<Edge>> {
        if source == sink { Some(path) }
        else {
            for neighbor in self.neighbors(source).keys() {
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

// HeapEdge is used for creating a min-heap over edges of the Graph
// in conjunction with std::collections::BinaryHeap

#[deriving(Eq, PartialEq)]
pub struct HeapEdge<E>(pub Edge, pub E);

impl<E: Ord> Ord<HeapEdge<E>> for HeapEdge<E> {
    fn cmp(&self, other: &HeapEdge<E>) -> Ordering {
        let (&HeapEdge(_, ref me), &HeapEdge(_, ref other)) = (self, other);
        match me.cmp(other) {
            Less => Greater,
            Equal => Equal,
            Greater => Less
        }
    }
}

impl<E: Ord> PartialOrd<HeapEdge<E>> for HeapEdge<E> {
    fn partial_cmp(&self, other: &HeapEdge<E>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}