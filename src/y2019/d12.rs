use num::integer::lcm;
use regex::Regex;

use crate::util::Day;

const INPUT: &str = include_str!("input/d12.in");
inventory::submit! {
    Day {year: 2019, day: 12, main}
}

pub fn solve(input: &str) -> (i32, i64) {
    let mut moons: Vec<Vec<i32>> = vec![];
    let mut moonps: Vec<Vec<i32>> = vec![];
    let mut moonvs: Vec<Vec<i32>> = vec![];
    let re = Regex::new(r"^<x=([\-0-9]+), y=([\-0-9]+), z=([\-0-9]+)>").unwrap();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        moons.push(vec![
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ]);
        moonps.push(vec![]);
        moonvs.push(vec![]);
    }

    let mut cycles = vec![];

    for i in 0..3 {
        let mut positions: Vec<i32> = moons.iter().map(|p| p[i]).collect();
        let mut velocities: Vec<i32> = positions.iter().map(|_| 0).collect();

        let p0 = positions.clone();

        let mut step = 0;
        loop {
            for (pos, vel) in positions.iter().zip(velocities.iter_mut()) {
                *vel += positions.iter().map(|&n| n.cmp(pos) as i32).sum::<i32>();
            }

            for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
                *pos += vel;
            }
            step += 1;

            if step == 1000 {
                for (moonp, &pos) in moonps.iter_mut().zip(positions.iter()) {
                    moonp.push(pos);
                }
                for (moonv, &pos) in moonvs.iter_mut().zip(velocities.iter()) {
                    moonv.push(pos);
                }
            }

            if positions == p0 {
                cycles.push((step + 1) as i64);
                break;
            }
        }
    }

    let part1 = moonps
        .iter()
        .zip(moonvs.iter())
        .map(|(ps, vs)| {
            ps.iter().map(|v| v.abs()).sum::<i32>() * vs.iter().map(|v| v.abs()).sum::<i32>()
        })
        .sum();

    let part2 = lcm(cycles[0], lcm(cycles[1], cycles[2]));

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
    assert_eq!(part1, 10944);
    assert_eq!(part2, 484244804958744);
}
