use euclid::{Point2D, Vector2D};
use itertools::Itertools;

use super::intcode::IntCode;
use crate::util::Day;

const INPUT: &str = include_str!("input/d17.in");
inventory::submit! {
    Day {year: 2019, day: 17, main}
}

enum M {}
type Point = Point2D<i32, M>;
type Vector = Vector2D<i32, M>;

fn get_val(view: &Vec<&str>, i: i32, j: i32) -> char {
    let width = view.first().unwrap().len() as i32;
    let height = view.len() as i32;
    if i < 0 || i >= height || j < 0 || j >= width {
        return '.';
    }

    return view[i as usize].chars().nth(j as usize).unwrap();
}

fn compute_alignment(view: &Vec<&str>) -> i32 {
    let width = view.first().unwrap().len() as i32;
    let height = view.len() as i32;

    let mut alignment = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if get_val(view, i, j) == '#'
                && get_val(view, i, j - 1) == '#'
                && get_val(view, i, j + 1) == '#'
                && get_val(view, i - 1, j) == '#'
                && get_val(view, i + 1, j) == '#'
            {
                alignment += i * j;
            }
        }
    }

    alignment
}

fn on_path(view: &Vec<&str>, point: Point) -> bool {
    get_val(view, point.x, point.y) == '#'
}

const DIRS: [Vector; 4] = [
    Vector::new(-1, 0),
    Vector::new(0, -1),
    Vector::new(1, 0),
    Vector::new(0, 1),
];

fn compute_path(view: &Vec<&str>) -> Vec<String> {
    let mut cur = Point::new(0, 2);
    let mut dir = 0;

    let mut path = vec![];
    let mut count = 0;

    loop {
        while on_path(view, cur + DIRS[dir]) {
            count += 1;
            cur += DIRS[dir];
        }
        if count > 0 {
            path.push(count.to_string());
            count = 0;
        }

        let left = (dir + 1) % 4;
        let right = (dir + 3) % 4;
        if on_path(view, cur + DIRS[left]) {
            dir = left;
            path.push("L".to_string());
        } else if on_path(view, cur + DIRS[right]) {
            dir = right;
            path.push("R".to_string());
        } else {
            break;
        }
    }

    path
}

fn find_start(path: &[String]) -> usize {
    path.iter()
        .enumerate()
        .find_or_first(|(_, s)| !["A", "B", "C"].contains(&s.as_str()))
        .unwrap()
        .0
}

fn is_valid(path: &[String], start: usize, length: usize) -> bool {
    path.iter()
        .skip(start)
        .take(length)
        .all(|s| !["A", "B", "C"].contains(&s.as_str()))
}

fn is_repeat(path: &Vec<String>, start: usize, length: usize) -> bool {
    let func = &path[start..start + length];

    for i in start + 1..path.len() - length + 1 {
        let slice = &path[i..i + length];
        if slice == func {
            return true;
        }
    }
    false
}

fn replace(path: &Vec<String>, start: usize, length: usize, rep: String) -> Vec<String> {
    let func = &path[start..start + length];

    let mut res = path.clone();

    for i in (start..path.len() - length + 1).rev() {
        let slice = &path[i..i + length];
        if slice == func {
            res.splice(i..i + length, vec![&rep].into_iter().cloned());
        }
    }
    res
}

fn extract_func(path: &mut Vec<String>, rep: &str) -> String {
    let start = find_start(path);
    let mut length = 1;

    while is_valid(path, start, length + 1) && is_repeat(path, start, length + 1) {
        length += 1;
    }

    let func = path[start..start + length].join(",") + "\n";
    *path = replace(path, start, length, rep.to_string());

    func
}

fn compute_routines(path: Vec<String>) -> String {
    let mut path = path;

    let a = extract_func(&mut path, "A");
    let b = extract_func(&mut path, "B");
    let c = extract_func(&mut path, "C");

    let routine = path.join(",") + "\n";

    routine + &a + &b + &c
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = IntCode::parse(input);
    let mut intcode = IntCode::load(program);

    intcode.run();
    let output = intcode.get_outputs();

    let res = output
        .iter()
        .map(|v| char::from_u32(*v as u32).unwrap())
        .collect::<String>();
    intcode.clear_outputs();

    let view: Vec<&str> = res.trim().split('\n').collect();

    let alignment = compute_alignment(&view);

    let path = compute_path(&view);

    let mut routines = compute_routines(path);

    routines += "n\n";

    let inputs: Vec<i64> = routines.chars().map(|c| c as i64).collect();

    let mut program = IntCode::parse(input);
    program[0] = 2;
    let mut intcode = IntCode::load(program);
    intcode.set_inputs(inputs);
    intcode.run();
    let dust = intcode.get_outputs().last().unwrap();

    (alignment, *dust as i32)
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
    assert_eq!(part1, 5740);
    assert_eq!(part2, 1022165);
}
