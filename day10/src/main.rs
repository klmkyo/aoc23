// easily the worst (and slowest) code I've ever written
// part 2 is not completed - my answer is short by 4
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Direction {
    x: isize,
    y: isize,
}

fn get_pipe_directions(pipe: char) -> Option<Vec<Direction>> {
    match pipe {
        'S' => Some(vec![
            Direction { x: 1, y: 0 },
            Direction { x: 0, y: 1 },
            Direction { x: -1, y: 0 },
            Direction { x: 0, y: -1 },
        ]),
        '|' => Some(vec![Direction { x: 0, y: 1 }, Direction { x: 0, y: -1 }]),
        '-' => Some(vec![Direction { x: 1, y: 0 }, Direction { x: -1, y: 0 }]),
        'L' => Some(vec![Direction { x: 0, y: -1 }, Direction { x: 1, y: 0 }]),
        'J' => Some(vec![Direction { x: 0, y: -1 }, Direction { x: -1, y: 0 }]),
        '7' => Some(vec![Direction { x: 0, y: 1 }, Direction { x: -1, y: 0 }]),
        'F' => Some(vec![Direction { x: 0, y: 1 }, Direction { x: 1, y: 0 }]),
        '.' => None,
        _ => panic!("Unknown pipe: {}", pipe),
    }
}

fn print_steps_grid(grid: &Vec<Vec<Option<u32>>>) {
    for line in grid {
        for elem in line {
            match elem {
                Some(val) => print!("{:3} ", val),
                None => print!("  . "),
            }
        }
        println!();
    }
}

