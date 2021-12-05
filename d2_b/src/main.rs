use std::str::FromStr;

fn main() {
    let input: Result<Vec<Command>, _> = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<Command>())
        .collect();
    let pos = locate_position(&input.expect("invalid input"));
    println!(
        "X: {}, Y: {}, Aim: {}, Result: {}",
        pos.0,
        pos.1,
        pos.2,
        pos.0 * pos.1
    )
}

fn locate_position(commands: &[Command]) -> (i32, i32, i32) {
    commands
        .iter()
        .map::<(i32, i32, i32), _>(|c| match c {
            Command::Forward(x) => (*x, 0, 0),
            Command::Up(x) => (0, 0, -(*x)),
            Command::Down(x) => (0, 0, *x),
        })
        .fold((0, 0, 0), |l, r| (l.0 + r.0, l.1 + (r.0 * l.2), l.2 + r.2))
}

#[derive(Clone, Copy)]
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
            _ => Err("invalid command".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{locate_position, Command};

    #[test]
    fn example_test() {
        assert_eq!(
            (15, 60, 10),
            locate_position(&[
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
