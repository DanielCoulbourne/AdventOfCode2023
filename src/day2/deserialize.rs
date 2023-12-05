trait SplitAndTrim {
    fn split_and_trim(&self, separator: &str) -> Vec<&str>;
}

impl SplitAndTrim for &str {
    fn split_and_trim(&self, separator: &str) -> Vec<&str> {
        self.split(separator)
            .map(
                |split| split.trim()
            ).filter(
                |split| split != &""
            )
            .collect::<Vec<&str>>()
    }
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    pub fn from_string(string: &str) -> Color {
        match string {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => {
                panic!("Invalid color string");
            }
        }
    }
}


#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
pub struct ColorCount {
    pub color: Color,
    pub count: i32,
}

impl ColorCount {
    pub fn from_string(string: &str) -> ColorCount {
        let parts = string.split_and_trim(" ");

        let count_str = parts.get(0)
            .expect(format!("Couldn't find count in string {}", string).as_str());

        let color_str = parts.get(1)
            .expect(format!("Couldn't find color in string {}", string).as_str());

        let color = Color::from_string(&color_str);

        let count = count_str.parse::<i32>()
            .expect(format!("Couldn't parse string `{}` to i32." , count_str).as_str());

        ColorCount {
            color,
            count,
        }
    }
}

#[derive(Debug)]
pub struct Reveal {
    color_counts: Vec<ColorCount>,
}

impl Reveal {
    pub fn from_string(string: &str) -> Reveal {
        let color_counts = string
            .split_and_trim(",")
            .iter()
            .map(
                |color_reveal| ColorCount::from_string(&color_reveal)
            )
            .collect();

        Reveal {
            color_counts,
        }
    }

    pub fn count_of_color(&self, color: Color) -> i32 {
        *&self.color_counts.iter()
            .find(
            |count| {
                count.color == color
            }
        ).unwrap_or(
            &ColorCount { color: color, count: 0 }
        ).count
    }
}

#[derive(Debug)]
pub struct GameRound {
    pub id: i32,
    reveals: Vec<Reveal>,
}

impl GameRound {
    pub fn from_string(string: &str) -> GameRound {
        let parts = string.split_and_trim(":");

        let id = parts.get(0).unwrap()
            .split_and_trim(" ")
            .get(1).unwrap()
            .parse::<i32>()
            .unwrap();

        let reveals = parts.get(1).unwrap()
            .split_and_trim(";")
            .iter()
            .map(
                |reveal| Reveal::from_string(reveal)
            ).collect();

        GameRound {
            id,
            reveals,
        }
    }

    pub fn max_revealed_of_color(&self, color: Color) -> i32 {
        *&self.reveals.iter().map(
            |reveal| reveal.count_of_color(color.clone())
        ).max().unwrap()
    }

    // The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
    // The minimum power of a round is the power of it's minimum cubes
    pub fn min_power(&self) -> i64 {
        (
            &self.max_revealed_of_color(Color::Red)
            * &self.max_revealed_of_color(Color::Green)
            * &self.max_revealed_of_color(Color::Blue)
        ) as i64
    }

    pub fn possible_with_color_counts(
        &self,
        red_count: ColorCount,
        green_count: ColorCount,
        blue_count: ColorCount,
    ) -> bool {
        red_count.count.ge(
            &self.max_revealed_of_color(
                Color::Red
            )
        ) &&
        green_count.count.ge(
            &self.max_revealed_of_color(
                Color::Green
            )
        ) &&
        blue_count.count.ge(
            &self.max_revealed_of_color(
                Color::Blue
            )
        )
    }
}

#[derive(Debug)]
pub struct Game {
    pub rounds: Vec<GameRound>,
}

impl Game {
    pub fn from_string(string: &str) -> Game {
        let rounds = string.split_and_trim("\n")
            .iter()
            .map(
                |round| GameRound::from_string(round)
            )
            .collect::<Vec<GameRound>>();

        Game { rounds }
    }
}