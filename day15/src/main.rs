use std::collections::HashMap;

fn hash(input: &str) -> u64 {
    let mut curr_val = 0_u64;

    for c in input.chars() {
        curr_val += c as u64;

        curr_val *= 17;

        curr_val %= 256;
    }

    curr_val
}

// Box 0: [rn 1] [cm 2]
// Box 3: [ot 7] [ab 5] [pc 6]
// prints all the boxes from HashMapType in the format above
fn print_boxes(hashmap: &HashMapType) {
    for (k, v) in hashmap.iter() {
        print!("Box {}: ", k);
        for (i, boxx) in v.iter().enumerate() {
            print!("[{} {}] ", boxx.0, boxx.1);
        }
        println!();
    }
}



type HashMapType = HashMap<u64, Vec<(String, u64)>>;

enum Operation {
    Remove,
    Set(u64)
}

const SPECIAL_OPERATIONS: [char; 2] = ['-', '='];

fn parse(input: &str) -> (String, Operation) {
    match input.chars().last().unwrap() {
        '-' => return (input[..input.len()-1].to_string(), Operation::Remove),
        _ => ()
    };

    let (label, set_value) = input.split_once('=').unwrap();

    let operation = Operation::Set(set_value.parse().unwrap());

    return (label.to_string(), operation);
}

fn hash_and_update_lens(input: &str, hashmap: &mut HashMapType) {
    let (label, operation) = parse(input);


    let box_index = hash(&label);
    let boxx = hashmap.entry(box_index).or_insert(Vec::new());

    match operation {
        Operation::Remove => {
            let pos = boxx.iter().position(|(box_label,_)| *box_label == label);
            if let Some(pos) = pos {
                boxx.remove(pos);
            }

            // println!("\n After removing {}:", label);
        },
        Operation::Set(val) => {
            if let Some(box_item) = boxx.iter_mut().find(|(box_label, _)| *box_label == label) {
                *box_item = (label, val);
            } else {
                boxx.push((label, val));
            }
        }
    }

    // print_boxes(hashmap);
}

fn main() {
    // part1();
    part2();
}

fn part2() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let strs: Vec<&str> = file.split(',').collect();

    let mut hashmap: HashMapType = HashMap::new();

    strs.iter().for_each(|s| hash_and_update_lens(s, &mut hashmap));

    let mut result = 0;
    // iterate over k and v of hashmap
    for (k, v) in hashmap.iter() {
        for (i, boxx) in v.iter().enumerate() {
            // println!("result += {} * {} * {}", k+1, i+1, boxx.1);
            result += (k+1) * (i+1) as u64 * boxx.1
        }
    }

    println!("Result: {}", result);
}


fn part1() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let strs: Vec<&str> = file.split(',').collect();

    let hashes: Vec<u64> = strs.iter().map(|s| hash(s)).collect();

    let hashes_sum:u64 = hashes.iter().sum();

    println!("Hashes: {:?}", hashes);
    println!("Hashes sum: {}", hashes_sum);
}
