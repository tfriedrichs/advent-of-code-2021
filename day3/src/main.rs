use std::fmt::Display;

use common::AoCSolution;

fn main() {
    Day3::A.run();
    Day3::B { width: 12 }.run();
}

enum Day3 {
    A,
    B { width: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Day3Result {
    A(u32, u32, u32),
    B(u32, u32, u32),
}

impl Display for Day3Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day3Result::A(gamma, epsilon, result) => {
                write!(f, "{}\n\tGamma: {}\n\tEpsilon: {}", result, gamma, epsilon)
            }
            Day3Result::B(oxygen, co2, result) => {
                write!(f, "{}\n\tOxygen: {}\n\tCO2: {}", result, oxygen, co2)
            }
        }
    }
}

impl AoCSolution for Day3 {
    type Input = Vec<u32>;
    type Output = Day3Result;

    fn parse_input(&self) -> Result<Self::Input, String> {
        let input: Result<Vec<u32>, _> = include_str!("../input.txt")
            .lines()
            .map(|l| u32::from_str_radix(l, 2))
            .collect();
        input.map_err(|_| "could not parse input".to_string())
    }

    fn compute_result(&self, input: Self::Input) -> Result<Self::Output, String> {
        match self {
            Day3::A => {
                let res = find_gamma(&input);
                Ok(Day3Result::A(res.0, res.1, res.0 * res.1))
            }
            Day3::B { width: w } => {
                let res = find_ratings(*w, &input);
                Ok(Day3Result::B(res.0, res.1, res.0 * res.1))
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Day3::A => "Day3 - A",
            Day3::B { width: _ } => "Day3 - B",
        }
    }
}

fn find_gamma(values: &[u32]) -> (u32, u32) {
    let mut gamma: u32 = 0;
    let mut significant_digits = 0;
    for i in 0..32 {
        let mask = 1 << i;
        let sum: u32 = values.iter().map(|v| (v & (mask)) >> i).sum();
        if sum == 0 {
            significant_digits = i;
            break;
        }
        let is_set: u32 = if (2 * sum) as usize >= values.len() {
            1
        } else {
            0
        };
        gamma += is_set << i;
    }
    let epsilon = !gamma & ((1 << significant_digits) - 1);
    (gamma, epsilon)
}

fn find_ratings(width: usize, values: &[u32]) -> (u32, u32) {
    let mut oxygen = values.to_vec();
    let mut co2 = values.to_vec();

    let shared = find_most_common_bit(width - 1, values);
    oxygen.retain(|v| ((v & 1 << (width - 1)) ^ (shared << width - 1)) == 0);
    co2.retain(|v| ((v & 1 << (width - 1)) ^ (shared << width - 1)) != 0);

    for i in 1..width {
        if oxygen.len() <= 1 && co2.len() <= 1 {
            break;
        }
        if oxygen.len() > 1 {
            let bit = find_most_common_bit(width - i - 1, &oxygen);
            oxygen.retain(|v| ((v & 1 << (width - i - 1)) ^ (bit << width - i - 1)) == 0);
        }
        if co2.len() > 1 {
            let bit = find_most_common_bit(width - i - 1, &co2);
            co2.retain(|v| ((v & 1 << (width - i - 1)) ^ (bit << width - i - 1)) != 0);
        }
    }
    (*oxygen.first().unwrap(), *co2.first().unwrap())
}

fn find_most_common_bit(index: usize, values: &[u32]) -> u32 {
    let mask = 1 << index;
    let sum: u32 = values.iter().map(|v| (v & mask) >> index).sum();
    if (2 * sum) as usize >= values.len() {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use common::AoCSolution;

    use crate::{Day3, Day3Result};

    #[test]
    fn example_test_a() {
        assert_eq!(
            Ok(Day3Result::A(22, 9, 198)),
            Day3::A.compute_result(vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ])
        )
    }

    #[test]
    fn example_test_b() {
        assert_eq!(
            Ok(Day3Result::B(23, 10, 230)),
            Day3::B { width: 5 }.compute_result(vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ])
        )
    }
}
