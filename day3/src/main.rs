use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut grid: Vec<Vec<char>> = read_lines("input.txt").unwrap().flatten().map(|l| l.chars().collect()).collect();

    let x_size = grid[0].len();
    let y_size = grid.len();

    let mut sum = 0;

    for y in 0..y_size - 1 {
        for x in 0..x_size - 1 {
            let c = grid[y][x];
            let is_special = !c.is_numeric() && c != '.';

            if !is_special {
                continue
            }

            let offsets: [(isize, isize); 9] = [
                (-1, -1), (0, -1), (1, -1),
                (-1, 0), (0, 0), (1, 0),
                (-1, 1), (0, 1), (1, 1)
            ];

            offsets.iter().for_each(|&(ox, oy)| {
                if let (Some(nx), Some(ny)) = (x.checked_add_signed(ox), y.checked_add_signed(oy)) {
                    if let Some(number) = get_number(&mut grid, nx, ny) {
                        sum += number;
                    }
                }
            });
        
        }
    }

    println!("{}", sum);
}


// Given a grid and x,y coordinates, where possibly a digit could be, return the
// whole number constructed from the digits to the left or right of the given
// coordinates.
// 
// Update: since the one number can be surrounded by multiple other numbers,
// this function has been modified to remove the used digits (by setting to '.')
fn get_number(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> Option<u32> {
    // first check if the given coordinates are a digit
    let c = grid[y][x];
    if !c.is_numeric() {
        return None
    }

    // get rightmost digit
    let mut x = x;
    let max_x = grid[0].len() - 1;
    while x < max_x && grid[y][x + 1].is_numeric() {
        x += 1;
    }

    let mut magnitude: u32 = 1;
    let mut number: u32 = 0;

    while let Some(digit) = grid[y][x].to_digit(10) {
        number += digit * magnitude;
        grid[y][x] = '.';

        if x == 0 {
            break
        }

        x -= 1;
        magnitude *= 10;
    }
    
    Some(number)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}