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

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn get_grid_dimensions(steps: &Vec<(Direction, i64, Color)>) -> (XY, XY) {
    let mut current_cords = XY { x: 0, y: 0 };

    let mut max_coords = XY { x: 0, y: 0 };
    let mut min_coords = XY { x: 0, y: 0 };

    for step in steps.iter() {
        let (direction, amount, _) = step;
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

type Step = (Direction, i64, Color);

fn generate_grid(steps: Vec<Step>, dimensions: XY, start: XY) -> Vec<Vec<Option<Color>>> {
    let mut grid: Vec<Vec<Option<Color>>> = (0..dimensions.y + 1)
        .map(|_| vec![None; dimensions.x as usize + 1])
        .collect();

    let mut current_cords = start;

    let mut queue: VecDeque<Step> = steps.into();

    while let Some(mut step) = queue.pop_front() {
        let (direction, amount, color) = step;
        for _ in 0..amount {
            match direction {
                Direction::Up => current_cords.y -= 1,
                Direction::Down => current_cords.y += 1,
                Direction::Left => current_cords.x -= 1,
                Direction::Right => current_cords.x += 1,
            }

            grid[current_cords.y as usize][current_cords.x as usize] = Some(color);
            step.1 -= 1;
        }
    }

    grid
}

fn print_color_grid(grid: &Vec<Vec<Option<Color>>>) {
    for row in grid.iter() {
        for color in row.iter() {
            match color {
                Some(color) => print!("\x1b[48;2;{};{};{}m \x1b[0m", color.r, color.g, color.b),
                None => print!(" "),
            }
        }
        println!();
    }
}

// used for differentiating between the different cell types in part_1
// we only want to flood fill the empty cells, and at the end count up
// the borders and inside
// did't have any ideas for better naming
enum ExtendedCell {
    Border(Color),
    Outside,
    Inside,
}

fn mark_outside(grid: Vec<Vec<Option<Color>>>) -> Vec<Vec<Option<ExtendedCell>>> {
    let mut grid: Vec<Vec<Option<ExtendedCell>>> = grid
        .clone()
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|color| color.map(ExtendedCell::Border))
                .collect()
        })
        .collect();

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

    return grid;
}

// same as print_color_grid, but # is border, and . is outside
fn print_extended_cell_grid(grid: &Vec<Vec<Option<ExtendedCell>>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            match cell {
                Some(ExtendedCell::Border(color)) => {
                    print!("\x1b[48;2;{};{};{}m#\x1b[0m", color.r, color.g, color.b)
                }
                Some(ExtendedCell::Outside) => print!("\x1b[48;2;0;0;0m.\x1b[0m"),
                Some(ExtendedCell::Inside) => print!(";"),
                None => print!(" "),
            }
        }
        println!();
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let steps: Vec<Step> = file
        .lines()
        .map(|line| {
            let mut x = line.split_ascii_whitespace();

            let direction = match x.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };

            let amount = x.next().unwrap().parse::<i64>().unwrap();

            let color_hex = x.next().unwrap();
            let color_hex = color_hex[2..color_hex.len() - 1].to_string();
            let color_hex = i32::from_str_radix(&color_hex, 16).unwrap();

            let color = Color {
                r: ((color_hex >> 16) & 0xFF) as u8,
                g: ((color_hex >> 8) & 0xFF) as u8,
                b: (color_hex & 0xFF) as u8,
            };

            (direction, amount, color)
        })
        .collect();

    // 2 pass method - first get the max dimensions of the grid, then populate it
    let (hw_dims, min_dims) = get_grid_dimensions(&steps);

    // move the min to 0,0
    let start_dims = XY {
        x: -min_dims.x,
        y: -min_dims.y,
    };

    let grid = generate_grid(steps, hw_dims, start_dims);

    // print_color_grid(&grid);

    let grid = mark_outside(grid);

    print_extended_cell_grid(&grid);

    let part_1_solution = grid.iter().fold(0, |row_total, row| {
        let current_row_sum = row.iter().fold(0, |acc: i32, cell| {
            acc + if let Some(ExtendedCell::Inside) | Some(ExtendedCell::Border(_)) = cell {
                1
            } else {
                0
            }
        });

        row_total + current_row_sum
    });

    println!("Part 1 solution: {}", part_1_solution);
}
