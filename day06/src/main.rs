use std::{fs, cmp::Ordering};

fn main() {
    // part1();
    part2();
}

fn part2() {
    let file = fs::read_to_string("input.txt").unwrap();

    // collect every but first number into a vec
    let time: u64 = file
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance_to_beat = file
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();


    println!("time: {}, distance: {}", time, distance_to_beat);

    // because 0 is possible too
    let hold_possibilities: u64 = time + 1;

    let needed_hold = (0..=hold_possibilities/2)
        .find(|hold| {
            let distance = hold * (time - hold);
            distance > distance_to_beat
        }).unwrap();

    println!("needed_hold: {:?}", needed_hold);

    // // finding first occurence of distance being greater than distance_to_beat,
    // // but using a binary search
    // let mut start = 0;
    // let mut end = hold_possibilities/2;

    // let mut iters = 0;

    // while end - start > 0 {
    //     iters += 1;
    //     let center = (start + end)/2;

    //     let distance = center * (time - center);

    //     println!("start: {}, end: {}, center: {}, distance: {}", start, end, center, distance);

    //     match distance.cmp(&distance_to_beat) {
    //         Ordering::Less => start = center + 1,
    //         Ordering::Equal => {
    //             start = center;
    //             println!("found equal");
    //             break
    //         },
    //         Ordering::Greater => end = center - 1,
    //     }

    //     println!("new start: {}, new end: {}\n", start, end);
    // }

    // let mut needed_hold: u64 = start;

    // // since we are not looking for an exact number, sometimes the binary search
    // // can converge to a number smaller than the actual number we are looking for
    // // so we add 1 if needed
    // if needed_hold * (time - needed_hold) <= distance_to_beat {
    //     println!("needed_hold ({}) is smaller than the actual number ({})", needed_hold, distance_to_beat);
    //     needed_hold += 1;
    // }

    let win_possibilities = hold_possibilities - (needed_hold)*2;
    println!("hold_possibilities: {}, needed_hold: {}, win_possibilities: {}", hold_possibilities, needed_hold, win_possibilities);
}


fn part1() {
    let file = fs::read_to_string("input.txt").unwrap();

    // collect every but first number into a vec
    let times: Vec<u32> =     file.lines().nth(0).unwrap().split_ascii_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = file.lines().nth(1).unwrap().split_ascii_whitespace().skip(1).map(|x| x.parse::<u32>().unwrap()).collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut product = 1_u64;

    for (&time, &distance_to_beat) in times.iter().zip(distances.iter()) {
        // because 0 is possible too
        let hold_possibilities: u32 = time + 1;

        let mut needed_hold = None;

        for hold in 0..=hold_possibilities/2 {
            let time = time;

            let distance = hold * (time - hold);

            println!("{} \t {}", hold, distance);

            if distance > distance_to_beat {
                needed_hold = Some(hold);
                break
            }
        }

        let win_possibilities = hold_possibilities - (needed_hold.unwrap())*2;

        println!("hold_possibilities: {}, needed_hold: {}, win_possibilities: {}", hold_possibilities, needed_hold.unwrap(), win_possibilities);

        product *= win_possibilities as u64;

        println!()
    }

    println!("{}", product);
}
