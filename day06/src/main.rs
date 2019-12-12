use std::collections::HashMap;
use std::cell::Cell;
use std::collections::hash_map::Entry;

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

struct OrbitGraph(HashMap::<String, Body>);

impl OrbitGraph {
    /// Creates a new OrbitGraph from the given orbit pairs.
    fn new(orbits: Vec<(String, String)>) -> Self {
        let mut graph = Self::with_capacity(orbits.len());

        for (orbitted_body, orbitting_body) in orbits {
            let b = graph.entry(orbitted_body.clone()).or_insert(Body::new(orbitted_body.clone()));
            b.orbitted_by.push(orbitting_body.clone());

            let b2 = graph.entry(orbitting_body.clone()).or_insert(Body::new(orbitting_body.clone()));
            b2.orbits = orbitted_body.clone();
        }

        // Now calculate the node depths.
        graph.calc_node_depth(0, "COM");
        graph
    }

    /// Constructs a new OrbitGraph with specified capacity.
    fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Hmm, entry requires a moved, not borrowed, key.
    fn entry(&mut self, key: String) -> Entry<String, Body> {
        self.0.entry(key)
    }

    fn calc_node_depth(&self, current_depth: usize, current_node: &str) {
        let mut current_node = self.0.get(current_node).unwrap();
        current_node.depth.set(current_depth);

        for child in &current_node.orbitted_by {
            self.calc_node_depth(current_depth + 1, &child);
        }
    }

    fn total_num_orbits(&self) -> usize {
        self.0.values().map(|n| n.depth.get()).sum()
    }
}

/* If you draw out the graph in their example, the total number of orbits
   is just the sum of the depth of each node (COM has depth 0, its children
   have depth 1, their children have depth 2 etc.)

   There is probably a better way of doing this, but it gets the right answer.
*/
fn main() {
    let input = get_puzzle_input();
    let orbit_graph = OrbitGraph::new(input);

    println!("Total number of orbits = {}", orbit_graph.total_num_orbits());
    assert_eq!(119831, orbit_graph.total_num_orbits());
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
        let orbit_graph = OrbitGraph::new(input);
        assert_eq!(42, orbit_graph.total_num_orbits());
    }

    #[test]
    pub fn test_puzzle_input() {
        let input = get_puzzle_input();
        let orbit_graph = OrbitGraph::new(input);
        assert_eq!(119831, orbit_graph.total_num_orbits());
    }
}
