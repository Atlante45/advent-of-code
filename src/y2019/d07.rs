use std::cmp;

use crate::util::Day;
use itertools::Itertools;

use super::intcode::{IntCode, Program};

const INPUT: &str = include_str!("input/d07.in");
inventory::submit! {
    Day {year: 2019, day: 7, main}
}

fn run_sequence(program: &Program, sequence: &[i32]) -> i32 {
    let mut signal = 0;

    let mut amps: [IntCode; 5] = [
        IntCode::load(program.clone()),
        IntCode::load(program.clone()),
        IntCode::load(program.clone()),
        IntCode::load(program.clone()),
        IntCode::load(program.clone()),
    ];

    for i in 0..5 {
        let intcode = &mut amps[i];
        intcode.set_inputs(vec![sequence[i] as i64]);
        intcode.run();
    }

    while !amps[4].halted() {
        for intcode in &mut amps {
            intcode.set_inputs(vec![signal]);
            intcode.run();
            let outputs = intcode.get_outputs();
            signal = *outputs.last().unwrap();
        }
    }

    assert!(amps.iter().all(|amp| amp.halted()));

    return signal as i32;
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = IntCode::parse(input);

    let mut part1 = 0;
    for sequence in (0..5).permutations(5) {
        let output = run_sequence(&program, &sequence);
        part1 = cmp::max(part1, output);
    }

    let mut part2 = 0;
    for sequence in (5..10).permutations(5) {
        let output = run_sequence(&program, &sequence);
        part2 = cmp::max(part2, output);
    }

    return (part1, part2);
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
    assert_eq!(part1, 368584);
    assert_eq!(part2, 35993240);
}
