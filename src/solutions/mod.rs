pub mod day1 {
    use std::include_str;

    pub fn solve() {
        let input: &str = include_str!(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/problems/day1/input.txt"
            )
        );

        let lines = input.lines();

        let values = lines.map(get_int_value_from_string).collect::<Vec<i32>>();

        let solution = values.into_iter()
            .reduce(|a, b| a + b)
            .expect("Failed to reduce the iterator")
            .to_string();

        println!("Day 1: {}", solution);
    }

    fn get_int_value_from_string(string: &str) -> i32 {
        let digits = string.chars().filter(
                |c| c.is_digit(10)
            )
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let first = digits.get(0).unwrap();
        let last = digits.get(digits.len() - 1).unwrap();

        let concatenated_first_lat = format!("{}{}", first, last);

        concatenated_first_lat.parse::<i32>().unwrap()
    }
}