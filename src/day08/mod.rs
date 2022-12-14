/* Bootstrapped */

use crate::utils::*;

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
        if self.is_edge(ridx, cidx) { return 0 }

        let target = self.map[ridx][cidx];

        // This is totally refactorable but I cant be fucked so meh
        let mut left = 0;
        let left_iter = self.map[ridx][0..cidx].iter().rev();
        for item in left_iter {
            left += 1;
            if item >= &target { break; }
        }

        let mut right = 0;
        let right_iter = self.map[ridx][cidx + 1..self.width].iter();
        for item in right_iter {
            right += 1;
            if item >= &target { break; }
        }

        let mut up = 0;
        let up_iter = self.map[0..ridx].iter().rev();
        for item in up_iter {
            up += 1;
            if item[cidx] >= target { break; }
        }

        let mut down = 0;
        let down_iter = self.map[ridx + 1..self.height].iter();
        for item in down_iter {
            down += 1;
            if item[cidx] >= target { break; }
        }

        up * down * left * right
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