/* Bootstrapped */

use crate::utils::*;
use std::collections::HashSet;

fn is_unique(s: &str, target: usize) -> bool {
    let set = s.chars().collect::<HashSet<char>>().len();
    if set != target {
        return false;
    }

    true
}

pub struct Solution {
    pub buffer: String
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            buffer: String::new()
        };
        sol.process_input("day06/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        self.buffer = read_file(path);
    }

    fn part1(&mut self) {
        let mut marker: usize = 4;
        let mut start: usize = 0;
        while marker <= self.buffer.len() {
            if is_unique(&self.buffer[start..marker], 4) {
                break;
            }
            start += 1;
            marker += 1;
        }

        println!("Part 1: {}", marker);
    }

    fn part2(&mut self) {
        let mut marker: usize = 14;
        let mut start: usize = 0;
        while marker <= self.buffer.len() {
            if is_unique(&self.buffer[start..marker], 14) {
                break;
            }

            start += 1;
            marker += 1;
        }

        println!("Part 2: {}", marker);
    }
}