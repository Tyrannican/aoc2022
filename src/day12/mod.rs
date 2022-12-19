/* Bootstrapped */

use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Node {
    elev: u8,
    neighbours: Vec<Node>
}

impl Node {
    pub fn new(elev: u8) -> Self {
        Self { elev, neighbours: vec![] }
    }
}

pub struct Solution {
    nodes: Vec<Node>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            nodes: vec![]
        };

        sol.process_input("day12/input.txt");
        sol
    }

    // Shameless C+P of a Djikstra
    // fn dijkstra(&self, start: &Node<T>, end: &Node<T>) -> Option<Vec<&Node<T>>> {
    //     // Set up the priority queue and distances map
    //     let mut heap = BinaryHeap::new();
    //     let mut distances = HashMap::new();

    //     // Initialize the distances of all nodes to infinity, except for the start node
    //     for node in &self.nodes {
    //         if node == start {
    //             distances.insert(node, 0);
    //         } else {
    //             distances.insert(node, std::isize::MAX);
    //         }
    //     }

    //     // Insert the start node into the priority queue
    //     heap.push((0, start));

    //     // Set up the predecessors map to store the path
    //     let mut predecessors = HashMap::new();

    //     // Perform the search
    //     while let Some((_, node)) = heap.pop() {
    //         // Check if we have reached the end node
    //         if node == end {
    //             // Construct the path by following the predecessors
    //             let mut path = Vec::new();
    //             let mut current_node = end;
    //             while current_node != start {
    //                 path.push(current_node);
    //                 current_node = predecessors[current_node];
    //             }
    //             path.push(start);
    //             path.reverse();
    //             return Some(path);
    //         }

    //         // Update the distances of the neighbors
    //         for neighbor in &node.neighbors {
    //             let distance = distances[node] + 1; // Assume a distance of 1 between all nodes
    //             if distance < distances[neighbor] {
    //                 distances.insert(neighbor, distance);
    //                 predecessors.insert(neighbor, node);
    //                 heap.push((distance, neighbor));
    //             }
    //         }
    //     }

    //     // Return None if the end node was not reached
    //     None
    // }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        for (x, line) in raw.split('\n').enumerate() {
            for (y, ch) in line.chars().enumerate() {
                let node: Node;
                if ch == 'S' {
                    // Start
                    
                } else if ch == 'E' {
                    // End

                } else {
                    node = Node::new(ch as u8);
                    self.nodes.push(node);
                }
            }
        }
    }

    fn part1(&mut self) {

    }

    fn part2(&mut self) {
        
    }
}