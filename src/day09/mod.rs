/* Bootstrapped */

use crate::utils::*;
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown
}

type Instruction = (Direction, i32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cell {
    x: i32,
    y: i32
}

pub struct Solution {
    instructions: Vec<Instruction>,
    nodes: RefCell<Vec<Cell>>,
    visited: HashSet<Cell>
}

impl Solution {
    pub fn new() -> Self {
        let initial_state = vec![Cell { x: 0, y: 0 }];

        let mut sol = Self {
            instructions: vec![],
            nodes: RefCell::new(vec![]),
            visited: HashSet::from_iter(initial_state)
        };
        sol.process_input("day09/input.txt");
        sol
    }

    fn init_nodes(&mut self, total: i32) {
        for _ in 0..total {
            self.nodes.borrow_mut().push(Cell { x: 0, y: 0 });
        }
    }

    fn clear(&mut self) {
        self.nodes.borrow_mut().clear();
        self.visited.clear();
    }

    pub fn calculate_position(&self, head: &mut Cell, tail: &mut Cell) -> Option<Cell> {
        let dx = tail.x - head.x;
        let dy = tail.y - head.y;

        if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
            return Some(Cell { x: head.x + dx.clamp(-1, 1), y: head.y + dy.clamp(-1, 1) })
        } else if dx == 2 || dx == -2 {
            return Some(Cell { x: head.x + dx.clamp(-1, 1), y: head.y })
        } else if dy == 2 || dy == -2 {
            return Some(Cell { x: head.x, y: head.y + dy.clamp(-1, 1) })
        }

        None
    }

    pub fn update_nodes(&self, direction: &Direction) -> Cell {
        let mut nodes = self.nodes.borrow_mut();
        let mut iter = nodes.iter_mut();
        let mut head = iter.next().unwrap();

        match direction {
            Direction::Up => head.y += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
            _ => {}
        }

        for tail in iter {
            if let Some(knot) = self.calculate_position(head, tail) {
                tail.x = knot.x;
                tail.y = knot.y;
            }

            head = tail;
        }

        Cell { x: head.x, y: head.y }

    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        self.instructions = raw.split('\n')
            .map(|s| {
                let split = s.split(' ').collect::<Vec<&str>>();
                let dir = match split[0] {
                    "R" => Direction::Right,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => Direction::Unknown
                };

                (dir, split[1].parse::<i32>().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) {
        self.init_nodes(2);
        let mut instructions = self.instructions.clone();

        for item in instructions.drain(..) {
            let (direction, moves) = item;

            for _ in 0..moves {
                let node = self.update_nodes(&direction);
                self.visited.insert(node);
            }
        }
        println!("Part 1: {}", self.visited.len());
    }

    fn part2(&mut self) {
        self.clear();
        self.init_nodes(10);

        let mut instructions = self.instructions.clone();

        for item in instructions.drain(..) {
            let (direction, moves) = item;

            for _ in 0..moves {
                let node = self.update_nodes(&direction);
                self.visited.insert(node);
            }
        }
        println!("Part 2: {:?}", self.visited.len());
    }
}