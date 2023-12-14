// sphaghetti code warning

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn print_grid_with_o(grid: &Vec<Vec<char>>, o_positions: &Vec<Pos>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if o_positions.contains(&Pos { x, y }) {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

fn hash_grid(grid: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}


enum Direction {
    North,
    South,
    East,
    West,
}

// unoptimized, messy, but working solution
// possible optimization would be grouping the rocks together
fn move_o_in_grid(grid: &mut Vec<Vec<char>>, o_positions: &mut Vec<Pos>, direction: &Direction) {
    // if South/North go thorugh o_positions in normal order
    // if East/West go through o_positions in reverse order

    match direction {
        Direction::North => {
            o_positions.sort_by(|a, b| a.y.cmp(&b.y));

            for o in o_positions.iter_mut() {
                let Pos { x, y } = o;

                // if already at the top, continue
                if *y == 0 {
                    continue;
                }

                // if the immediately next element is not ., continue
                if grid[*y - 1][*x] != '.' {
                    continue;
                }

                // for now lest just assume the best possible case (O being moved
                // to the top), the for loop will correct it if it is not the case
                let mut target_position = Pos { x: *x, y: 0 };
                for yy in (0..*y-1).rev() {
                    let element = grid[yy][*x];

                    if element != '.' {
                        target_position = Pos { x: *x, y: yy+1 };
                        break;
                    }
                }

                // update the grid
                let target_element = &mut grid[target_position.y][target_position.x];
                *target_element = 'O';
                let current_element = &mut grid[*y][*x];
                *current_element = '.';

                // update the o position
                *y = target_position.y;
            }
        },
        Direction::South => {
            o_positions.sort_by(|a, b| a.y.cmp(&b.y));

            for o in o_positions.iter_mut().rev() {
                let Pos { x, y } = o;

                if *y == grid.len() - 1 {
                    continue;
                }

                if grid[*y + 1][*x] != '.' {
                    continue;
                }

                let mut target_position = Pos { x: *x, y: grid.len() - 1 };
                for yy in (*y+1)..grid.len() {
                    let element = grid[yy][*x];

                    if element != '.' {
                        target_position = Pos { x: *x, y: yy-1 };
                        break;
                    }
                }

                let target_element = &mut grid[target_position.y][target_position.x];
                *target_element = 'O';
                let current_element = &mut grid[*y][*x];
                *current_element = '.';

                *y = target_position.y;
            }
        },
        Direction::East => {
            // somehow forgot that for east and west the o_positions should be sorted
            // by their x coordinate, so that the o's are moved from left to right

            o_positions.sort_by(|a, b| a.x.cmp(&b.x));

            for o in o_positions.iter_mut().rev() {
                let Pos { x, y } = o;

                if *x == grid[0].len() - 1 {
                    continue;
                }

                if grid[*y][*x + 1] != '.' {
                    continue;
                }

                let mut target_position = Pos { x: grid[0].len() - 1, y: *y };
                for xx in (*x+1)..grid[0].len() {
                    let element = grid[*y][xx];

                    if element != '.' {
                        target_position = Pos { x: xx-1, y: *y };
                        break;
                    }
                }

                let target_element = &mut grid[target_position.y][target_position.x];
                *target_element = 'O';
                let current_element = &mut grid[*y][*x];
                *current_element = '.';

                *x = target_position.x;
            }
        },
        Direction::West => {
            o_positions.sort_by(|a, b| a.x.cmp(&b.x));

            for o in o_positions.iter_mut() {
                let Pos { x, y } = o;

                if *x == 0 {
                    continue;
                }

                if grid[*y][*x - 1] != '.' {
                    continue;
                }

                let mut target_position = Pos { x: 0, y: *y };
                for xx in (0..*x-1).rev() {
                    let element = grid[*y][xx];

                    if element != '.' {
                        target_position = Pos { x: xx+1, y: *y };
                        break;
                    }
                }

                let target_element = &mut grid[target_position.y][target_position.x];
                *target_element = 'O';
                let current_element = &mut grid[*y][*x];
                *current_element = '.';

                *x = target_position.x;
            }
        },
    }        
}


const ITERATIONS: usize = 1000000000;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let mut o_positions: Vec<Pos> = Vec::new();

    // for every O in grid, add it to the vector for fast lookup
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if *c == 'O' {
                o_positions.push(Pos { x, y });
            }
        }
    }

    
    // the rock tilting has a cycle, which we can use to our advantage
    // keep remembering all the o_positions until we find a cycle
    // then we can skip the millions of iterations and just calculate
    // what iteration of the cycle the last ITERATION will be
    let mut hashes: HashMap<u64, usize> = HashMap::new();

    let mut cycle_start: usize = 0;
    let mut cycle_length: usize = 0;

    for i in 0..ITERATIONS {
        for direction in [Direction::North, Direction::West, Direction::South, Direction::East].iter() {
            move_o_in_grid(&mut grid, &mut o_positions, direction);
        }

        let hash = hash_grid(&grid);
        // match exists or not exists
        match hashes.get(&hash) {
            Some(&prev_i) => {
                cycle_start = prev_i;
                cycle_length = i - prev_i;
                break;
            },
            None => {
                hashes.insert(hash, i);
            },
        }

        // you know its bad when you have to whip out the progress bar
        if i % (ITERATIONS/100) == 0 {
            print_grid(&grid);
            println!("{}% done", i/(ITERATIONS/100));
        }
    }

    if(cycle_length == 0 || cycle_start == 0) {
        panic!("No cycle found");
    }

    // calculate at which position in the cycle the last ITERATION will be
    let last_iteration_in_cycle_index = (ITERATIONS - cycle_start) % cycle_length;

    // since the grid was left in the state of the last iteration in the cycle
    // we can pick up the work from this state
    for _ in 0..last_iteration_in_cycle_index-1 {
        for direction in [Direction::North, Direction::West, Direction::South, Direction::East].iter() {
            move_o_in_grid(&mut grid, &mut o_positions, direction);
        }

        // print_grid(&grid);
        println!("{}", o_positions.iter().map(|pos| grid.len()-pos.y).sum::<usize>());
    }

    let y_len = grid.len();
    let part_2_solution: usize = o_positions.iter().map(|pos| y_len-pos.y).sum();

    println!("Part 2: {}", part_2_solution);
}
