use crate::util::Day;

use super::intcode::IntCode;

const INPUT: &'static str = include_str!("input/d09.in");
inventory::submit! {
    Day {year: 2019, day: 9, main: main}
}

pub fn solve(input: &str) -> (i64, i64) {
    let program = IntCode::parse(input);

    let mut intcode = IntCode::load(program.clone());
    intcode.set_inputs(vec![1]);
    intcode.run();
    let part1 = *intcode.get_outputs().last().unwrap();

    let mut intcode = IntCode::load(program.clone());
    intcode.set_inputs(vec![2]);
    intcode.run();
    let part2 = *intcode.get_outputs().last().unwrap();

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
    assert_eq!(part1, 3460311188);
    assert_eq!(part2, 42202);
}
