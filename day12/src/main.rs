// bad code incoming

use memoize::memoize;

#[memoize]
fn get_possibilities(input_slice: Vec<char>, remaining_slice: Vec<u64>) -> u64 {
    let first_num: u64 = *remaining_slice.first().unwrap();

    if remaining_slice.len() == 1 {
        let remaining_space = input_slice.len();

        let mut total = 0;

        for i in 0..remaining_space {
            let s: String = ".".repeat(i as usize).to_owned() + &"#".repeat(first_num as usize);


            if fits(
                s.chars().into_iter().collect(),
                input_slice.iter().map(|x| *x).collect(),
            ) {
                // even if it fits, if there are any more # after what we checked, then it doesnt fit
                if input_slice
                    .iter()
                    .skip(i as usize + first_num as usize)
                    .any(|x| *x == '#')
                {
                    continue;
                }

                total += 1;
            }
        }

        return total;
    }

    let space_reserved: u64 =
        remaining_slice.iter().sum::<u64>() + (remaining_slice.len() as u64 - 2);

    let remaining_space = (input_slice.len() as u64).checked_sub(space_reserved).unwrap_or(0);
    
    let mut total = 0;
    for i in 0..remaining_space {
        let s: String = ".".repeat(i as usize).to_owned() + &"#".repeat(first_num as usize) + ".";

        if fits(
            s.chars().into_iter().collect(),
            input_slice.iter().map(|x| *x).collect(),
        ) {
            total += get_possibilities(
                input_slice[(i + first_num + 1) as usize..].to_vec(),
                remaining_slice[1..].to_vec(),
            );
        }
    }

    total
}



fn fits(input: Vec<char>, possibility: Vec<char>) -> bool {
    if input.len() > possibility.len() {
        return false;
    }

    // println!("input: {:?}, possibility: {:?}", input.iter().collect::<String>(), possibility.iter().collect::<String>());

    input.iter().zip(possibility.iter()).all(|(i, p)| {
        if *p == '?' {
            // println!("FITS");
            return true;
        }

        // println!("{}", if i == p { "FITS" } else { "DOES NOT FIT" });
        i == p
    })
}

fn copy_vec_5_times_with_question_mark_inbetween(vec: Vec<char>) -> Vec<char> {
    let mut new_vec = Vec::new();

    for _ in 0..4 {
        new_vec.extend(vec.clone());
        new_vec.push('?');
    }
    new_vec.extend(vec.clone());

    new_vec
}

fn copy_vec_5_times<T: Clone>(vec: Vec<T>) -> Vec<T> {
    let mut new_vec = Vec::new();

    for _ in 0..5 {
        new_vec.extend(vec.clone());
    }

    new_vec
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let elems: Vec<(Vec<char>, Vec<u64>)> = file
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let input = split.next().unwrap().chars().into_iter().collect();
            let numbers = split
                .next()
                .unwrap()
                .split(',')
                .flat_map(|x| x.parse::<u64>())
                .collect();

            (input, numbers)
        })
        .collect();

    // part 1
    let mut total = 0;
    for (input, numbers) in &elems {
        let count = get_possibilities(input.clone(), numbers.clone());
        // println!("{:?} -> {}", input.iter().collect::<String>(), count);
        total += count;
    }

    println!("total: {}", total);

    // part 2
    let mut total: u128 = 0;
    for (input, numbers) in elems {
        println!("{:?} -> {:?}", input.iter().collect::<String>(), numbers);
        let count = get_possibilities(
            copy_vec_5_times_with_question_mark_inbetween(input),
            copy_vec_5_times(numbers),
        );
        total += count as u128;
    }

    println!("total: {}", total);
}

// 1st attempt:
// 1. find first symbols:
//      - if ? see if you can fit in there
//      - if # remove the count of # from first (next) number
// 2. recursively(?), given a limited range of chars,
//    and the numbers, find the next hash positions
//    while recursion shrinks the range of chars
// checks if there are no # in vec after the index specified by start_pos
// fn no_hashes_after(vec: &Vec<char>, start_pos: usize) -> bool {
//     vec.iter().skip(start_pos).all(|c| *c != '#')
// }

// fn get_possibilities(vec: &Vec<char>, start_pos: usize, mut remaining: Vec<u64>, count: &mut u64) {
//     if (start_pos > vec.len() - 1) {
//         println!("returning because start_pos: {} > vec.len() - 1: {}", start_pos, vec.len() - 1);
//         return;
//     }

//     println!(
//         "\nget_possibilities() start_pos: {}, count: {}, remaining: {:?}",
//         start_pos, count, remaining
//     );

//     if remaining.is_empty() {
//         println!("remaining is empty, returning");
//         if no_hashes_after(vec, start_pos) {
//             println!(" +++1 count + 1; {}", start_pos);
//             *count += 1;
//         }
//         return;
//     }

//     println!("vec: {:?}", &vec[start_pos..].iter().collect::<String>());

//     // go through the vector. if its ., ignore it
//     // if its # or ?, deduct 1 from first remaining (or remove it if 0)
//     // if its ?, also run get_possibilities on (start+1..)
//     for (i, c) in vec.iter().skip(start_pos).enumerate() {
//         println!(
//             "{}: {}, count: {}, remaining: {:?}",
//             start_pos + i,
//             c,
//             count,
//             remaining
//         );

//         if *c == '.' {
//             println!("encountered ., continuing");
//             continue;
//         }

//         *remaining.first_mut().unwrap() -= 1;

//         if *remaining.first().unwrap() == 0 {
//             println!(" - remaining is 0, removing");
//             remaining.remove(0);

//             // if the next symbol is #, then this path is wrong
//             if let Some('#') = vec.get(start_pos + i + 1) {
//                 println!(" - next symbol is #, returning");
//                 continue;
//             }

//             if remaining.is_empty() && start_pos < vec.len() - 1 {
//                 println!("remaining is empty, returning");
//                 if no_hashes_after(vec, start_pos + i) {
//                     println!(" +++2 count + 1; {}", start_pos + i);
//                     *count += 1;
//                 }
//                 return;
//             }

//             // since there needs to be a gap between the numbers,

//             get_possibilities(vec, start_pos + i + 2, remaining.clone(), count);
//         } else if *c == '?' {
//             println!(" - encountered ?, remaining: {:?}", remaining);

//             get_possibilities(vec, start_pos + i + 1, remaining.clone(), count);
//         }
//     }
// }
