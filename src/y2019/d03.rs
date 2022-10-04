use std::{cmp, collections::HashMap, str::FromStr};

use euclid::{Point2D, Vector2D};

use crate::util::Day;

const INPUT: &str = include_str!("input/d03.in");
inventory::submit! {
    Day {year: 2019, day: 3, main}
}

fn parse_line(line: &str) -> Vec<&str> {
    line.trim().split(',').collect()
}

enum M {}
type Point = Point2D<i32, M>;
type Vector = Vector2D<i32, M>;
type Segment = (Point, Point);

fn intersection((v1, v2): &Segment, (h1, h2): &Segment) -> Option<Point> {
    if v1.x >= h1.min(*h2).x
        && v1.x <= h1.max(*h2).x
        && h1.y >= v1.min(*v2).y
        && h1.y <= v1.max(*v2).y
    {
        return Some(Point::new(v1.x, h1.y));
    }
    return None;
}

pub fn solve(input: &str) -> (i32, i32) {
    let vectors: HashMap<char, Vector> = HashMap::from([
        ('L', Vector::new(-1, 0)),
        ('R', Vector::new(1, 0)),
        ('U', Vector::new(0, 1)),
        ('D', Vector::new(0, -1)),
    ]);

    let mut lines = input.trim().lines();

    let mut pos = Point::zero();
    let mut dist = 0;
    let mut hset = HashMap::new();
    let mut vset = HashMap::new();

    let line = parse_line(lines.next().unwrap());
    for segment in line {
        let letter = segment.chars().next().unwrap();
        let d: i32 = String::from_str(segment).unwrap()[1..].parse().unwrap();
        let point = vectors[&letter];
        let new_pos = pos + point * d;

        if point.x == 0 {
            vset.insert((pos, new_pos), dist);
        } else {
            hset.insert((pos, new_pos), dist);
        }

        pos = new_pos;
        dist += d;
    }

    let mut part1 = i32::MAX;
    let mut part2 = i32::MAX;
    pos = Point::zero();
    dist = 0;

    let line = parse_line(lines.next().unwrap());
    for segment in line {
        let letter = segment.chars().next().unwrap();
        let d: i32 = String::from_str(segment).unwrap()[1..].parse().unwrap();
        let point = vectors[&letter];
        let new_pos = pos + point * d;

        if point.x == 0 {
            for (h, d) in &hset {
                if let Some(ip) = intersection(&(pos, new_pos), h) {
                    part1 = cmp::min(part1, ip.x.abs() + ip.y.abs());
                    part2 = cmp::min(part2, dist + (ip - pos).abs().y + d + (ip - h.0).abs().x);
                }
            }
        } else {
            for (v, d) in &vset {
                if let Some(ip) = intersection(v, &(pos, new_pos)) {
                    part1 = cmp::min(part1, ip.x.abs() + ip.y.abs());
                    part2 = cmp::min(part2, dist + (ip - pos).abs().x + d + (ip - v.0).abs().y);
                }
            }
        }

        pos = new_pos;
        dist += d;
    }

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
    assert_eq!(part1, 221);
    assert_eq!(part2, 18542);
}
