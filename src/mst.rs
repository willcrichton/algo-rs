//! Implements [minimum spanning tree algorithms](http://en.wikipedia.org/wiki/Minimum_spanning_tree) on graphs.

use std::collections::{HashSet, BinaryHeap};
use std::hash::Hash;

use graph::{Graph, HeapEdge};

/// Given a graph G, if G is connected, returns the edges in the MST of G, otherwise None.
pub trait MinimumSpanningTree<G: Graph> where G::EdgeValue: Ord {
    fn minimum_spanning_tree(&self, graph: &G) -> Option<Vec<(G::NodeIndex, G::NodeIndex)>>;
}

pub struct Kruskals;

impl<G: Graph> MinimumSpanningTree<G> for Kruskals
    where G: Eq, G::NodeIndex: Hash + Eq, G::EdgeValue: Ord + Copy
{
    fn minimum_spanning_tree(&self, graph: &G) -> Option<Vec<(G::NodeIndex, G::NodeIndex)>> {
        let mut vertices: HashSet<G::NodeIndex> = HashSet::new();
        let mut edges = Vec::new();

        let mut edge_heap = BinaryHeap::new();
        for (from, to, value) in graph.edges().into_iter() {
            edge_heap.push(HeapEdge::<G>((from, to), *value));
        }

        while vertices.len() < graph.nodes().len() {
            let HeapEdge((from, to), _) = match edge_heap.pop() {
                Some(edge) => edge,
                None => { return None; }
            };

            if !vertices.contains(&from) || !vertices.contains(&to) {
                vertices.insert(from);
                vertices.insert(to);
                edges.push((from, to));
            }
        }

        Some(edges)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use graph::{AdjacencyList, Graph};

    #[test]
    fn simple() {
        let mut graph: AdjacencyList<(), usize> = AdjacencyList::new();
        let mut vertices = Vec::new();

        for _ in 0..3 {
            vertices.push(graph.add_node(()));
        }

        // MST is (0, 1) and (2, 0)
        let edges = vec![((vertices[0], vertices[1]), 1usize),
                         ((vertices[1], vertices[2]), 3),
                         ((vertices[2], vertices[0]), 2)];

        for ((from, to), weight) in edges.into_iter() {
            graph.add_edge(from, to, weight);
        }

        /*let mst = Kruskals.minimum_spanning_tree(&graph).unwrap();
        assert!(mst.contains(&(vertices[0], vertices[1])));
        assert!(mst.contains(&(vertices[2], vertices[0])));
        assert!(!mst.contains(&(vertices[1], vertices[2])));*/
    }
}