use crate::util::Day;

const INPUT: &'static str = include_str!("input/d01.in");
inventory::submit! {
    Day {year: 2019, day: 1, main: main}
}

fn fuel_req1(mass: &i32) -> i32 {
    return mass / 3 - 2;
}

fn fuel_req2(mass: &i32) -> i32 {
    let mut fuel = 0;

    let mut new_fuel = mass / 3 - 2;
    while new_fuel > 0 {
        fuel += new_fuel;
        new_fuel = new_fuel / 3 - 2
    }
    return fuel;
}

pub fn solve(input: &str) -> (i32, i32) {
    let numbers: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let part1: i32 = numbers.iter().map(|n| fuel_req1(n)).sum();
    let part2: i32 = numbers.iter().map(|n| fuel_req2(n)).sum();
    return (part1, part2);
}

pub fn main() {
    let (part1, part2) = solve(INPUT);

    println!("Problem {}:", file!());
    println!("    part 1: {part1}");
    println!("    part 2: {part2}");
}

#[cfg(test)]
#[test]
fn tests() {
    let (part1, part2) = solve(INPUT);
    assert_eq!(part1, 3263320);
    assert_eq!(part2, 4892135);
}

// use crate::util;

// pub const YEAR: usize = 2019;
// pub const DAY: usize = 1;

// impl util::Day for util::Problem<YEAR, DAY> {
//     fn example_input() -> &'static str {
//         include_str!("example.txt")
//     }
//     fn problem_input() -> &'static str {
//         include_str!("input/d01.in")
//     }

//     fn example_ans() -> (i64, i64) {
//         (34241, 51316)
//     }

//     fn part1(input: &str) -> i64 {
//         let numbers: Vec<i64> = input
//             .trim()
//             .split('\n')
//             .map(|s| s.trim().parse().unwrap())
//             .collect();

//         return numbers.iter().map(|n| fuel_req1(n)).sum();
//     }
//     fn part2(input: &str) -> i64 {
//         let numbers: Vec<i64> = input
//             .trim()
//             .split('\n')
//             .map(|s| s.trim().parse().unwrap())
//             .collect();

//         return numbers.iter().map(|n| fuel_req2(n)).sum();
//     }
// }
