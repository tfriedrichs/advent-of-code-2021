use std::{fmt::Display, str::FromStr};

use common::AoCSolution;

fn main() {
    Day2::A.run();
    Day2::B.run();
}

enum Day2 {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Day2Result {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
    result: i32,
}

impl Display for Day2Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\tHorizontal Position: {}\n\tDepth: {}\n\tAim: {}",
            self.result, self.horizontal_position, self.depth, self.aim
        )
    }
}

impl AoCSolution for Day2 {
    type Input = Vec<Command>;
    type Output = Day2Result;

    fn parse_input(&self) -> Result<Self::Input, String> {
        let input: Result<Vec<Command>, _> = include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<Command>())
            .collect();
        input.map_err(|_| "could not parse input".to_string())
    }

    fn compute_result(&self, input: Self::Input) -> Result<Self::Output, String> {
        match self {
            Day2::A => {
                let pos = locate_position(&input);
                Ok(Day2Result {
                    horizontal_position: pos.0,
                    depth: pos.1,
                    aim: 0,
                    result: pos.0 * pos.1,
                })
            }
            Day2::B => {
                let pos = locate_position_with_aim(&input);
                Ok(Day2Result {
                    horizontal_position: pos.0,
                    depth: pos.1,
                    aim: pos.2,
                    result: pos.0 * pos.1,
                })
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Day2::A => "Day2 - A",
            Day2::B => "Day2 - B",
        }
    }
}

fn locate_position(commands: &[Command]) -> (i32, i32) {
    commands
        .iter()
        .map::<(i32, i32), _>(|c| match c {
            Command::Forward(x) => (*x, 0),
            Command::Up(x) => (0, -(*x)),
            Command::Down(x) => (0, *x),
        })
        .fold((0, 0), |l, r| (l.0 + r.0, l.1 + r.1))
}

fn locate_position_with_aim(commands: &[Command]) -> (i32, i32, i32) {
    commands
        .iter()
        .map::<(i32, i32, i32), _>(|c| match c {
            Command::Forward(x) => (*x, 0, 0),
            Command::Up(x) => (0, 0, -(*x)),
            Command::Down(x) => (0, 0, *x),
        })
        .fold((0, 0, 0), |l, r| (l.0 + r.0, l.1 + (r.0 * l.2), l.2 + r.2))
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, value) = s.split_once(' ').ok_or("no whitespace")?;
        let value = value
            .parse::<i32>()
            .map_err(|_| "invalid value".to_string())?;
        match command {
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            "forward" => Ok(Command::Forward(value)),
            _ => Err("invalid command".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use common::AoCSolution;

    use crate::{Command, Day2, Day2Result};

    #[test]
    fn example_test_a() {
        assert_eq!(
            Ok(Day2Result {
                horizontal_position: 15,
                depth: 10,
                aim: 0,
                result: 150
            }),
            Day2::A.compute_result(vec![
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2)
            ])
        )
    }

    #[test]
    fn example_test_b() {
        assert_eq!(
            Ok(Day2Result {
                horizontal_position: 15,
                depth: 60,
                aim: 10,
                result: 900
            }),
            Day2::B.compute_result(vec![
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2)
            ])
        )
    }
}
