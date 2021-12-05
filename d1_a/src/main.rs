fn main() {
    let input: Result<Vec<_>, _> = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<u32>())
        .collect();
    println!(
        "Result: {}",
        count_increases(&input.expect("could not read input"))
    );
}

fn count_increases(values: &[u32]) -> usize {
    return values.windows(2).filter(|w| w[1] > w[0]).count();
}

#[cfg(test)]
mod test {
    use crate::count_increases;

    #[test]
    fn monotonic_increase() {
        assert_eq!(3, count_increases(&[1, 2, 3, 4]))
    }

    #[test]
    fn example_increase() {
        assert_eq!(
            7,
            count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        )
    }
}
