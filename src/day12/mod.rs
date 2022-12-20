/* Bootstrapped */

use crate::utils::*;
use std::rc::Rc;
use std::cell::RefCell;

const CARDINAL: [(i32, i32); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1)
];

#[derive(Debug, Clone)]
pub struct Node {
    elev: u8,
    neighbors: Vec<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(elev: u8) -> Self {
        Self { elev, neighbors: vec![] }
    }
}

pub struct Solution {
    start: Option<Rc<RefCell<Node>>>,
    target: Option<Rc<RefCell<Node>>>,
    nodes: Vec<Vec<Rc<RefCell<Node>>>>,
    width: i32,
    height: i32
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            start: None,
            target: None,
            nodes: vec![],
            height: 0,
            width: 0
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

    fn add_node(&mut self, node: Node, inner: &mut Vec<Rc<RefCell<Node>>>, sp: char) {
        let rc = Rc::new(RefCell::new(node));
        match sp {
            'S' => self.start = Some(Rc::clone(&rc)),
            'E' => self.target = Some(Rc::clone(&rc)),
            _ => {}
        }

        inner.push(Rc::clone(&rc));
    }

    fn update_neighbors(&mut self) {
        // TODO: Fix this
        for ridx in 0..self.height {
            for cidx in 0..self.width {
                let mut node = self.nodes[ridx as usize][cidx as usize].borrow_mut();
                for (x, y) in CARDINAL {
                    let dx = cidx + x;
                    let dy = ridx + y;
                    if dx < 0 || dx >= self.width || dy < 0 || dy >= self.height { continue; }
                    let check = &self.nodes[dy as usize][dx as usize];
                    let check_borrow = check.borrow();
                    if check_borrow.elev == node.elev + 1 || check_borrow.elev <= node.elev {
                        node.neighbors.push(Rc::clone(check));
                    }
                }
            }
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        for line in raw.split('\n') {
            let mut inner = vec![];
            for ch in line.chars() {
                if ch == 'S' || ch == 'E' {
                    let node = Node::new(0);
                    self.add_node(node, &mut inner, ch);
                } else {
                    let node = Node::new(ch as u8);
                    self.add_node(node, &mut inner, '_');
                }
            }
            self.nodes.push(inner);
        }
        self.height = self.nodes.len() as i32;
        self.width = self.nodes[0].len() as i32;
        self.update_neighbors();
    }

    fn part1(&mut self) {
        println!("Nodes: {:?}", self.nodes[0]);
    }

    fn part2(&mut self) {
        
    }
}