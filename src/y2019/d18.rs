use crate::util::Day;

const INPUT: &str = include_str!("input/d18.in");
inventory::submit! {
    Day {year: 2019, day: 18, main}
}

// struct Maze<'a> {
//     maze: &'a str,
// }

// impl Maze<'_> {}

pub fn solve(input: &str) -> (i32, i32) {
    let _lines: Vec<&str> = input.trim().lines().map(str::trim).collect();

    (0, 0)
}

pub fn main() {
    let (part1, part2) = solve(INPUT);

    let file = file!();
    println!("Problem {file}:");
    println!("    part 1: {part1}");
    println!("    part 2: {part2}");
}

#[cfg(test)]
#[test]
fn tests() {
    let (part1, part2) = solve(INPUT);
    assert_eq!(part1, 0);
    assert_eq!(part2, 0);
}
