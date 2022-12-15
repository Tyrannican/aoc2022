/* Bootstrapped */

use crate::utils::*;
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
    head: Cell,
    tail: Cell,
    visited: HashSet<Cell>
}

impl Solution {
    pub fn new() -> Self {
        let initial_state = vec![Cell { x: 0, y: 0 }];

        let mut sol = Self {
            instructions: vec![],
            head: Cell { x: 0, y: 0 },
            tail: Cell{ x: 0, y: 0 },
            visited: HashSet::from_iter(initial_state)
        };
        sol.process_input("day09/input.txt");
        sol
    }

    fn is_touching(&self) -> bool {
        let mut touching = false;
        for coord in CardinalPoints.iter() {
            let (x, y) = coord;
            if self.tail.x + x == self.head.x && self.tail.y + y == self.head.y {
                touching = true;
                break;
            }
        }

        touching
    }

    fn update_corner(&mut self) {
        // Top right
        if (self.head.x == self.tail.x + 1 && self.head.y == self.tail.y + 2)
        || (self.head.x == self.tail.x + 2 && self.head.y == self.tail.y + 1)
        {
            self.tail.x += 1;
            self.tail.y += 1;
        }

        // Top Left
        if (self.head.x == self.tail.x - 1 && self.head.y == self.tail.y + 2)
        || (self.head.x == self.tail.x - 2 && self.head.y == self.tail.y + 1)
        {
            self.tail.x -= 1;
            self.tail.y += 1;
        }

        // Bottom Left
        if (self.head.x == self.tail.x - 1 && self.head.y == self.tail.y - 2)
        || (self.head.x == self.tail.x - 2 && self.head.y == self.tail.y - 1)
        {
            self.tail.x -= 1;
            self.tail.y -= 1;
        }

        // Bottom Right
        if (self.head.x == self.tail.x + 1 && self.head.y == self.tail.y - 2)
        || (self.head.x == self.tail.x + 2 && self.head.y == self.tail.y - 1)
        {
            self.tail.x += 1;
            self.tail.y -= 1;
        }
    }

    fn update_cadinal(&mut self) {
        if self.head.x == self.tail.x + 2 {
            self.tail.x += 1;
        }
        else if self.head.x == self.tail.x - 2 {
            self.tail.x -= 1;
        }
        else if self.head.y == self.tail.y + 2 {
            self.tail.y += 1;
        }
        else if self.head.y == self.tail.y - 2 {
            self.tail.y -= 1;
        }
    }

    pub fn update_tail(&mut self) {
        if !self.is_touching() {
            self.update_corner();
            self.update_cadinal();
        }

        self.visited.insert(Cell { x: self.tail.x, y: self.tail.y });
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
        let mut instructions = self.instructions.clone();

        for item in instructions.drain(..) {
            let (direction, moves) = item;

            for _ in 0..moves {
                match direction {
                    Direction::Up => self.head.y += 1,
                    Direction::Down => self.head.y -= 1,
                    Direction::Left => self.head.x -= 1,
                    Direction::Right => self.head.x += 1,
                    _ => {}
                }
                self.update_tail();
            }
        }
        println!("Part 1: {}", self.visited.len());
    }

    fn part2(&mut self) {
        
    }
}