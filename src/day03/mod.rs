/* Bootstrapped */

use crate::utils::*;
use std::{collections::HashSet, ops::BitAnd};

fn build_hashset(s: &str) -> HashSet<char> {
    let mut uniq = HashSet::new();
    for ch in s.chars() {
        uniq.insert(ch);
    }
    uniq
}

fn get_priority(item: char) -> isize {
    let priority = if item.is_ascii_lowercase() {
        (item as isize - 97) + 1
    } else {
        (item as isize  - 65 + 26) + 1
    };

    priority
}

pub struct Rucksack {
    pub full_contents: String,
    pub section1: String,
    pub section2: String
}

impl Rucksack {
    pub fn find_common_item(&self) -> char {
        let uniq1 = build_hashset(&self.section1);
        let uniq2 = build_hashset(&self.section2);
        let mut out = uniq1.intersection(&uniq2);
        
        *out.next().unwrap()
    }
}

pub struct Solution {
    pub rucksacks: Vec<Rucksack>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            rucksacks: Vec::new()
        };
        sol.process_input("day03/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        self.rucksacks = raw.split("\n")
            .map(|contents| {
                let (section1, section2) = contents.split_at(contents.len() / 2);
                Rucksack {
                    full_contents: contents.to_string(),
                    section1: section1.to_string(),
                    section2: section2.to_string()
                }
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        for rucksack in self.rucksacks.iter() {
            let common_item = rucksack.find_common_item();
            let priority = get_priority(common_item);
            total += priority;
        }

        println!("Part 1: {}", total);
    }

    fn part2(&mut self) {
        let mut total = 0;
        for sacks in self.rucksacks.chunks(3) {
            let sack_set1 = build_hashset(&sacks[0].full_contents);
            let sack_set2 = build_hashset(&sacks[1].full_contents);
            let sack_set3 = build_hashset(&sacks[2].full_contents);

            // Should _only_ be one item
            let common_item = sack_set1.bitand(&sack_set2)
                .bitand(&sack_set3)
                .drain()
                .collect::<Vec<char>>()
                [0];

            total += get_priority(common_item);
        }

        println!("Part 2: {}", total);
    }
}