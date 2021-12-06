const WIDTH: usize = 12;

fn main() {
    let input: Result<Vec<u32>, _> = include_str!("../input.txt")
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect();
    let result = find_ratings(WIDTH, &input.expect("invalid input"));
    println!(
        "Oxygen: {}, CO2: {}, Result: {}",
        result.0,
        result.1,
        result.0 * result.1
    );
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
    use crate::find_ratings;

    #[test]
    fn example_test() {
        assert_eq!(
            (23, 10),
            find_ratings(
                5,
                &[
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ]
            )
        )
    }
}
