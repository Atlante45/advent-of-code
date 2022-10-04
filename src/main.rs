#![feature(min_specialization)]

use std::{collections::HashMap, time::Instant};

use structopt::StructOpt;
use util::Day;

mod util;

pub mod y2019 {
    automod::dir!(pub "src/y2019");
}

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc")]
struct Opt {
    #[structopt(default_value = "2019")]
    year: u32,
    #[structopt(default_value = "17")]
    day: u32,
    #[structopt(short, long)]
    all: bool,
}

fn main() {
    let opt = Opt::from_args();
    let mut days: HashMap<u32, HashMap<u32, fn()>> = HashMap::new();

    for day in inventory::iter::<Day> {
        let day: &Day = &*day;
        days.entry(day.year).or_default().insert(day.day, day.main);
    }

    if opt.all {
        let now = Instant::now();
        let mut year_nums = days.keys().collect::<Vec<&u32>>();
        year_nums.sort();
        for y in year_nums {
            let days_map = days.get(y).unwrap();

            let mut day_nums = days_map.keys().collect::<Vec<&u32>>();
            day_nums.sort();
            for d in day_nums {
                if let Some(main) = days_map.get(d) {
                    let now = Instant::now();
                    main();
                    let elapsed = now.elapsed().as_secs_f32() * 1_000.0;
                    println!("  Ran in {elapsed:.2}")
                }
            }
        }
        println!("Ran in {} ms", now.elapsed().as_secs_f32() * 1000.0);
    } else {
        match days.get(&opt.year) {
            Some(days) => match days.get(&opt.day) {
                Some(main) => main(),
                None => panic!("Unknown day {}", opt.day),
            },
            None => panic!("Unknown year {}", opt.year),
        };
    }
}
