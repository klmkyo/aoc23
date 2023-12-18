use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct XY {
    x: i64,
    y: i64,
}

// #[derive(Debug, Clone, Copy)]
// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

fn get_grid_dimensions(steps: &Vec<(Direction, i64)>) -> (XY, XY) {
    let mut current_cords = XY { x: 0, y: 0 };

    let mut max_coords = XY { x: 0, y: 0 };
    let mut min_coords = XY { x: 0, y: 0 };

    for step in steps.iter() {
        let (direction, amount) = step;
        match direction {
            Direction::Up => current_cords.y -= amount,
            Direction::Down => current_cords.y += amount,
            Direction::Left => current_cords.x -= amount,
            Direction::Right => current_cords.x += amount,
        }

        if current_cords.x > max_coords.x {
            max_coords.x = current_cords.x;
        } else if current_cords.x < min_coords.x {
            min_coords.x = current_cords.x;
        }

        if current_cords.y > max_coords.y {
            max_coords.y = current_cords.y;
        } else if current_cords.y < min_coords.y {
            min_coords.y = current_cords.y;
        }
    }

    (
        XY {
            x: max_coords.x - min_coords.x,
            y: max_coords.y - min_coords.y,
        },
        min_coords,
    )
}

type Step = (Direction, i64);

// used for differentiating between the different cell types in part_1
// we only want to flood fill the empty cells, and at the end count up
// the borders and inside
// did't have any ideas for better naming
#[derive(Debug, Clone, Copy)]
enum ExtendedCell {
    Border,
    Outside,
    Inside,
}

fn generate_grid(steps: Vec<Step>, dimensions: XY, start: XY) -> Vec<Vec<Option<ExtendedCell>>> {
    let mut grid: Vec<Vec<Option<ExtendedCell>>> = (0..dimensions.y + 1)
        .map(|_| vec![None; dimensions.x as usize + 1])
        .collect();

    let mut current_cords = start;

    let mut queue: VecDeque<Step> = steps.into();

    while let Some(mut step) = queue.pop_front() {
        let (direction, amount) = step;
        for _ in 0..amount {
            match direction {
                Direction::Up => current_cords.y -= 1,
                Direction::Down => current_cords.y += 1,
                Direction::Left => current_cords.x -= 1,
                Direction::Right => current_cords.x += 1,
            }

            grid[current_cords.y as usize][current_cords.x as usize] = Some(ExtendedCell::Border);
            step.1 -= 1;
        }
    }

    grid
}

fn mark_outside(grid: &mut Vec<Vec<Option<ExtendedCell>>>) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // add all the edges of the grid to the queue
    for x in 0..grid[0].len() {
        queue.push_back((0, x));
        queue.push_back((grid.len() - 1, x));
    }

    for y in 0..grid.len() {
        queue.push_back((y, 0));
        queue.push_back((y, grid[0].len() - 1));
    }

    while let Some((y, x)) = queue.pop_front() {
        let current_cell = &mut grid[y][x];

        // we don't want to flood fill the border, nor what we've already filled
        if current_cell.is_some() {
            continue;
        }

        // we want to flood fill the outside
        *current_cell = Some(ExtendedCell::Outside);

        // add the surrounding cells to the queue
        if x > 0 {
            queue.push_back((y, x - 1));
        }

        if x < grid[0].len() - 1 {
            queue.push_back((y, x + 1));
        }

        if y > 0 {
            queue.push_back((y - 1, x));
        }

        if y < grid.len() - 1 {
            queue.push_back((y + 1, x));
        }
    }

    // mark all the inside cells
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x].is_none() {
                grid[y][x] = Some(ExtendedCell::Inside);
            }
        }
    }
}

// same as print_color_grid, but # is border, and . is outside
fn print_extended_cell_grid(grid: &Vec<Vec<Option<ExtendedCell>>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            match cell {
                Some(ExtendedCell::Border) => {
                    print!("#");
                }
                Some(ExtendedCell::Outside) => print!("."),
                Some(ExtendedCell::Inside) => print!(";"),
                None => print!(" "),
            }
        }
        println!();
    }
}

// a far more effectient solution would possibly be to use the corners to calculate the
// area of the shape, for example into rectangles, and then count their areas
// sadly don't have the time to implement that
fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let steps: Vec<Step> = file
        .lines()
        .map(|line| {
            let (_, part2) = line.split_once('#').unwrap();

            let amount = i64::from_str_radix(&part2[0..5], 16).unwrap();

            let direction = match part2.chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Invalid direction"),
            };

            (direction, amount)
        })
        .collect();

    // 2 pass method - first get the max dimensions of the grid, then populate it
    let (hw_dims, min_dims) = get_grid_dimensions(&steps);

    // move the min to 0,0
    let start_dims = XY {
        x: -min_dims.x,
        y: -min_dims.y,
    };

    let mut grid = generate_grid(steps, hw_dims, start_dims);

    // print_color_grid(&grid);

    mark_outside(&mut grid);

    // print_extended_cell_grid(&grid);

    let part_1_solution = grid.iter().fold(0, |row_total, row| {
        let current_row_sum = row.iter().fold(0, |acc: i32, cell| {
            acc + if let Some(ExtendedCell::Inside) | Some(ExtendedCell::Border) = cell {
                1
            } else {
                0
            }
        });

        row_total + current_row_sum
    });

    println!("Part 2 solution: {}", part_1_solution);
}
