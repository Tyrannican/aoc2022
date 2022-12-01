use crate::utils::*;

pub struct Solution {
    calories: Vec<Vec<i32>>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            calories: Vec::new()
        };

        sol.process_input("day01/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        let mut inner: Vec<i32> = Vec::new();
        for calorie in raw.split("\n") {
            if calorie == "" {
                self.calories.push(inner.drain(..).collect());
            } else {
                if let Ok(val) = calorie.parse::<i32>() {
                    inner.push(val)
                }
            }
        }
    }

    fn part1(&mut self) {
        let totals = self.calories.iter()
            .map(|c| c.iter().sum::<i32>())
            .collect::<Vec<i32>>();

        let max = totals.iter().max().unwrap();
        println!("Part 1: {}", max);
    }

    fn part2(&mut self) {
        let mut totals = self.calories.iter()
            .map(|c| c.iter().sum::<i32>())
            .collect::<Vec<i32>>();

        totals.sort_by(|a, b| b.cmp(a));

        println!("Part 2: {}", totals[0] + totals[1] + totals[2]);
    }
}