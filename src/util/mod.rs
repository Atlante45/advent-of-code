use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Day {
    pub year: u32,
    pub day: u32,
    pub main: fn(),
}

impl Day {
    // pub fn new(year: u32, day: u32, main: fn()) -> Day {
    //     Day { year: year, day: day, main: main }
    // }
}
inventory::collect!(Day);

#[macro_export]
macro_rules! get_year {
    () => {
        match file!().get(5..9) {
            Some(year) => year,
            None => panic!("Unexpected file"),
        }
    };
}

#[macro_export]
macro_rules! get_day {
    () => {
        match file!().get(11..13) {
            Some(day) => day,
            None => panic!("Unexpected file"),
        }
    };
}

lazy_static! {
    static ref ALPHABET: HashMap<&'static str, &'static str> = HashMap::from([
        (".##.\n#..#\n#..#\n####\n#..#\n#..#", "A"),
        ("###.\n#..#\n###.\n#..#\n#..#\n###.", "B"),
        (".##.\n#..#\n#...\n#...\n#..#\n.##.", "C"),
        ("####\n#...\n###.\n#...\n#...\n####", "E"),
        ("####\n#...\n###.\n#...\n#...\n#...", "F"),
        (".##.\n#..#\n#...\n#.##\n#..#\n.###", "G"),
        ("#..#\n#..#\n####\n#..#\n#..#\n#..#", "H"),
        ("###.\n.#..\n.#..\n.#..\n.#..\n###.", "I"),
        ("..##\n...#\n...#\n...#\n#..#\n.##.", "J"),
        ("#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#", "K"),
        ("#...\n#...\n#...\n#...\n#...\n####", "L"),
        (".##.\n#..#\n#..#\n#..#\n#..#\n.##.", "O"),
        ("###.\n#..#\n#..#\n###.\n#...\n#...", "P"),
        ("###.\n#..#\n#..#\n###.\n#.#.\n#..#", "R"),
        (".###\n#...\n#...\n.##.\n...#\n###.", "S"),
        ("#..#\n#..#\n#..#\n#..#\n#..#\n.##.", "U"),
        ("#...\n#...\n.#.#\n..#.\n..#.\n..#.", "Y"),
        ("####\n...#\n..#.\n.#..\n#...\n####", "Z"),
    ]);
}
// const FILL: char = '#';
const EMPTY: char = '.';
const WIDTH: usize = 4;
const HEIGHT: usize = 6;

pub fn ocr(drawing: String) -> String {
    let lines: Vec<&str> = drawing.trim().split('\n').collect();
    assert_eq!(lines.len(), HEIGHT);
    let length = lines[0].len();
    assert!(lines.iter().all(|line| line.len() == length));

    let mut result = String::new();

    let mut index = 0;
    while length - index > WIDTH {
        let skip = lines
            .iter()
            .all(|&line| line.chars().nth(index).unwrap() == EMPTY);
        if skip {
            index += 1;
            continue;
        }

        let letter = lines
            .iter()
            .map(|&line| line.chars().skip(index).take(WIDTH).join(""))
            .join("\n");
        let letter = match ALPHABET.get(letter.as_str()) {
            Some(c) => c,
            None => panic!("No matching letter"),
        };
        result += letter;
        index += if *letter == "Y" { 5 } else { 4 };
    }

    return result;
}

// #[macro_export]
// macro_rules! register {
//     () => {
//         inventory::submit! {Day::new(get_year!().parse().unwrap(), get_day!().parse().unwrap(), main)}
//     };
// }

// extern crate termion;
// use termion::color;

// pub struct Problem<const Y: usize, const D: usize> {}

// pub trait Day {
//     fn example_input() -> &'static str;
//     fn problem_input() -> &'static str;
//     fn example_ans() -> (i64, i64);

//     fn part1(input: &str) -> i64;
//     fn part2(input: &str) -> i64;
// }

// impl<T> Day for T {
//     default fn example_input() -> &'static str {
//         return "";
//     }
//     default fn problem_input() -> &'static str {
//         return "";
//     }

//     default fn example_ans() -> (i64, i64) {
//         return (0, 0);
//     }

//     default fn part1(input: &str) -> i64 {
//         return 0;
//     }

//     default fn part2(input: &str) -> i64 {
//         return 0;
//     }
// }

// pub fn run_example<const Y: usize, const D: usize>() {
//     let input = <Problem<Y, D> as Day>::example_input();

//     if input.len() == 0 {
//         return;
//     }

//     let red = color::Fg(color::Red).to_string();
//     let green = color::Fg(color::Green).to_string();

//     let part1_res = <Problem<Y, D> as Day>::part1(input);
//     let part2_res = <Problem<Y, D> as Day>::part2(input);
//     let (part1_ans, part2_ans) = <Problem<Y, D> as Day>::example_ans();

//     println!("Example:");
//     print!("{}", if part1_res == part1_ans { &green } else { &red });
//     println!("    part 1: {} == {}", part1_res, part1_ans);
//     print!("{}", if part2_res == part2_ans { &green } else { &red });
//     println!("    part 2: {} == {}", part2_res, part2_ans);
//     print!("{}", color::Fg(color::Reset));
// }

// pub fn run_problem<const Y: usize, const D: usize>() {
//     let input = <Problem<Y, D> as Day>::problem_input();

//     if input.len() == 0 {
//         return;
//     }

//     println!("Problem:");
//     println!("    part 1: {}", <Problem<Y, D> as Day>::part1(input));
//     println!("    part 2: {}", <Problem<Y, D> as Day>::part2(input));
// }

// pub fn solve<const Y: usize, const D: usize>() {
//     run_example::<Y, D>();
//     run_problem::<Y, D>();
// }

// #[macro_export]
// macro_rules! test {
//     ( $( $x:expr ),* ) => {

//         match 1 {
//             $(
//                 $x => util::solve::<2019, $x>(),
//             )*
//             _ => println!(""),
//         }
//     };
// }