fn print_visits_grid(grid: &Vec<Vec<bool>>) {
    for line in grid {
        for elem in line {
            if *elem {
                print!(" x ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let grid_x = file.lines().nth(0).unwrap().len();
    let grid_y = file.lines().count();

    let start_position: (usize, usize) = file
        .lines()
        .enumerate()
        .find_map(|(y, line)| line.chars().position(|c| c == 'S').map(|x| (x, y)))
        .unwrap();
    // println!("start_position: {:?}", start_position);

    let mut steps_grid: Vec<Vec<Option<u32>>> = (0..grid_y).map(|_| vec![None; grid_x]).collect();
    steps_grid[start_position.1][start_position.0] = Some(0);

    let mut directions_grid: Vec<Vec<Option<Vec<Direction>>>> = file
        .lines()
        .map(|line| line.chars().map(get_pipe_directions).collect())
        .collect();

    let directions_grid_og = directions_grid.clone();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back(start_position);
    while let Some((x, y)) = queue.pop_front() {
        let steps: u32 = steps_grid[y][x].unwrap();

        // println!("\nAt the position: ({}, {}) with {} steps", x, y, steps);
        if let Some(directions) = directions_grid[y][x].clone() {
            for direction in directions {
                let other_x: usize = x.wrapping_add(direction.x as usize);
                let other_y = y.wrapping_add(direction.y as usize);

                // check for overflows
                if other_x >= grid_x || other_y >= grid_y {
                    continue;
                }

                // println!(" - elem: ({}, {})", other_x, other_y);
                let other_directions = &mut directions_grid[other_y][other_x];

                if other_directions.is_none() {
                    continue;
                }

                let directions = other_directions.as_mut().unwrap();
                // println!(" - - approaching from: {:?}", direction);
                // println!(" - - directions: {:?}", directions);
                if let Some(index) = directions
                    .iter()
                    .position(|d| d.x == -direction.x && d.y == -direction.y)
                {
                    // println!(" - - x: {}, y: {} is enterable from the direction: {:?}", other_x, other_y, directions[index]);
                    steps_grid[other_y][other_x] = Some(steps + 1);

                    directions.remove(index);
                    queue.push_back((other_x, other_y));
                }
            }
        }

        // print_steps_grid(&steps_grid);
        // println!("queue: {:?}", queue);
    }

    // print_steps_grid(&steps_grid);

    let most_steps = steps_grid.iter().flatten().flatten().max().unwrap();
    println!("most_steps: {}", most_steps);

    let mut grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    // part 2

    // remove all the pipes that connect to nothing or other pipes that won't connect,
    // because these pipes will count as elements enclosed by loop
    let mut old_grid: Vec<Vec<char>> = grid.clone();
    loop {
        for y in 0..grid_y {
            for x in 0..grid_x {
                // get directions of current element
                let directions = &directions_grid_og[y][x];

                if directions.is_none() {
                    continue;
                }

                let directions = directions.as_ref().unwrap();

                // check if its an S by any chance
                if directions.len() == 4 {
                    continue;
                }

                for direction in directions.clone() {
                    // if any of the directions overflows, set current element to a .
                    let other_x = x.wrapping_add(direction.x as usize);
                    let other_y = y.wrapping_add(direction.y as usize);

                    if other_x >= grid_x || other_y >= grid_y {
                        grid[y][x] = '.';
                        break;
                    }

                    // if the elemeent at the other position is not a pipe, set current element to a .
                    let other_element = grid[other_y][other_x];

                    if other_element == '.' {
                        grid[y][x] = '.';
                        break;
                    }

                    // if its a pipe, check if it has the opposite direction
                    let other_directions = &directions_grid_og[other_y][other_x];

                    if other_directions.is_none() {
                        grid[y][x] = '.';
                        break;
                    }

                    let other_directions = other_directions.as_ref().unwrap();
                    
                    // if none of the directions in other_directions is the opposite of direction, set current element to a .
                    if !other_directions
                        .iter()
                        .any(|d| d.x == -direction.x && d.y == -direction.y)
                    {
                        grid[y][x] = '.';
                        break;
                    }
                }
            }
        }

        if old_grid == grid {
            break;
        }

        old_grid = grid.clone();
    }

    // print the grid
    for line in &grid {
        for elem in line {
            print!("{}", elem);
        }
        println!();
    }

    let mut visits_grid: Vec<Vec<bool>> = vec![vec![false; grid_x * 2]; grid_y * 2];

    // enlarge the grid by 2x to make the gaps between the pipes visible
    for (y, line) in grid.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            let bits: [[bool; 2]; 2] = match pipe {
                '|' => [[true, false], [true, false]],
                '-' => [[false, false], [true, true]],
                'L' => [[true, false], [true, true]],
                'J' => [[true, false], [true, false]],
                '7' => [[false, false], [true, false]],
                'F' => [[false, false], [true, true]],
                '.' => [[false, false], [false, false]],
                'S' => [[true, true], [true, true]],
                _ => panic!("Unknown pipe: {}", pipe),
            };

            for (dy, row) in bits.iter().enumerate() {
                for (dx, bit) in row.iter().enumerate() {
                    visits_grid[y * 2 + dy][x * 2 + dx] = *bit;
                }
            }
        }
    }

    let visits_grid_x = visits_grid[0].len();
    let visits_grid_y = visits_grid.len();

    // fill with borders of grid
    let mut fill_queue: VecDeque<(usize, usize)> = VecDeque::new();
    // top/bottom rows
    for x in 0..visits_grid_x {
        // check if an element at this position already was marked in the steps_grid
        if (steps_grid[0][x / 2].is_none()) {
            fill_queue.push_back((x, 0));
        }
        if (steps_grid[grid_y - 1][x / 2].is_none()) {
            fill_queue.push_back((x, visits_grid_y - 1));
        }
    }
    // left/right columns
    for y in 0..visits_grid_y {
        // check if an element at this position already was marked in the steps_grid
        if (steps_grid[y / 2][0].is_none()) {
            fill_queue.push_back((0, y));
        }
        if (steps_grid[y / 2][grid_x - 1].is_none()) {
            fill_queue.push_back((visits_grid_x - 1, y));
        }
    }

    while let Some((x, y)) = fill_queue.pop_front() {
        const DIRECTIONS_TO_CHECK: [Direction; 4] = [
            Direction { x: 0, y: 1 },
            Direction { x: 1, y: 0 },
            Direction { x: 0, y: -1 },
            Direction { x: -1, y: 0 },
        ];

        // println!("checking: ({}, {})", x, y);
        visits_grid[y][x] = true;
        // print_visits_grid(&visits_grid);
        // println!();

        for direction in DIRECTIONS_TO_CHECK {
            let other_x = x.wrapping_add(direction.x as usize);
            let other_y = y.wrapping_add(direction.y as usize);

            // check for overflows
            if other_x >= visits_grid_x || other_y >= visits_grid_y {
                continue;
            }

            let element_to_check = &mut visits_grid[other_y][other_x];

            if *element_to_check == false {
                fill_queue.push_back((other_x, other_y));
                // println!("added to queue: ({}, {})", other_x, other_y);
            }
        }
    }

    // print_visits_grid(&visits_grid);

    let mut part_2_solution = 0;
    for y in 0..grid_y {
        for x in 0..grid_x {
            let visits_y = y * 2;
            let visits_x = x * 2;

            const DIRS: [[usize; 2]; 4] = [[0, 0], [0, 1], [1, 0], [1, 1]];
            if DIRS
                .iter()
                .all(|[dy, dx]| visits_grid[visits_y + dy][visits_x + dx] == false)
            {
                part_2_solution += 1;
            }
        }
    }

    println!("part_2_solution: {}", part_2_solution);
}
