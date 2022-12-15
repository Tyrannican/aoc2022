/* Bootstrapped */

use crate::utils::*;

#[derive(Debug, PartialEq)]
enum Operation {
    Noop,
    Addx(i32)
}

pub struct Solution {
    cycle: i32,
    register: i32,
    operations: Vec<Operation>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            cycle: 1,
            register: 1,
            operations: vec![]
        };
        sol.process_input("day10/input.txt");
        sol
    }

    fn cycle_check(&self, register_values: &mut Vec<i32>) {
        if self.cycle == 20
            || self.cycle == 60
            || self.cycle == 100
            || self.cycle == 140
            || self.cycle == 180
            || self.cycle == 220
        {
            register_values.push(self.cycle * self.register);
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);

        self.operations = raw.split('\n')
            .map(|line| {
                let split = line.split(' ').collect::<Vec<&str>>();
                if split.len() == 1 { return Operation::Noop }

                Operation::Addx(split[1].parse::<i32>().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut register_values: Vec<i32> = vec![];
        for op in self.operations.iter() {
            match op {
                Operation::Addx(value) => {
                    for _ in 0..2 {
                        self.cycle_check(&mut register_values);
                        self.cycle += 1;
                    }
                    self.register += value;
                }
                Operation::Noop => {
                    self.cycle_check(&mut register_values);
                    self.cycle += 1
                }
            }
        }
        println!("Part 1: {}", register_values.iter().sum::<i32>());
    }

    fn part2(&mut self) {
        
    }
}