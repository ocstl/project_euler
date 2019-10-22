use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::{Graph, Undirected};
use std::fs;

const FILENAME: &str = "inputs/p107_network.txt";
const NO_EDGE: &str = "-";

/// Using network.txt, a 6K text file containing a network with forty vertices, and given in
/// matrix form, find the maximum saving which can be achieved by removing redundant edges whilst
/// ensuring that the network remains connected.
fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();
    let edges = input.lines().enumerate().flat_map(|(from, line)| {
        line.split(',')
            .enumerate()
            .skip(from)
            .filter_map(move |(to, weight)| {
                if weight == NO_EDGE {
                    None
                } else {
                    Some((from, to, weight.parse::<usize>().unwrap()))
                }
            })
    });

    let graph = Graph::<usize, usize, Undirected, usize>::from_edges(edges);
    let initial_weight: usize = graph.edge_references().map(|edge| edge.weight()).sum();

    let min_span =
        Graph::<usize, usize, Undirected, usize>::from_elements(min_spanning_tree(&graph));
    let minimal_weight: usize = min_span.edge_references().map(|edge| edge.weight()).sum();

    let answer = initial_weight - minimal_weight;

    println!("The answer is: {}", answer);
}
