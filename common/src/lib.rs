use std::{fmt::Display, time::Instant};

pub trait AoCSolution {
    type Input;
    type Output: Display;

    fn name(&self) -> &str;
    fn parse_input(&self) -> Result<Self::Input, String>;
    fn compute_result(&self, input: Self::Input) -> Result<Self::Output, String>;

    fn run(&self) {
        println!();
        println!("=== Running {} ===", self.name());
        println!();
        let start = Instant::now();
        let input = self.parse_input().expect("invalid input");
        println!(
            "parsed input in: {:?}",
            Instant::now().saturating_duration_since(start)
        );
        let result = self
            .compute_result(input)
            .expect("error during computation");
        println!(
            "computed result in: {:?}",
            Instant::now().saturating_duration_since(start)
        );
        println!("Result: {}", result);
    }
}
