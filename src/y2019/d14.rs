use std::{cmp::min, collections::HashMap, fmt::Debug};

use crate::util::Day;

const INPUT: &str = include_str!("input/d14.in");
inventory::submit! {
    Day {year: 2019, day: 14, main}
}

#[derive(Debug)]
struct Compound {
    quantity: i64,
    name: String,
}

impl Compound {
    fn parse(s: &str) -> Compound {
        let mut parts = s.trim().split(' ');
        return Compound {
            quantity: parts.next().unwrap().parse().unwrap(),
            name: String::from(parts.next().unwrap()),
        };
    }
}

#[derive(Debug)]
struct Reaction {
    output: Compound,
    inputs: Vec<Compound>,
}

impl Reaction {
    fn parse(s: &str) -> Reaction {
        let mut parts = s.trim().split("=>");
        let inp = parts
            .next()
            .unwrap()
            .trim()
            .split(", ")
            .map(Compound::parse)
            .collect::<Vec<_>>();
        let out = Compound::parse(parts.next().unwrap());
        return Reaction {
            inputs: inp,
            output: out,
        };
    }
}

fn ores_required(reactions: &HashMap<String, Reaction>, quantity: i64) -> i64 {
    let mut ores = 0;
    let mut leftovers = HashMap::new();
    let mut compounds = HashMap::from([("FUEL", quantity)]);
    while !compounds.is_empty() {
        let (&name, quantity) = compounds.iter().next().unwrap();
        let mut quantity = *quantity;

        if let Some(q) = leftovers.get_mut(name) {
            let sub = min(quantity, *q);
            quantity -= sub;
            *q -= sub;

            if *q == 0 {
                leftovers.remove(name);
            }
        }

        if quantity > 0 {
            let react = reactions.get(name).unwrap();
            let remainder = quantity % react.output.quantity;
            let mut count = quantity / react.output.quantity;

            if remainder != 0 {
                assert!(remainder > 0);
                assert!(react.output.quantity - remainder > 0);
                *leftovers.entry(name).or_default() += react.output.quantity - remainder;
                count += 1;
            }

            for c in react.inputs.iter() {
                let quantity = count * c.quantity;
                if c.name == "ORE" {
                    ores += quantity;
                } else {
                    *compounds.entry(c.name.as_str()).or_default() += quantity;
                }
            }
        }

        compounds.remove(name);
    }

    return ores;
}

pub fn solve(input: &str) -> (i32, i32) {
    let reactions = input
        .trim()
        .lines()
        .map(|l| {
            let reaction = Reaction::parse(l);
            (reaction.output.name.clone(), reaction)
        })
        .collect::<HashMap<_, _>>();

    let part1 = ores_required(&reactions, 1);

    let ores: i64 = 1000000000000;
    let mut fuel = ores / part1;
    let mut step = 100000;

    loop {
        fuel += step;
        let ores_req = ores_required(&reactions, fuel);

        if ores_req > ores {
            fuel -= step;
            if step > 1 {
                step /= 10;
            } else {
                break;
            }
        }
    }

    return (part1 as i32, fuel as i32);
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
    assert_eq!(part1, 220019);
    assert_eq!(part2, 5650230);
}
