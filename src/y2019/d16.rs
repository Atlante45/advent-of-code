use crate::util::Day;

const INPUT: &'static str = include_str!("input/d16.in");
inventory::submit! {
    Day {year: 2019, day: 16, main: main}
}

fn compute_sum(input: &Vec<i32>, index: usize) -> i32 {
    let rounds = index + 1;
    let mut sum = 0;
    for r in 0..rounds {
        let mut i = rounds - 1 + r;
        while i < input.len() {
            sum += input[i];
            i += 4 * rounds;
        }
        i = 3 * rounds - 1 + r;
        while i < input.len() {
            sum -= input[i];
            i += 4 * rounds;
        }
    }
    return (sum % 10).abs();
}

fn do_phase(input: Vec<i32>) -> Vec<i32> {
    let size = input.len();
    (0..size).map(|i| compute_sum(&input, i)).collect()
}

fn part1(signal: &Vec<i32>) -> i32 {
    let mut signal = signal.clone();
    for _ in 0..100 {
        signal = do_phase(signal);
    }
    return signal.iter().take(8).fold(0, |acc, x| 10 * acc + *x);
}

fn part2(signal: &Vec<i32>) -> i32 {
    let num_signals = 10_000;
    let full_offset = signal.iter().take(7).fold(0, |acc, x| 10 * acc + *x);
    let full_length = num_signals * signal.len() as i32;
    let inverse_offset = full_length - full_offset;

    let num_phases = 100;
    let mut vec = vec![0; num_phases];
    let mut res = vec![];

    for i in 0..inverse_offset {
        let index = signal.len() - (i as usize % signal.len()) - 1;
        let n = signal[index];

        let mut acc = 0;
        vec = vec
            .iter()
            .map(|v| -> i32 {
                acc += v;
                (n + acc) % 10
            })
            .collect();

        if inverse_offset - i - 1 < 8 {
            res.push(*vec.last().unwrap());
        }
    }

    res.reverse();
    return res.iter().fold(0, |acc, x| 10 * acc + *x);
}

pub fn solve(input: &str) -> (i32, i32) {
    let signal: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let part1 = part1(&signal);

    let part2 = part2(&signal);

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
    assert_eq!(part1, 84487724);
    assert_eq!(part2, 84692524);
}
