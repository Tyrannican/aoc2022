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
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        for line in raw.split('\n') {
            let inner = line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            self.map.push(inner);
        }

        self.height = self.map.len();
        self.width = self.map[0].len();
    }

    fn part1(&mut self) {

    }

    fn part2(&mut self) {
        
    }
}