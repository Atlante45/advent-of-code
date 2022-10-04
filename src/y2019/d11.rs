use euclid::{Point2D, Vector2D};
use std::collections::HashMap;

use super::intcode::IntCode;
use crate::util::{ocr, Day};

enum M {}
type Point = Point2D<i32, M>;
type Vector = Vector2D<i32, M>;

const INPUT: &'static str = include_str!("input/d11.in");
inventory::submit! {
    Day {year: 2019, day: 11, main: main}
}

pub fn solve(input: &str) -> (i32, String) {
    let dirs: Vec<Vector> = vec![
        Vector::new(0, -1),
        Vector::new(1, 0),
        Vector::new(0, 1),
        Vector::new(-1, 0),
    ];

    let program = IntCode::parse(input);
    let mut intcode = IntCode::load(program);

    let mut pos = Point::new(0, 0);
    let mut rot = 0;

    let mut min = pos;
    let mut max = pos;

    let mut tiles = HashMap::from([(pos, 1)]);
    while !intcode.halted() {
        let tile = *tiles.get(&pos).unwrap_or(&0);

        intcode.set_inputs(vec![tile]);
        intcode.run();
        let outputs = intcode.get_outputs();
        assert_eq!(outputs.len(), 2);
        tiles.insert(pos, outputs[0]);

        rot = (rot + 2 * outputs[1] as i32 - 1).rem_euclid(4);
        pos += dirs[rot as usize];

        min = min.min(pos);
        max = max.max(pos);

        intcode.clear_outputs();
    }

    let part1 = tiles.len() as i32;

    let mut render = String::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let tile = *tiles.get(&Point::new(x, y)).unwrap_or(&0);
            render += if tile == 1 { "#" } else { "." }
        }
        render += "\n";
    }
    let part2 = ocr(render);

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
    assert_eq!(part1, 249);
    assert_eq!(part2, "APFKRKBR");
}
