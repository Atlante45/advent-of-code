use crate::{util::Day, y2019::intcode::IntCode};

const INPUT: &'static str = include_str!("input/d05.in");
inventory::submit! {
    Day {year: 2019, day: 5, main: main}
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = IntCode::parse(input);

    let mut intcode = IntCode::load(program.clone());
    intcode.set_inputs(vec![1]);
    intcode.run();
    let outputs = intcode.get_outputs();
    let part1 = *outputs.last().unwrap();

    let mut intcode = IntCode::load(program);
    intcode.set_inputs(vec![5]);
    intcode.run();
    let outputs = intcode.get_outputs();
    let part2 = *outputs.last().unwrap();

    return (part1 as i32, part2 as i32);
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
    assert_eq!(part1, 15097178);
    assert_eq!(part2, 1558663);
}
