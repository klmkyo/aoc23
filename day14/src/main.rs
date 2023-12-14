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



    // for o in o_positions, try to move it up. if there is a # above it, the move
    // cannot be made. if there is a . above it, the move can be made, and the o
    // position is updated.

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


    let y_len = grid.len();
    let part_1_solution: usize = o_positions.iter().map(|pos| y_len-pos.y).sum();

    println!("Part 1: {}", part_1_solution);


    // print_grid_with_o(&grid, &o_positions);
}
