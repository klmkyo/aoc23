// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.

// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Direction {
    x: isize,
    y: isize,
}

fn get_pipe_directions(pipe: char) -> Option<Vec<Direction>> {
    match pipe {
        'S' => Some(vec![Direction { x: 1, y: 0 }, Direction { x: 0, y: 1 }, Direction { x: -1, y: 0 }, Direction { x: 0, y: -1 }]),
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

fn print_grid(grid: &Vec<Vec<Option<u32>>>) {
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
                if let Some(index) = directions.iter().position(|d| d.x == -direction.x && d.y == -direction.y) {
                    // println!(" - - x: {}, y: {} is enterable from the direction: {:?}", other_x, other_y, directions[index]);
                    steps_grid[other_y][other_x] = Some(steps + 1);
                    
                    directions.remove(index);
                    queue.push_back((other_x, other_y));
                }
            }
        }

        // print_grid(&visits_grid);
        // println!("queue: {:?}", queue);
    }

    let most_steps = steps_grid.iter().flatten().flatten().max().unwrap();
    println!("most_steps: {}", most_steps);
}
