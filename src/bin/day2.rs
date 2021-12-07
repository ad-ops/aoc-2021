use std::str::FromStr;
use thiserror::Error;
use aoc_2021::{puzzle_main, puzzle_tests, Puzzle};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("unknown instruction")]
    UnknownInstruction,
    #[error("argument contains illegal value")]
    InvalidArgument,
    #[error("argument misses whitespace")]
    MissingWhitespace,
}

#[derive(Debug)]
struct Position {
    horizontal: u32,
    depth: u32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    // used for part1
    fn set_course(&mut self, command: Command) {
        match command {
            Command::Forward(d) => self.horizontal = self.horizontal + d,
            Command::Down(d) => self.depth = self.depth + d,
            Command::Up(d) => self.depth = if self.depth < d { 0 } else { self.depth - d},
        }
    }

    // used for part2
    fn set_aim(&mut self, command: Command) {
        match command {
            Command::Forward(d) => {
                self.horizontal = self.horizontal + d;
                let changed_depth = self.depth as i32 + self.aim * d as i32;
                self.depth = if changed_depth < 0 { 0 } else { changed_depth as u32 };
            },
            Command::Down(d) => self.aim = self.aim + d as i32,
            Command::Up(d) => self.aim = self.aim - d as i32,
        }
    }
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, distance) = s.split_once(' ').ok_or(Error::MissingWhitespace)?;
        let distance = distance.parse::<u32>().map_err(|_| Error::UnknownInstruction)?;

        match command {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            _ => Err(Error::UnknownInstruction),
        }
    }
}

fn solver_part1(input: Vec<String>) -> String {
    let mut position: Position = Position::new();
    input
        .into_iter()
        .filter_map(|l| l.parse::<Command>().ok())
        .for_each(|command| position.set_course(command));

    let solution = position.horizontal * position.depth;
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let mut position: Position = Position::new();
    input
        .into_iter()
        .filter_map(|l| l.parse::<Command>().ok())
        .for_each(|command| position.set_aim(command));

    let solution = position.horizontal * position.depth;
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("150", "900");