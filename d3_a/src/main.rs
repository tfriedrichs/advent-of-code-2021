fn main() {
    let input: Result<Vec<u32>, _> = include_str!("../input.txt")
        .lines()
        .map(|l| u32::from_str_radix(l, 2))
        .collect();
    let result = find_gamma(&input.expect("invalid input"));
    println!(
        "Gamma: {}, Epsilon: {}, Result: {}",
        result.0,
        result.1,
        result.0 * result.1
    );
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

#[cfg(test)]
mod test {
    use crate::find_gamma;

    #[test]
    fn example_test() {
        assert_eq!(
            (22, 9),
            find_gamma(&[
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ])
        )
    }
}
