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

    pub fn update_tail(&mut self) {
        // TODO: Move the tail in accordance with the tail
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

            println!("Instruction: ({:?}, {}) -- Head: {:?}", direction, moves, self.head);
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
        println!("Length: {:?}", self.visited.len());
    }

    fn part2(&mut self) {
        
    }
}