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
    monkeys: Vec<Monkey>,
    inspection: Vec<i64>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            monkeys: vec![],
            inspection: vec![]
        };
        sol.process_input("day11/input.txt");
        sol
    }

    pub fn display(&self) {
        for (idx, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", idx, monkey.items);
        }

        println!("Inspections: {:?}", self.inspection);
    }

    pub fn round(&mut self, monkeys: &mut Vec<Monkey>, calm: bool, modulus: i64) {
        for idx in 0..monkeys.len() {
            // println!("Monkey: {}", idx);
            let monkey = &mut monkeys[idx];
            // println!("Items: {:?}", monkey.items);
            let items = monkey.items
                .drain(..)
                .map(|mut w| {
                    self.inspection[idx] += 1;
                    match monkey.operation {
                        Arithmetic::Add(val) => {
                            if val == -1 { w += w; w }
                            else { w += val; w }

                        }
                        Arithmetic::Mul(val) => {
                            if val == -1 { w *= w; w }
                            else { w *= val; w }
                        }
                    }
                })
                .map(|mut w| {
                    if calm { w /= 3; w }
                    else { w = w % modulus; w }
                })
                .collect::<Vec<i64>>();
            // println!("Items after: {:?} (Old: {:?})", items, monkey.items);
            
            let condition = monkey.condition;
            let true_target = monkey.true_target;
            let false_target = monkey.false_target;

            for item in items {
                let target = if item % condition == 0 {
                    // println!("Divisible by {}, passing to Monkey: {}", condition, true_target);
                    true_target
                } else {
                    // println!("Not divisible by {}, passing to Monkey: {}", condition, false_target);
                    false_target
                };

                monkeys[target as usize].items.push(item);
            }
            println!();
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

            self.monkeys.push(create_monkey(monkey));
            self.inspection.push(0);
        }

    }

    fn part1(&mut self) {
        let mut monkeys = self.monkeys.clone();
        for _ in 0..20 {
            self.round(&mut monkeys, true, 0);
        }
        self.inspection.sort_by(|a, b| b.cmp(a));
        println!("Part 1: {}", self.inspection[0] * self.inspection[1]);
    }

    fn part2(&mut self) {
        // let mut monkeys = self.monkeys.clone();
        // let modulus: i64 = self.monkeys.iter()
        //     .map(|m| m.condition)
        //     .product();

        // for _ in 0..10000 {
        //     self.round(&mut monkeys, false, modulus);
        // }
        // println!("Inspections: {:?}", self.inspection);
    }
}