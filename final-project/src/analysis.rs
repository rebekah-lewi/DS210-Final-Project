use std::collections::{HashSet, HashMap, VecDeque};
use std::convert::TryInto;

pub struct Graph {
    pub Nodes: Vec<i32>,
    pub AdjacencyList: HashMap<i32, HashSet<i32>>,
}

impl Graph {
    pub fn new(edges: &[(usize, usize)]) -> Self {
        let mut graph = Graph {
            Nodes: Vec::new(),
            AdjacencyList: HashMap::new(),
        };

        for &(a, b) in edges {
            let a_i32: i32 = a.try_into().expect("Index out of i32 range");
            let b_i32: i32 = b.try_into().expect("Index out of i32 range");

            graph.Nodes.push(a_i32);
            graph.Nodes.push(b_i32);
            graph.AdjacencyList.entry(a_i32).or_insert_with(HashSet::new).insert(b_i32);
            graph.AdjacencyList.entry(b_i32).or_insert_with(HashSet::new).insert(a_i32);
        }

        graph.Nodes.sort();
        graph.Nodes.dedup();

        graph
    }

    pub fn are_connected(&self, node1: i32, node2: i32) -> bool {
        if let Some(neighbors) = self.AdjacencyList.get(&node1) {
            return neighbors.contains(&node2);
        }
        false
    }
}

pub fn graph_analysis(graph: &Graph, start: i32) -> HashMap<i32, i32> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((current_node, current_distance)) = queue.pop_front() {
        if let Some(neighbors) = graph.AdjacencyList.get(&current_node) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    queue.push_back((neighbor, current_distance + 1));
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }
    }
    distances
}
