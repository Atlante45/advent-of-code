use crate::util::Day;

const INPUT: &str = include_str!("input/d04.in");
inventory::submit! {
    Day {year: 2019, day: 4, main}
}

fn check(n: i32) -> (bool, bool) {
    let mut last_digit = n % 10;
    let mut digits = n / 10;

    let mut repeat = false;
    let mut double = false;
    let mut count = 1;
    while digits > 0 {
        let new_digit = digits % 10;
        digits /= 10;

        if new_digit > last_digit {
            return (false, false);
        }
        if last_digit == new_digit {
            repeat = true;
            count += 1;
        } else {
            double |= count == 2;
            count = 1;
        }

        last_digit = new_digit;
    }
    double |= count == 2;

    (repeat, double)
}

pub fn solve(input: &str) -> (i32, i32) {
    let numbers: Vec<i32> = input
        .trim()
        .split('-')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for i in numbers[0]..numbers[1] {
        let (check1, check2) = check(i);
        if check1 {
            part1 += 1;
        }
        if check2 {
            part2 += 1;
        }
    }

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
    assert_eq!(part1, 1686);
    assert_eq!(part2, 1145);
}
