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

type Peak = Rc<RefCell<Node>>;
type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    elev: u8,
    x: i32,
    y: i32,
    neighbors: Vec<Peak>
}

impl Node {
    pub fn new(elev: u8, x: i32, y: i32) -> Self {
        Self {
            elev,
            x, y,
            neighbors: vec![]
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut neighbors = vec![];
        for n in &self.neighbors {
            neighbors.push(char::from_u32(n.borrow().elev as u32).unwrap());
        }

        let s = format!("Node {{ X: {}, Y: {}, elev: {}, neighbors: {:?} }}", self.x, self.y, self.elev, neighbors);
        write!(f, "{}", s)
    }
}

pub struct Solution {
    start: Option<Peak>,
    target: Option<Peak>,
    nodes: Vec<Vec<Peak>>,
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

    fn add_node(&mut self, node: Node, inner: &mut Vec<Peak>, sp: char) {
        let rc = Rc::new(RefCell::new(node));
        match sp {
            'S' => self.start = Some(Rc::clone(&rc)),
            'E' => self.target = Some(Rc::clone(&rc)),
            _ => {}
        }

        inner.push(Rc::clone(&rc));
    }

    fn update_neighbors(&mut self) {
        for ridx in 0..self.height {
            for cidx in 0..self.width {
                let mut node = self.nodes[ridx as usize][cidx as usize].borrow_mut();
                for (x, y) in CARDINAL {
                    let dx = ridx + x;
                    let dy = cidx + y;
                    if dx < 0 || dx >= self.height || dy < 0 || dy >= self.width { continue; }
                    let check = &self.nodes[dx as usize][dy as usize];
                    let check_borrow = check.borrow();
                    if check_borrow.elev == node.elev + 1 || check_borrow.elev <= node.elev {
                        node.neighbors.push(Rc::clone(check));
                    }
                }
            }
        }
    }

    fn display(&self) {
        for ridx in 0..self.height {
            for cidx in 0..self.width {
                let node = self.nodes[ridx as usize][cidx as usize].borrow();
                println!("{}", node);
            }
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        for (x, line) in raw.split('\n').enumerate() {
            let mut inner = vec![];
            for (y, ch) in line.chars().enumerate() {
                if ch == 'S' || ch == 'E' {
                    let node = if ch == 'S' {
                        Node::new('a' as u8, x as i32 ,y as i32)
                    } else {
                        Node::new('z' as u8, x as i32, y as i32)
                    };
                    self.add_node(node, &mut inner, ch);
                } else {
                    let node = Node::new(ch as u8, x as i32, y as i32);
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
        println!("{}", self.target.as_ref().unwrap().borrow());
    }

    fn part2(&mut self) {
        
    }
}