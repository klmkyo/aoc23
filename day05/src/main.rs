use std::{cmp::Ordering, fs};

const SEEDS_START_INDEX: usize = 7;

#[derive(Debug)]
struct Portal {
    start: i64,
    end: i64,
    shift: i64,
}

impl PartialEq for Portal {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Portal {}

impl PartialOrd for Portal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for Portal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl Portal {
    fn transport(&self, num: i64) -> Option<i64> {
        let range = self.start..=self.end;

        if range.contains(&num) {
            Some(num + self.shift)
        } else {
            None
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let seeds: Vec<i64> = file.lines().next().unwrap()[SEEDS_START_INDEX..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>())
        .flatten()
        .collect();

    println!("Seeds: {:?}", seeds);

    let portals_groups: Vec<Vec<Portal>> = file
        .lines()
        .skip(2)
        .collect::<Vec<&str>>()
        .join("\n")
        .split("\n\n")
        .map(|map| {
            let lines = map.lines().skip(1);

            lines
                .map(|line| {
                    println!("Line: {}", line);

                    let parts: Vec<i64> = line
                        .split_whitespace()
                        .map(|x| x.parse::<i64>())
                        .flatten()
                        .collect();

                    println!("Parts: {:?}", parts);

                    let start = parts[1];
                    let end = parts[1] + parts[2] - 1;
                    let shift = parts[0] - parts[1];

                    println!("start: {}, end: {}, shift: {}", start, end, shift);

                    Portal { start, end, shift }
                })
                .collect::<Vec<Portal>>()
        })
        .collect();

    
    // part 1
    // let mut min = i64::MAX;
    // for seed in seeds {
    //     let mut seed = seed;
    //     for portals_group in &portals_groups {
    //         for portal in portals_group {
    //             if let Some(transported_seed) = portal.transport(seed) {
    //                 seed = transported_seed;
    //                 break;
    //             }
    //         }

    //         println!("Num: {}", seed);
    //     }

    //     if seed < min {
    //         min = seed;
    //     }

    // }

    // part 2
    // TODO very inefficient bruteforce solution used - could be solved by
    // working with seeds as ranges, not as individual numbers
    // don't have time to implement it now :/
    let mut min_start_seed = seeds[0];
    let mut min_result = i64::MAX;

    for pair in seeds.chunks(2) {
        let amount = pair[1];

        let start_seed = pair[0];
        let end_seed: i64 = start_seed + amount;
        
        for seed in start_seed..end_seed {
            let mut seed = seed;
            for portals_group in &portals_groups {
                for portal in portals_group {
                    if let Some(transported_seed) = portal.transport(seed) {
                        seed = transported_seed;
                        break;
                    }
                }
            }

            if seed < min_result {
                min_result = seed;
                min_start_seed = start_seed;
            }
        }
    }

    println!("Min: {}", min_result);
    println!("Min start seed: {}", min_start_seed);
}
