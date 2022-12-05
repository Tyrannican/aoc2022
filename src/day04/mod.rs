/* Bootstrapped */

use crate::utils::*;

#[derive(Debug)]
struct Section {
    pub start: i32,
    pub end: i32
}

impl Section {
    pub fn overlap(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
        || (other.start >= self.start && other.end <= self.end)
    }
}

type Pair = (Section, Section);

fn create_sections(section_range: &str) -> Section {
    let sections: Vec<&str> = section_range.split("-").collect();
    let start = sections[0].parse::<i32>().unwrap();
    let end = sections[1].parse::<i32>().unwrap();

    Section { start, end }
}

fn self_contained(sections: &Pair) -> bool {
    let (sec1, sec2) = sections;

    if (
        sec1.start >= sec2.start && sec1.end <= sec2.end
    ) || (
        sec2.start >= sec1.start && sec2.end <= sec1.end
    ) {
        return true
    }

    false
}

fn section_overlap(sections: &Pair) -> bool {
    let (sec1, sec2) = sections;
    sec1.overlap(&sec2) || sec2.overlap(&sec1)
}

pub struct Solution {
    pairs: Vec<Pair>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            pairs: vec![]
        };
        sol.process_input("day04/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let raw = read_file(path);
        self.pairs = raw.split("\n")
            .map(|pair| {
                let pair: Vec<&str> = pair.split(",").collect();
                let section1 = create_sections(&pair[0]);
                let section2 = create_sections(&pair[1]);

                (section1, section2)
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        for pair in self.pairs.iter() {
            if self_contained(pair) {
                total += 1;
            }
        }

        println!("Part 1: {}", total);
    }

    fn part2(&mut self) {
        let mut total = 0;

        for pair in self.pairs.iter() {
            if section_overlap(pair) {
                total += 1;
            }
        }

        println!("Part 2: {}", total);
    }
}