use crate::util::{ocr, Day};
use itertools::Itertools;

const INPUT: &str = include_str!("input/d08.in");
inventory::submit! {
    Day {year: 2019, day: 8, main}
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

pub fn solve(input: &str) -> (i32, String) {
    let data = input.trim().chars().map(|c| c.to_digit(10).unwrap());
    let data = data.chunks(LAYER_SIZE);

    let mut image = [2; LAYER_SIZE];
    let mut zero_count = i32::MAX;
    let mut part1 = 0;
    for layer in &data {
        let mut count = 0;
        let mut one_count = 0;
        let mut two_count = 0;
        for (i, v) in layer.enumerate() {
            match v {
                0 => count += 1,
                1 => one_count += 1,
                2 => two_count += 1,
                _ => (),
            }

            if image[i] == 2 {
                image[i] = v;
            }
        }
        if count < zero_count {
            zero_count = count;
            part1 = one_count * two_count;
        }
    }

    let mut render = String::new();
    for line in &image.iter().chunks(WIDTH) {
        render += line
            .map(|v| if *v == 0 { "." } else { "#" })
            .collect::<Vec<_>>()
            .join("")
            .as_str();
        render += "\n";
    }
    let part2 = ocr(render);

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
    assert_eq!(part1, 1320);
    assert_eq!(part2, "RCYKR");
}
