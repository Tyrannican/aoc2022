/* Bootstrapped */

use crate::utils::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Arithmetic {
    Add(i64),
    Mul(i64),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<i64>,
    operation: Arithmetic,
    condition: i64,
    true_target: u64,
    false_target: u64
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
                    .map(|item| item.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
            }
            "Operation" => {
                let add = ins[1].split("+ ").collect::<Vec<&str>>();
                if add.len() > 1 {
                    let val = match add[1] {
                        "old" => -1,
                        _ => add[1].parse::<i64>().unwrap()
                    };
                    monkey.operation = Arithmetic::Add(val);
                } else {
                    let mul = ins[1].split("* ").collect::<Vec<&str>>();
                    let val = match mul[1] {
                        "old" => -1,
                        _ => mul[1].parse::<i64>().unwrap()
                    };
                    monkey.operation = Arithmetic::Mul(val);
                }
            }
            "Test" => {
                let digit = ins[1].split(' ').collect::<Vec<&str>>()[2];
                monkey.condition = digit.parse::<i64>().unwrap();
            }
            "If true" => {
                let target = ins[1].split(' ').collect::<Vec<&str>>()[3];
                monkey.true_target = target.parse::<u64>().unwrap();
            },
            "If false" => {
                let target = ins[1].split(' ').collect::<Vec<&str>>()[3];
                monkey.false_target = target.parse::<u64>().unwrap();
            }
            _ => {}
        }
    }

    monkey
}

pub struct Solution {
    monkeys: HashMap<usize, Monkey>,
    inspection: Vec<i64>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            monkeys: HashMap::new(),
            inspection: vec![]
        };
        sol.process_input("day11/input.txt");
        sol
    }

    pub fn display(&self) {
        for (idx, monkey) in self.monkeys.iter() {
            println!("Monkey {}: {:?}", idx, monkey.items);
        }

        println!("Inspections: {:?}", self.inspection);
    }

    pub fn round(&mut self, calm: bool, modulus: i64) {
        let length = self.monkeys.len();

        for idx in 0..length {
            // println!("Monkey: {}", idx);
            let items = self.monkeys
                .get_mut(&idx)
                .unwrap()
                .items.drain(..)
                .collect::<Vec<i64>>();
            
            let operation = self.monkeys.get(&idx).unwrap().operation.clone();
            let condition = self.monkeys.get(&idx).unwrap().condition;
            let true_target = self.monkeys.get(&idx).unwrap().true_target;
            let false_target = self.monkeys.get(&idx).unwrap().false_target;

            for mut worry in items {
                self.inspection[idx] += 1;
                // println!("Item: {}", worry);
                match operation {
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
                // println!("Worry after operation: {}", worry);

                if calm {
                    worry /= 3;
                } else {
                    worry = worry % modulus;
                }

                // println!("Worry after calm down: {}", worry);

                // Toss to monkey
                if worry % condition == 0 {
                    // println!("Divisible by {}: sending to monkey {}", condition, true_target);
                    let target = self.monkeys.get_mut(&(true_target as usize)).unwrap();
                    target.items.push(worry);
                } else {
                    // println!("Not divisible by {}: sending to monkey {}", condition, false_target);
                    let target = self.monkeys.get_mut(&(false_target as usize)).unwrap();
                    target.items.push(worry);
                }
            }
            // println!();
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

        for _ in 0..self.monkeys.len() { self.inspection.push(0); }
    }

    fn part1(&mut self) {
        for _ in 0..20 {
            self.round(true, 0);
        }
        self.inspection.sort_by(|a, b| b.cmp(a));
        println!("Part 1: {}", self.inspection[0] * self.inspection[1]);
    }

    fn part2(&mut self) {
        let modulus: i64 = self.monkeys.values()
            .map(|m| m.condition)
            .product();

        for _ in 0..10000 {
            self.round(false, modulus);
        }
        println!("Inspections: {:?}", self.inspection);
    }
}