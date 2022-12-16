/* Bootstrapped */

use crate::utils::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Arithmetic {
    Add(i32),
    Mul(i32),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<i32>,
    operation: Arithmetic,
    condition: i32,
    true_target: u32,
    false_target: u32
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            items: vec![],
            operation: Arithmetic::Add(1),
            condition: 0,
            true_target: 0,
            false_target: 0
        }
    }
}

fn create_monkey(mut monkey_data: Vec<&str>) -> Monkey {
    monkey_data.remove(0);
    let instructions = monkey_data.iter()
        .map(|i| i.split(':').map(|s| s.trim()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut monkey = Monkey::new();
    
    for ins in instructions.iter() {
        match ins[0] {
            "Starting items" => {
                monkey.items = ins[1].split(", ")
                    .map(|item| item.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            }
            "Operation" => {
                let add = ins[1].split("+ ").collect::<Vec<&str>>();
                if add.len() > 1 {
                    let val = match add[1] {
                        "old" => -1,
                        _ => add[1].parse::<i32>().unwrap()
                    };
                    monkey.operation = Arithmetic::Add(val);
                } else {
                    let mul = ins[1].split("* ").collect::<Vec<&str>>();
                    let val = match mul[1] {
                        "old" => -1,
                        _ => mul[1].parse::<i32>().unwrap()
                    };
                    monkey.operation = Arithmetic::Mul(val);
                }
            }
            "Test" => {
                let digit = ins[1].split(' ').collect::<Vec<&str>>()[2];
                monkey.condition = digit.parse::<i32>().unwrap();
            }
            "If true" => {
                let target = ins[1].split(' ').collect::<Vec<&str>>()[3];
                monkey.true_target = target.parse::<u32>().unwrap();
            },
            "If false" => {
                let target = ins[1].split(' ').collect::<Vec<&str>>()[3];
                monkey.false_target = target.parse::<u32>().unwrap();
            }
            _ => {}
        }
    }

    monkey
}

pub struct Solution {
    monkeys: HashMap<usize, Monkey>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            monkeys: HashMap::new()
        };
        sol.process_input("day11/input.txt");
        sol
    }

    pub fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            let mut monkey = self.monkeys.get_mut(&idx).unwrap();
            for mut worry in monkey.items.drain(..) {
                match monkey.operation {
                    Arithmetic::Add(val) => {
                        if val == -1 {
                            worry += worry;
                        } else {
                            worry += val;
                        }
                    }
                    Arithmetic::Mul(val) => {
                        if val == -1 {
                            worry *= worry;
                        } else {
                            worry *= val
                        }
                    }
                }

                worry /= 3;

                // Toss to monkey
                if worry % monkey.condition == 0 {

                } else {

                }
            }
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        for (idx, data) in raw.split("\n\n").enumerate() {
            let monkey = data.split("\n")
                .map(|c| c.trim())
                .collect::<Vec<&str>>();

            self.monkeys.insert(idx, create_monkey(monkey));
        }
    }

    fn part1(&mut self) {
        println!("Monkeys: {:?}", self.monkeys);
    }

    fn part2(&mut self) {
        
    }
}