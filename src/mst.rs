use std::collections::{HashSet, BinaryHeap};
use rustc::util::nodemap::FnvHasher;

use graph::{Graph, NodeIndex, Edge, HeapEdge};

pub trait MinimumSpanningTree<N, E: Ord> {
    fn minimum_spanning_tree(&self, graph: &Graph<N, E>) -> Vec<Edge>;
}

pub struct Kruskals;

impl<N, E: Ord> MinimumSpanningTree<N, E> for Kruskals {
    fn minimum_spanning_tree(&self, graph: &Graph<N, E>) -> Vec<Edge> {
        let mut vertices = HashSet::with_hasher(FnvHasher);
        let mut edges = Vec::new();
        let all_nodes: Vec<&NodeIndex> = graph.nodes.keys().collect();

        let mut edge_heap = BinaryHeap::new();
        for (to, neighbors) in graph.edges.iter() {
            for (from, weight) in neighbors.iter() {
                edge_heap.push(HeapEdge((*to, *from), weight));
            }
        }

        while vertices.len() < graph.nodes.len() {
            let HeapEdge((from, to), _) = match edge_heap.pop() {
                Some(edge) => edge,
                None => panic!("couldn't find MST before edges ran out"),
            };

            if !vertices.contains(&from) || !vertices.contains(&to) {
                vertices.insert(from);
                vertices.insert(to);
                edges.push((from, to));
            }
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::Graph;

    #[test]
    fn simple() {
        let mut graph = Graph::new();
        let mut vertices = Vec::new();

        for _ in range(0, 3u) {
            vertices.push(graph.add_node(()));
        }

        // MST is (0, 1) and (2, 0)
        let edges = vec![((vertices[0], vertices[1]), 1u),
                         ((vertices[1], vertices[2]), 3u),
                         ((vertices[2], vertices[0]), 2u)];

        for (edge, weight) in edges.into_iter() {
            graph.add_edge(edge, weight);
        }

        let mst = Kruskals.minimum_spanning_tree(&graph);
        assert!(mst.contains(&(vertices[0], vertices[1])));
        assert!(mst.contains(&(vertices[2], vertices[0])));
        assert!(!mst.contains(&(vertices[1], vertices[2])));
    }
}