use super::intcode::*;
use crate::util::Day;

const INPUT: &str = include_str!("input/d02.in");
inventory::submit! {
    Day {year: 2019, day: 2, main}
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = IntCode::parse(input);

    let mut intcode = IntCode::load(program.clone());
    intcode.set(1, 1, 12);
    intcode.set(2, 1, 2);
    let part1 = intcode.run();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = IntCode::load(program.clone());
            intcode.set(1, 1, noun);
            intcode.set(2, 1, verb);
            if intcode.run() == 19690720 {
                let part2 = 100 * noun + verb;
                return (part1 as i32, part2 as i32);
            }
        }
    }

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
    assert_eq!(part1, 9706670);
    assert_eq!(part2, 2552);
}