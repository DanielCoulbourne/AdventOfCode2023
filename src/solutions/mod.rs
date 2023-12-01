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
        let cleaned_string = replace_number_strings(string);

        let digits = cleaned_string
            .chars()
            .filter(
                |c| c.is_digit(10)
            )
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let first = digits.get(0).unwrap();
        let last = digits.get(digits.len() - 1).unwrap();

        let concatenated_first_last = format!("{}{}", first, last);

        println!("{} -> {} -> {}", string, cleaned_string, concatenated_first_last);

        concatenated_first_last.parse::<i32>().unwrap()
    }

    fn replace_number_strings(init: &str) -> String {
        // replace isnt good enough.
        // some of the numbers share a letter
        // like eightwo is both 8 and 2
        
        init.replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
            // .replace("zero", "0") --- the instructions dont include zero
    }
}