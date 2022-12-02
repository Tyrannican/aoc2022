/* Bootstrapped */

use std::str::FromStr;

use crate::utils::*;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;

#[derive(Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    pub fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    pub fn outcome(&self, other: &Self) -> i32 {
        let mut score = self.score();

        match self {
            Self::Rock => {
                match other {
                    Self::Rock => score += DRAW_SCORE,
                    Self::Paper => {},
                    Self::Scissors => score += WIN_SCORE
                }
            },
            Self::Paper => {
                match other {
                    Self::Rock => score += WIN_SCORE,
                    Self::Paper => score += DRAW_SCORE,
                    Self::Scissors => {}
                }
            }
            Self::Scissors => {
                match other {
                    Self::Rock => {},
                    Self::Paper => score += WIN_SCORE,
                    Self::Scissors => score += DRAW_SCORE
                }
            }
        }

        score
    }

    pub fn rigged(&self, result: &GameResult) -> i32 {
        match self {
            Self::Rock => {
                match result {
                    GameResult::Win => WIN_SCORE + Self::Paper.score(),
                    GameResult::Draw => DRAW_SCORE + Self::Rock.score(),
                    GameResult::Loss => Self::Scissors.score()
                }
            },
            Self::Paper => {
                match result {
                    GameResult::Win => WIN_SCORE + Self::Scissors.score(),
                    GameResult::Draw => DRAW_SCORE + Self::Paper.score(),
                    GameResult::Loss => Self::Rock.score()
                }
            },
            Self::Scissors => {
                match result {
                    GameResult::Win => WIN_SCORE + Self::Rock.score(),
                    GameResult::Draw => DRAW_SCORE + Self::Scissors.score(),
                    GameResult::Loss => Self::Paper.score()
                }
            }
        }
    }
}

impl std::str::FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(())
        }        
    }
}

enum GameResult {
    Win,
    Draw,
    Loss
}

impl std::str::FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(())
        }
    }
}

struct Game {
    opponent: RPS,
    player: RPS
}

impl Game {
    pub fn resolve(&self) -> i32 {
        self.player.outcome(&self.opponent)
    }
}


pub struct Solution {
    raw_input: String,
    games: Vec<Game>
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self {
            raw_input: String::new(),
            games: vec![]
        };
        sol.process_input("day02/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        self.raw_input = read_file(path);
        self.games = self.raw_input.split("\n")
            .map(|game| {
                let game_split: Vec<&str> = game.split(" ").collect();
                Game {
                    opponent: RPS::from_str(game_split[0]).unwrap(),
                    player: RPS::from_str(game_split[1]).unwrap()
                }
            })
            .collect();
    }

    fn part1(&mut self) {
        let mut total = 0;
        for game in &self.games {
            total += game.resolve();
        }

        println!("Part 1: {}", total);
    }

    fn part2(&mut self) {
        let mut total = 0;
        for game in self.raw_input.split("\n") {
            let game_split: Vec<&str> = game.split(" ").collect();
            let opponent = RPS::from_str(game_split[0]).unwrap();
            let rigged = GameResult::from_str(game_split[1]).unwrap();
            total += opponent.rigged(&rigged);
        }
        
        println!("Part 2: {}", total);
    }
}