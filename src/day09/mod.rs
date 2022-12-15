/* Bootstrapped */

use crate::utils::*;
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown
}

const CardinalPoints: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1)
];

type Instruction = (Direction, i32);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cell {
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
    }

    fn is_touching(&self, head: &mut Cell, tail: &mut Cell) -> bool {
        let mut touching = false;
        for coord in CardinalPoints.iter() {
            let (x, y) = coord;
            if tail.x + x == head.x && tail.y + y == head.y {
                touching = true;
                break;
            }
        }

        touching
    }

    fn update_corner(&self, head: &mut Cell, tail: &mut Cell) {
        // Top right
        if (head.x == tail.x + 1 && head.y == tail.y + 2)
        || (head.x == tail.x + 2 && head.y == tail.y + 1)
        {
            tail.x += 1;
            tail.y += 1;
        }

        // Top Left
        if (head.x == tail.x - 1 && head.y == tail.y + 2)
        || (head.x == tail.x - 2 && head.y == tail.y + 1)
        {
            tail.x -= 1;
            tail.y += 1;
        }

        // Bottom Left
        if (head.x == tail.x - 1 && head.y == tail.y - 2)
        || (head.x == tail.x - 2 && head.y == tail.y - 1)
        {
            tail.x -= 1;
            tail.y -= 1;
        }

        // Bottom Right
        if (head.x == tail.x + 1 && head.y == tail.y - 2)
        || (head.x == tail.x + 2 && head.y == tail.y - 1)
        {
            tail.x += 1;
            tail.y -= 1;
        }
    }

    fn update_cardinal(&self, head: &mut Cell, tail: &mut Cell) {
        if head.x == tail.x + 2 {
            tail.x += 1;
        }
        else if head.x == tail.x - 2 {
            tail.x -= 1;
        }
        else if head.y == tail.y + 2 {
            tail.y += 1;
        }
        else if head.y == tail.y - 2 {
            tail.y -= 1;
        }
    }

    pub fn update_nodes(&self) -> Cell {
        println!("Iteration");
        let mut nodes = self.nodes.borrow_mut();
        let mut iter = nodes.iter_mut();

        let mut head = iter.next().unwrap();
        for tail in iter {
            if !self.is_touching(head, tail) {
                self.update_corner(head, tail);
                self.update_cardinal(head, tail);
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

            let mut head = &mut self.nodes.borrow_mut()[0];
            for _ in 0..moves {
                match direction {
                    Direction::Up => head.y += 1,
                    Direction::Down => head.y -= 1,
                    Direction::Left => head.x -= 1,
                    Direction::Right => head.x += 1,
                    _ => {}
                }
                let node = self.update_nodes();
                self.visited.insert(node);
            }
        }
        println!("Part 1: {:?}", self.nodes.borrow()[0]);
    }

    fn part2(&mut self) {
        self.clear();
        self.init_nodes(10);
    }
}