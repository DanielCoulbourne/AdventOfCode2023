#![allow(dead_code)]

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
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let first = digits.get(0).unwrap();
        let last = digits.get(digits.len() - 1).unwrap();

        let concatenated_first_last = format!("{}{}", first, last);

        concatenated_first_last.parse::<i32>().unwrap()
    }

    fn replace_number_strings(init: &str) -> String {
        let numbers: Vec<(&str, &str)> = vec![("1", "one"), ("2", "two"), ("3", "three"), ("4", "four"), ("5", "five"), ("6", "six"), ("7", "seven"), ("8", "eight"), ("9", "nine"), ("0", "zero")];
   
        let haystack = init.to_string();
        let matches = numbers
            .iter()
            .fold(
                vec![],
                |acc, (number, word)| append_matches(haystack.clone(), acc, (number, word))
            );

        let number_string = matches
            .iter()
            .fold(
                String::new(),
                |acc, (_, number)| {
                    let mut result = acc.clone();
                    let char = number.to_string();

                    result.push_str(char.as_str());

                    result
                }
            );

        number_string
    }

    fn append_matches(haystack: String, acc: Vec<(usize, String)>, (number, string): (&str, &str)) -> Vec<(usize, String)> {
        let mut new_acc = acc.clone();

        let word_matches = haystack.match_indices(string).collect::<Vec<(usize, &str)>>();
        let number_matches = haystack.match_indices(number).collect::<Vec<(usize, &str)>>();

        new_acc.extend(
            word_matches.into_iter().map(|(index, _)| (index, number.to_string()))
        );
        
        new_acc.extend(
            number_matches.into_iter().map(|(index, _)| (index, number.to_string()))
        );

        new_acc.sort_by(
            |(a_index, _), (b_index, _)| a_index.cmp(b_index)
        );

        new_acc
    }
}