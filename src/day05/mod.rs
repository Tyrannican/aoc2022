/* Bootstrapped */

use crate::utils::*;
use std::collections::HashMap;

pub struct Instruction {
    pub total: i32,
    pub src: i32,
    pub dest: i32
}

pub struct Solution {
    pub stacks: HashMap<i32, Vec<char>>,
    pub instructions: Vec<Instruction>
}

impl Solution {
    pub fn new() -> Self {
        let mut stacks = HashMap::new();
        for i in 1..10 {
            stacks.insert(i, Vec::new());
        }

        let mut sol = Self {
            stacks,
            instructions: Vec::new()
        };

        sol.process_input("day05/input.txt");
        sol
    }

    fn clear(&mut self) {
        let mut stacks = HashMap::new();
        for i in 1..10 {
            stacks.insert(i, Vec::new());
        }

        self.stacks = stacks;
        self.instructions = Vec::new();
    }

    fn build_stacks(&mut self, line: &str) {
        let mut nth = 1;
        let mut stack_number = 1;
        let end_idx = line.len() - 1;
        while nth <= end_idx {
            let ch = line.chars().nth(nth).unwrap();
            let stack = self.stacks.get_mut(&stack_number).unwrap();
            if ch != ' ' && !ch.is_ascii_digit() { stack.push(ch) }
            nth += 4;
            stack_number += 1;
        }
    }

    fn build_instructions(&mut self, ins: &str) {
        let breakdown: Vec<&str> = ins.split(" ").collect();
        let total = breakdown[1].parse::<i32>().unwrap();
        let src = breakdown[3].parse::<i32>().unwrap();
        let dest = breakdown[5].parse::<i32>().unwrap();

        self.instructions.push(Instruction { total, src, dest });
    }

    fn execute(&mut self) {
        for op in self.instructions.iter() {
            for _ in 0..op.total {
                let src_stack = self.stacks.get_mut(&op.src).unwrap();
                let src_char = src_stack.remove(0);
                {
                    let dest_stack = self.stacks.get_mut(&op.dest).unwrap();
                    dest_stack.insert(0, src_char);
                }
            }
        }
    }

    fn execute_bulk(&mut self) {
        for op in self.instructions.iter() {
            let src_stack = self.stacks.get_mut(&op.src).unwrap();
            let crates: Vec<char> = src_stack.drain(0..op.total as usize).collect();
            {
                let dest_stack = self.stacks.get_mut(&op.dest).unwrap();
                dest_stack.splice(0..0, crates);
            }
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        let mut process_instructions = false;

        for line in raw.split("\n") {
            if line == "" {
                process_instructions = true;
                continue;
            }

            if !process_instructions {
                self.build_stacks(line);
            } else {
                self.build_instructions(line);
            }
            
        }
    }

    fn part1(&mut self) {
        self.execute();
        let mut output = String::new();
        for i in 1..10 {
            let stack = self.stacks.get(&i).unwrap();
            output.push(stack[0]);
        }
        println!("Part 1: {}", output);
    }

    fn part2(&mut self) {
        self.clear();
        self.process_input("day05/input.txt");
        self.execute_bulk();

        let mut output = String::new();
        for i in 1..10 {
            let stack = self.stacks.get(&i).unwrap();
            output.push(stack[0]);
        }

        println!("Part 2: {}", output);
    }
}