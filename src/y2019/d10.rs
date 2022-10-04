use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use num::integer::gcd;

use crate::util::Day;

const INPUT: &str = include_str!("input/d10.in");
inventory::submit! {
    Day {year: 2019, day: 10, main}
}

fn count(map: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if map[x][y] != '#' {
        return 0;
    }

    let mut set = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '#' || (i, j) == (x, y) {
                continue;
            }

            let dx = i as i32 - x as i32;
            let dy = j as i32 - y as i32;

            let gcd = gcd(dx.abs(), dy.abs());

            set.insert((dx / gcd, dy / gcd));
        }
    }

    return set.len() as i32;
}

fn dcomp(a: (i32, i32), b: (i32, i32)) -> Ordering {
    if a.0.abs() + a.1.abs() > b.0.abs() + b.1.abs() {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn rcomp(a: (i32, i32), b: (i32, i32)) -> Ordering {
    let a = (a.1, a.0);
    let b = (b.1, b.0);

    if a.0 == 0 && a.1 < 0 {
        return Ordering::Less;
    }
    if b.0 == 0 && b.1 < 0 {
        return Ordering::Greater;
    }

    let a_angle = (a.0 as f32).atan2(a.1 as f32);
    let b_angle = (b.0 as f32).atan2(b.1 as f32);
    if a_angle > b_angle {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn order(map: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut set: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '#' || (i, j) == (x, y) {
                continue;
            }

            let dx = i as i32 - x as i32;
            let dy = j as i32 - y as i32;

            let gcd = gcd(dx.abs(), dy.abs());

            let new_coords = (dx, dy);
            let v = set.entry((dx / gcd, dy / gcd)).or_default();
            let pos = v.binary_search_by(|c| dcomp(*c, new_coords)).unwrap_err();
            v.insert(pos, new_coords);
        }
    }

    let mut values: Vec<Vec<(i32, i32)>> = vec![];
    for (k, v) in set {
        let pos = values.binary_search_by(|c| rcomp(c[0], k)).unwrap_err();
        values.insert(pos, v);
    }

    let mut count = 0;
    loop {
        for i in values.iter_mut() {
            if let Some(coords) = i.pop() {
                count += 1;
                if count == 200 {
                    return (y as i32 + coords.1) * 100 + x as i32 + coords.0;
                }
            }
        }
    }
}

pub fn solve(input: &str) -> (i32, i32) {
    let lines = input.trim().lines();
    let map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut part1 = 0;
    let mut coords = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let count = count(&map, i, j);
            if count > part1 {
                part1 = count;
                coords = (i, j);
            }
        }
    }

    let part2 = order(&map, coords.0, coords.1);

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
    assert_eq!(part1, 253);
    assert_eq!(part2, 815);
}
