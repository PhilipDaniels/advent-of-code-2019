use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Body {
    name: String,
    orbits: String,
    orbitted_by: Vec<String>,
    depth: usize,
}

impl Body {
    fn new(name: String) -> Self {
        Body {
            name: name,
            orbits: "".to_string(),
            orbitted_by: vec![],
            depth: 0,
        }
    }
}

type OrbitGraph = HashMap::<String, Body>;

fn build_orbit_graph(orbits: Vec<(String, String)>) -> OrbitGraph {
    let mut graph = OrbitGraph::new();

    for (orbitted_body, orbitting_body) in orbits {
        let b = graph.entry(orbitted_body.clone()).or_insert(Body::new(orbitted_body.clone()));
        b.orbitted_by.push(orbitting_body.clone());

        let b2 = graph.entry(orbitting_body.clone()).or_insert(Body::new(orbitting_body.clone()));
        b2.orbits = orbitted_body.clone();
    }

    graph
}

fn calc_node_depths(graph: &OrbitGraph) -> Vec<Body> {
    let mut all_nodes = Vec::with_capacity(graph.len());
    calc_node_depth(&mut all_nodes, graph, 0, "COM");
    all_nodes
}

fn calc_node_depth(done_nodes: &mut Vec<Body>, graph: &OrbitGraph, current_depth: usize, current_node: &str) {
    let mut current_node = graph.get(current_node).unwrap().clone();
    current_node.depth = current_depth;

    for child in &current_node.orbitted_by {
        calc_node_depth(done_nodes, graph, current_depth + 1, &child);
    }

    done_nodes.push(current_node);


    // match graph.get(current_node) {
    //     None => None,
    //     Some(node) => {
    //         let children = &node.orbitted_by;

    //         let child_orbits: i32 = children.iter()
    //             .map(|child| num_orbits_of_body(graph, current_depth + 1, child))
    //             .sum();

    //         ((current_depth + 1) * children.len() as i32) + child_orbits
    //     }
    // }
}


fn calculate_number_of_orbits(graph: &OrbitGraph) -> usize {
    num_orbits_of_body(graph, 0, "COM") as usize
}

/* If you draw out the graph in their example, the total number of orbits
   is just the sum of the depth of each node (COM has depth 0, its children
   have depth 1, their children have depth 2 etc.)

   There is probably a better way of doing this, but it gets the right answer.
*/
fn num_orbits_of_body(graph: &OrbitGraph, current_depth: i32, current_node: &str) -> i32 {
    match graph.get(current_node) {
        None => 0,
        Some(node) => {
            let children = &node.orbitted_by;

            let child_orbits: i32 = children.iter()
                .map(|child| num_orbits_of_body(graph, current_depth + 1, child))
                .sum();

            ((current_depth + 1) * children.len() as i32) + child_orbits
        }
    }
}


fn main() {
    let input = get_puzzle_input();
    let orbit_graph = build_orbit_graph(input);
    let num_orbits = calculate_number_of_orbits(&orbit_graph);
    // 119831 is the right answer.
    println!("Total number of orbits = {}", num_orbits);
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
        let graph = build_orbit_graph(input);
        let num_orbits = calculate_number_of_orbits(&graph);
        assert_eq!(42, num_orbits);
    }

    #[test]
    pub fn test_puzzle_input() {
        let input = get_puzzle_input();
        let graph = build_orbit_graph(input);
        let num_orbits = calculate_number_of_orbits(&graph);
        assert_eq!(num_orbits, 119831);
    }

    #[test]
    pub fn calc_node_depths_test() {
        let input = get_example_input();
        let graph = build_orbit_graph(input);
        let mut all_nodes_with_depths = calc_node_depths(&graph);
        all_nodes_with_depths.sort();
        //println!("all_nodes_with_depths = \n{:#?}", all_nodes_with_depths);
        let sum_orbits: usize = all_nodes_with_depths.iter().map(|body| body.depth).sum();
        assert_eq!(sum_orbits, 42);
    }
}