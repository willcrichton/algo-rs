use std::collections::HashMap;
use std::num::Int;
use rustc::util::nodemap::FnvHasher;

use graph::{Graph, Edge, NodeIndex};

pub trait MaxFlow<N, E: Int> {
    fn max_flow(&self, graph: &Graph<N, E>, source: &NodeIndex, sink: &NodeIndex) -> Vec<(Edge, E)>;
}

pub struct FordFulkerson;

impl<N, E: Int> MaxFlow<N, E> for FordFulkerson {
    fn max_flow(&self, graph: &Graph<N, E>, source: &NodeIndex, sink: &NodeIndex) -> Vec<(Edge, E)>
    {
        let mut flows = HashMap::with_hasher(FnvHasher);
        for (from, neighbors) in graph.edges.iter() {
            for to in neighbors.keys() {
                flows.insert((from, to), 0u);
            }
        }

        let mut path = graph.find_path(source, sink);
        while path.is_some() {
            /*let flow = path
                .iter()
                .map(|e| graph.weights.get(e).unwrap() - flows.get(&e).unwrap())
                .min_by(|x| x)
                .unwrap();

            for edge in path.iter() {
                flows.insert(edge, flows.get(edge).unwrap() - flow);
            }*/

            path = graph.find_path(source, sink);
        }

        return Vec::new();
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

        for _ in range(0, 4u) {
            vertices.push(graph.add_node(()));
        }

        let edges = vec![
            ((vertices[0], vertices[1]), 10u),
            ((vertices[0], vertices[2]), 10u),
            ((vertices[1], vertices[2]), 1u),
            ((vertices[1], vertices[3]), 10u),
            ((vertices[2], vertices[3]), 10u),
            ];

        for (edge, weight) in edges.into_iter() {
            graph.add_edge(edge, weight);
        }

        //let min_flow = FordFulkerson.max_flow(&graph, &vertices[0], &vertices[3]);
        //println!("{}", min_flow);
    }
}