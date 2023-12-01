use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt").unwrap().flatten().collect::<Vec<String>>();

    println!("sum of words: {}", sum_words(&lines));
    println!("sum of basic: {}", sum_basic(&lines));
}


const NUMBERS_LENGTH: usize = 9;
const NUMBER_STRINGS: [&str; NUMBERS_LENGTH] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_first_number (line: &str) -> Option<usize> {
    let mut word_progresses = [0 as usize; NUMBERS_LENGTH];
    

    for char in line.chars() {
        if char.is_numeric() {
            return Some(char.to_digit(10).unwrap() as usize);
        }

        for (w_index, word) in NUMBER_STRINGS.iter().enumerate() {
            let word_length = word.len();

            let current_word_progress = word_progresses[w_index];
            let mut current_char = word.chars().nth(current_word_progress).unwrap();

            if char != current_char {                 
                let first_char = word.chars().nth(0).unwrap();
                current_char = first_char;

                word_progresses[w_index] = 0;
            }

            if char == current_char {
                word_progresses[w_index] += 1;

                if word_progresses[w_index] == word_length {
                    let number = w_index + 1;
                    return Some(number);
                }
            }
        }
    }

    return None;
}


fn get_last_number (line: &str) -> Option<usize> {
    let mut word_progresses = [0 as usize; NUMBERS_LENGTH];

    for char in line.chars().rev() {
        if char.is_numeric() {
            return Some(char.to_digit(10).unwrap() as usize);
        }

        for (w_index, word) in NUMBER_STRINGS.iter().enumerate() {
            let word_length = word.len();

            let current_word_progress = word_progresses[w_index];
            let mut current_char = word.chars().nth(word_length - current_word_progress - 1).unwrap();

            if char != current_char {                 
                let last_char = word.chars().last().unwrap();
                current_char = last_char;

                word_progresses[w_index] = 0;
            }

            if char == current_char {
                word_progresses[w_index] += 1;

                if word_progresses[w_index] == word_length {
                    let number = w_index + 1;
                    return Some(number);
                }
            } 
        }
    }

    return None;
}


fn sum_words(lines: &[String]) -> usize {
    let sum: usize = lines.iter().map(|line| {
        let first = get_first_number(&line).unwrap();
        let last = get_last_number(&line).unwrap();

        first * 10 + last

    }).sum();

    sum
}

fn sum_basic(lines: &[String]) -> usize {
    let sum: usize = lines.iter().map(|line| {
        // extract the first and last number from the line
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();

        let first_as_num = first.to_digit(10).unwrap() as usize;
        let last_as_num = last.to_digit(10).unwrap() as usize;

        first_as_num * 10 + last_as_num
    }).sum();

    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}