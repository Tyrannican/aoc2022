/* Bootstrapped */

use crate::utils::*;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Noop,
    Addx(i32)
}

pub struct Solution {
    cycle: i32,
    register: i32,
    operations: Vec<Operation>,
    crt: Vec<Vec<char>>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            cycle: 1,
            register: 1,
            operations: vec![],
            crt: Vec::with_capacity(6)
        };
        sol.process_input("day10/input.txt");
        sol
    }
    
    fn reset(&mut self) {
        self.cycle = 1;
        self.register = 1;
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

    fn draw(&mut self) {
        let cidx = self.cycle - 1;
        let crt_row = cidx / 40;
        let crt_idx = cidx % 40;

        if crt_idx >= self.register - 1 && crt_idx <= self.register + 1 {
            self.crt[crt_row as usize].push('#');
        } else {
            self.crt[crt_row as usize].push('.');
        }
    }

    fn display(&self) {
        for row in self.crt.iter() {
            for col in row.iter() {
                print!("{}", col);
            }
            println!();
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

        for _ in 0..6 {
            self.crt.push(Vec::with_capacity(40));
        }
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
        self.reset();
        let ops = self.operations.clone();
        for op in ops {
            match op {
                Operation::Addx(value) => {
                    for _ in 0..2 {
                        self.draw();
                        self.cycle += 1;
                    }
                    self.register += value;
                }
                Operation::Noop => {
                    self.draw();
                    self.cycle += 1;
                }
            }
        }

        println!("Part 2:");
        self.display();
    }
}