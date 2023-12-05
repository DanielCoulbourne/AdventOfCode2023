#![allow(dead_code)]

mod deserialize;

pub mod day2 {
    use crate::day2::deserialize::{Game, ColorCount, Color};

    pub fn solve() {
        println!("=== DAY 2 ===");

        solve_a();
        solve_b();

    }

    pub fn solve_a() {
        let input: &str = include_str!(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/problems/day2/input.txt"
            )
        );

        let red_count = ColorCount { color: Color::Red, count: 12 };
        let green_count = ColorCount { color: Color::Green, count: 13 };
        let blue_count = ColorCount { color: Color::Blue, count: 14 };

        let game = Game::from_string(input);

        let possible_rounds_sum: i32 = game.rounds.iter().filter(
                |round| round.possible_with_color_counts(
                        red_count.clone(),
                        green_count.clone(),
                        blue_count.clone()
                    )
            ).map(
                |round| round.id
            ).sum();

        println!("2A: {}", possible_rounds_sum)
    }

    pub fn solve_b() {
        let input: &str = include_str!(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/problems/day2/input.txt"
            )
        );

        let game = Game::from_string(input);

        let solution: i64 = game.rounds.iter().map(
                |round| round.min_power()
            ).sum();

        println!("2B: {}", solution)
    }
}