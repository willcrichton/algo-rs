//! Implements [max flow algorithms](http://en.wikipedia.org/wiki/Maximum_flow_problem) on graphs.

use rustc::util::nodemap::FnvHasher;
use std::collections::HashMap;
use std::num::Int;
use std::hash::Hash;
use std::default::Default;

use graph::Graph;
use FnvMap;

/// Given a graph G with capacities on the edges, source S, and sink T, return the edges in G
/// on the max flow between S and T.
///
/// Assumes that S is connected to T.
pub trait MaxFlow<G: Graph> where G::EdgeValue: Int {
    fn max_flow(&self, graph: &G, source: G::NodeIndex, sink: G::NodeIndex)
                -> Vec<((G::NodeIndex, G::NodeIndex), G::EdgeValue)>;
}

#[derive(Copy)]
pub struct FordFulkerson;

impl<G: Graph> MaxFlow<G> for FordFulkerson
    where G::EdgeValue: Int, G::NodeIndex: Hash<FnvHasher> + Eq
{
    fn max_flow(&self, graph: &G, source: G::NodeIndex, sink: G::NodeIndex)
                -> Vec<((G::NodeIndex, G::NodeIndex), G::EdgeValue)>
    {
        let mut flows: FnvMap<(G::NodeIndex, G::NodeIndex), usize> =
            HashMap::with_hash_state(Default::default());

        for &(from, to, _) in graph.edges().iter() {
            flows.insert((from, to), 0us);
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
    use graph::{AdjacencyList, Graph};

    #[test]
    fn simple() {
        let mut graph = AdjacencyList::new();
        let mut vertices = Vec::new();

        for _ in range(0, 4us) {
            vertices.push(graph.add_node(()));
        }

        let edges = vec![
            ((vertices[0], vertices[1]), 10us),
            ((vertices[0], vertices[2]), 10us),
            ((vertices[1], vertices[2]), 1us),
            ((vertices[1], vertices[3]), 10us),
            ((vertices[2], vertices[3]), 10us),
            ];

        for ((from, to), weight) in edges.into_iter() {
            graph.add_edge(from, to, weight);
        }

        //let min_flow = FordFulkerson.max_flow(&graph, &vertices[0], &vertices[3]);
        //println!("{}", min_flow);
    }
}