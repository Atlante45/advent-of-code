use std::collections::HashMap;

use crate::util::Day;

const INPUT: &str = include_str!("input/d06.in");
inventory::submit! {
    Day {year: 2019, day: 6, main}
}

fn find_path<'a>(parent_map: &'a HashMap<&str, &str>, object: &'a str) -> Vec<&'a str> {
    if object == "COM" {
        return vec![object];
    }
    let mut path = find_path(parent_map, parent_map[object]);
    path.push(object);
    path
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut orbits_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parents_map = HashMap::new();

    for line in input.lines() {
        let objects: Vec<&str> = line.trim().split(')').collect();
        orbits_map.entry(objects[0]).or_default().push(objects[1]);
        parents_map.insert(objects[1], objects[0]);
    }

    let mut part1 = 0;

    let mut list = vec!["COM"];
    let mut depth = 0;
    while !list.is_empty() {
        part1 += list.len() as i32 * depth;
        let mut new_list = vec![];
        for parent in list {
            let objects = orbits_map.entry(parent).or_default();
            new_list.append(objects);
        }
        list = new_list;
        depth += 1;
    }

    let patha = find_path(&parents_map, "YOU");
    let pathb = find_path(&parents_map, "SAN");
    let mut i = 0;
    while patha[i] == pathb[i] {
        i += 1;
    }
    let part2 = (patha.len() - i + pathb.len() - i - 2) as i32;

    (part1, part2)
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
    assert_eq!(part1, 417916);
    assert_eq!(part2, 523);
}
