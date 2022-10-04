use euclid::{Point2D, Vector2D};
use std::collections::HashSet;

use super::intcode::IntCode;
use crate::util::Day;

const INPUT: &'static str = include_str!("input/d15.in");
inventory::submit! {
    Day {year: 2019, day: 15, main: main}
}

enum M {}
type Point = Point2D<i32, M>;
type Vector = Vector2D<i32, M>;

fn vec_dir(dir: i64) -> Vector {
    match dir {
        1 => Vector::new(1, 0),
        2 => Vector::new(-1, 0),
        3 => Vector::new(0, -1),
        4 => Vector::new(0, 1),
        _ => panic!("Invalid move"),
    }
}

fn inverse(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("Invalid move"),
    }
}

fn execute_move(intcode: &mut IntCode, dir: i64) -> Option<&i64> {
    intcode.set_inputs(vec![dir]);
    intcode.run();
    intcode.get_outputs().last()
}

fn step_to_oxygen(
    intcode: &mut IntCode,
    set: &mut HashSet<Point>,
    cur: &Point,
    depth: i32,
) -> Option<i32> {
    for dir in 1..=4 {
        let mp = vec_dir(dir);
        let tgt = *cur + mp;

        if set.contains(&tgt) {
            continue;
        }

        let ret = execute_move(intcode, dir);

        match ret {
            Some(0) => (),
            Some(1) => {
                set.insert(tgt);
                if let Some(res) = step_to_oxygen(intcode, set, &tgt, depth + 1) {
                    return Some(res);
                }
                execute_move(intcode, inverse(dir));
            }
            Some(2) => return Some(depth + 1),
            _ => panic!("Unknown return code"),
        };
    }

    return None;
}

fn traverse(intcode: &mut IntCode, set: &mut HashSet<Point>, cur: &Point, depth: i32) -> i32 {
    let mut max = 0;

    for dir in 1..=4 {
        let mp = vec_dir(dir);
        let tgt = *cur + mp;

        if set.contains(&tgt) {
            continue;
        }

        let ret = execute_move(intcode, dir);

        match ret {
            Some(0) => (),
            Some(1) => {
                set.insert(tgt);
                let depth = traverse(intcode, set, &tgt, depth + 1);
                max = max.max(depth);
                execute_move(intcode, inverse(dir));
            }
            Some(2) => (),
            _ => panic!("Unknown return code"),
        };
    }

    return max + 1;
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = IntCode::parse(input);
    let mut intcode = IntCode::load(program);

    let mut set = HashSet::new();

    let cur = Point::zero();
    let distance = step_to_oxygen(&mut intcode, &mut set, &cur, 0);

    set = HashSet::new();
    let depth = traverse(&mut intcode, &mut set, &cur, 0);

    return (distance.unwrap(), depth - 1);
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
    assert_eq!(part1, 424);
    assert_eq!(part2, 446);
}
