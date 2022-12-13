/* Bootstrapped */

use crate::utils::*;
use std::iter::Rev;
use std::slice::Iter;

pub struct Solution {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
    visible: u32,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            map: vec![],
            visible: 0,
            height: 0,
            width: 0
        };
        sol.process_input("day08/input.txt");
        sol
    }

    fn is_edge(&self, ridx: usize, cidx: usize) -> bool {
        ridx == 0 || ridx == self.height - 1 || cidx == 0 || cidx == self.width - 1
    }

    fn is_visible(&mut self, ridx: usize, cidx: usize) {
        if self.is_edge(ridx, cidx) { self.visible += 1; return; }

        let target = self.map[ridx][cidx];

        let up = &self.map[0..ridx].iter().map(|m| m[cidx]).filter(|t| t >= &target).collect::<Vec<u32>>();
        if up.len() == 0 { self.visible += 1; return; }

        let down = &self.map[ridx + 1..self.height].iter().map(|m| m[cidx]).filter(|t| t >= &target).collect::<Vec<u32>>();
        if down.len() == 0 { self.visible += 1; return; }

        let left = &self.map[ridx][0..cidx].iter().filter(|t| *t >= &target).map(|t| *t).collect::<Vec<u32>>();
        if left.len() == 0 { self.visible += 1; return; }

        let right = &self.map[ridx][cidx + 1..self.width].iter().filter(|t| *t >= &target).map(|t| *t).collect::<Vec<u32>>();
        if right.len() == 0 { self.visible += 1; return; }
    }

    fn scenic_score(&self, ridx: usize, cidx: usize) -> u32 {
        0
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        for line in raw.split('\n') {
            let inner = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            self.map.push(inner);
        }
        self.height = self.map.len();
        self.width = self.map[0].len();
    }

    fn part1(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.is_visible(row, col);
            }
        }

        println!("Part 1: {}", self.visible);
    }

    fn part2(&mut self) {
        let mut max = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let score = self.scenic_score(row, col);
                if score > max { max = score; }
            }
        }

        println!("Part 2: {}", max);
    }
}