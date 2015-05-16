//! Implements [max flow algorithms](http://en.wikipedia.org/wiki/Maximum_flow_problem) on graphs.

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ord;

use graph::Graph;

/// Given a graph G with capacities on the edges, source S, and sink T, return the edges in G
/// on the max flow between S and T.
///
/// Assumes that S is connected to T.
pub trait MaxFlow<G: Graph> where G::EdgeValue: Ord {
    fn max_flow(&self, graph: &G, source: G::NodeIndex, sink: G::NodeIndex)
                -> Vec<((G::NodeIndex, G::NodeIndex), G::EdgeValue)>;
}

pub struct FordFulkerson;

impl<G: Graph> MaxFlow<G> for FordFulkerson
    where G::EdgeValue: Ord, G::NodeIndex: Hash + Eq
{
    fn max_flow(&self, graph: &G, source: G::NodeIndex, sink: G::NodeIndex)
                -> Vec<((G::NodeIndex, G::NodeIndex), G::EdgeValue)>
    {
        let mut flows: HashMap<(G::NodeIndex, G::NodeIndex), usize> = HashMap::new();

        for &(from, to, _) in graph.edges().iter() {
            flows.insert((from, to), 0usize);
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
    #![allow(unused_imports)]
    use super::*;
    use graph::{AdjacencyList, Graph};

    #[test]
    fn simple() {
        let mut graph = AdjacencyList::new();
        let mut vertices = Vec::new();

        for _ in 0..4 {
            vertices.push(graph.add_node(()));
        }

        let edges = vec![
            ((vertices[0], vertices[1]), 10usize),
            ((vertices[0], vertices[2]), 10),
            ((vertices[1], vertices[2]), 1),
            ((vertices[1], vertices[3]), 10),
            ((vertices[2], vertices[3]), 10),
            ];

        for ((from, to), weight) in edges.into_iter() {
            graph.add_edge(from, to, weight);
        }

        //let min_flow = FordFulkerson.max_flow(&graph, &vertices[0], &vertices[3]);
        //println!("{}", min_flow);
    }
}