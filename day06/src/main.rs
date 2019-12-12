use std::collections::HashMap;
use std::cell::Cell;

fn get_input(raw_input: &str) -> Vec<(String, String)> {
    raw_input.lines()
        .map(|s| {
            let mut parts_iterator = s.split(')');
            (
                parts_iterator.next().unwrap().to_string(),
                parts_iterator.next().unwrap().to_string()
            )
        })
        .collect()
}

fn get_puzzle_input() -> Vec<(String, String)> {
    let data = include_str!("input.txt");
    get_input(data)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Body {
    name: String,
    orbits: String,
    orbitted_by: Vec<String>,
    depth: Cell<usize>,
}

impl Body {
    fn new(name: String) -> Self {
        Body {
            name: name,
            orbits: "".to_string(),
            orbitted_by: vec![],
            depth: Cell::new(0),
        }
    }
}

type OrbitGraph = HashMap::<String, Body>;

fn build_graph_with_depths(orbits: Vec<(String, String)>) -> OrbitGraph {
    let graph = build_orbit_graph(orbits);
    calc_node_depths(&graph)
}

fn build_orbit_graph(orbits: Vec<(String, String)>) -> OrbitGraph {
    let mut graph = OrbitGraph::with_capacity(orbits.len());

    for (orbitted_body, orbitting_body) in orbits {
        let b = graph.entry(orbitted_body.clone()).or_insert(Body::new(orbitted_body.clone()));
        b.orbitted_by.push(orbitting_body.clone());

        let b2 = graph.entry(orbitting_body.clone()).or_insert(Body::new(orbitting_body.clone()));
        b2.orbits = orbitted_body.clone();
    }

    graph
}

/// Builds a new graph with the node depth of each node updated
/// to be its correct value. This is somewhat functional-programming
/// style, though it is horribly inefficient I found it the easiest
/// way to get it working (recursion on mutable structures is hard).
fn calc_node_depths(graph: &OrbitGraph) -> OrbitGraph {
    let mut new_graph = OrbitGraph::with_capacity(graph.len());
    calc_node_depth(&mut new_graph, graph, 0, "COM");

    new_graph
}

fn calc_node_depth(new_graph: &mut OrbitGraph, graph: &OrbitGraph, current_depth: usize, current_node: &str) {
    let mut current_node = graph.get(current_node).unwrap().clone();
    current_node.depth.set(current_depth);

    for child in &current_node.orbitted_by {
        calc_node_depth(new_graph, graph, current_depth + 1, &child);
    }

    new_graph.insert(current_node.name.clone(), current_node);
}

trait OrbitCount {
    fn num_orbits(&self) -> usize;
}

 impl OrbitCount for OrbitGraph {
    fn num_orbits(&self) -> usize {
        self.values().map(|n| n.depth.get()).sum()
    }
}

/* If you draw out the graph in their example, the total number of orbits
   is just the sum of the depth of each node (COM has depth 0, its children
   have depth 1, their children have depth 2 etc.)

   There is probably a better way of doing this, but it gets the right answer.
*/
fn main() {
    let input = get_puzzle_input();
    let orbit_graph = build_graph_with_depths(input);

    println!("Total number of orbits = {}", orbit_graph.num_orbits());
    assert_eq!(119831, orbit_graph.num_orbits());


}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<(String, String)> {
        get_input("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L")

    }
    #[test]
    pub fn test_example_input() {
        let input = get_example_input();
        let orbit_graph = build_graph_with_depths(input);
        assert_eq!(42, orbit_graph.num_orbits());
    }

    #[test]
    pub fn test_puzzle_input() {
        let input = get_puzzle_input();
        let orbit_graph = build_graph_with_depths(input);
        assert_eq!(119831, orbit_graph.num_orbits());
    }
}