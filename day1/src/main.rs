use common::AoCSolution;

fn main() {
    Day1::A.run();
    Day1::B.run();
}
enum Day1 {
    A,
    B,
}

impl AoCSolution for Day1 {
    type Input = Vec<u32>;
    type Output = usize;

    fn parse_input(&self) -> Result<Self::Input, String> {
        let input: Result<Vec<_>, _> = include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<u32>())
            .collect();
        input.map_err(|_| "could not parse input".to_string())
    }

    fn compute_result(&self, input: Self::Input) -> Result<Self::Output, String> {
        match self {
            Day1::A => count_increases(&input),
            Day1::B => count_increasing_windows(&input),
        }
    }

    fn name(&self) -> &str {
        match self {
            Day1::A => "Day1 - A",
            Day1::B => "Day1 - B",
        }
    }
}

fn count_increases(input: &[u32]) -> Result<usize, String> {
    Ok(input.windows(2).filter(|w| w[1] > w[0]).count())
}

fn count_increasing_windows(input: &[u32]) -> Result<usize, String> {
    let sums: Vec<_> = input.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    Ok(sums.windows(2).filter(|x| x[1] > x[0]).count())
}

#[cfg(test)]
mod test {
    use crate::Day1;
    use common::AoCSolution;

    #[test]
    fn monotonic_increase_a() {
        assert_eq!(Ok(3), Day1::A.compute_result(vec![1, 2, 3, 4]))
    }

    #[test]
    fn example_increase_a() {
        assert_eq!(
            Ok(7),
            Day1::A.compute_result(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        );
    }

    #[test]
    fn monotonic_increase_b() {
        assert_eq!(Ok(2), Day1::B.compute_result(vec![1, 2, 3, 4, 8]))
    }

    #[test]
    fn example_increase_b() {
        assert_eq!(
            Ok(5),
            Day1::B.compute_result(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        )
    }
}
