use core::time;
use std::{collections::HashMap, thread::sleep};

use crate::util::Day;

use super::intcode::IntCode;

const INPUT: &str = include_str!("input/d13.in");
inventory::submit! {
    Day {year: 2019, day: 13, main}
}

const RENDER: bool = false;

fn render(tiles: &HashMap<(i64, i64), i64>, score: i64, max_x: i64, max_y: i64) {
    let mut draw = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            draw += match tiles.get(&(x, y)) {
                Some(1) => {
                    if y == 0 && x == 0 {
                        "+"
                    } else if y == 0 {
                        "-"
                    } else {
                        "|"
                    }
                }
                Some(2) => "=",
                Some(3) => "_",
                Some(4) => "0",
                Some(_) => " ",
                None => " ",
            }
        }
        draw += "\n";
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("Score: {}\n{}\n", score, draw);
    sleep(time::Duration::from_millis(16));
}

pub fn solve(input: &str) -> (i32, i32) {
    let program = IntCode::parse(input);
    let mut intcode = IntCode::load(program);

    intcode.set(0, 1, 2);
    intcode.run();
    let outputs = intcode.get_outputs();

    let mut part1 = 0;

    let mut tiles = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut score = 0;

    let mut ball = 0;
    let mut ball_level = 0;
    let mut paddle = 0;
    let mut paddle_level = 0;

    for chunk in outputs.chunks(3) {
        let (x, y) = (chunk[0], chunk[1]);
        let val = chunk[2];

        if (x, y) == (-1, 0) {
            score = val;
            continue;
        }

        if RENDER && val != 0 {
            tiles.insert((x, y), val);
        }

        match val {
            2 => part1 += 1,
            3 => {
                paddle = x;
                paddle_level = y;
            }
            4 => {
                ball = x;
                ball_level = y;
            }
            _ => (),
        }

        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    if RENDER {
        render(&tiles, score, max_x, max_y);
    }
    intcode.clear_outputs();

    let mut dir = 1;
    while !intcode.halted() {
        let mut ball_next = ball + dir;
        if ball_next == 0 {
            ball_next += 2;
        }
        if ball_next == max_x {
            ball_next -= 2;
        }
        let mut m = ball_next.cmp(&paddle) as i64;
        if ball_level == paddle_level - 1 && ball == paddle {
            m = 0;
        }

        intcode.set_inputs(vec![m]);
        intcode.run();
        let outputs = intcode.get_outputs();

        for chunk in outputs.chunks(3) {
            let (x, y) = (chunk[0], chunk[1]);
            let val = chunk[2];

            if (x, y) == (-1, 0) {
                score = val;
                continue;
            }

            if RENDER && val != 10 {
                tiles.insert((x, y), val);
            }

            if val == 3 {
                paddle = x;
            } else if val == 4 {
                dir = x - ball;
                ball = x;
                ball_level = y;
            }
        }

        if RENDER {
            render(&tiles, score, max_x, max_y);
        }
        intcode.clear_outputs();
    }

    return (part1 as i32, score as i32);
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
    assert_eq!(part1, 280);
    assert_eq!(part2, 13298);
}
