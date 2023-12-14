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

    print_grid_with_o(&grid, &o_positions);


    loop {
        let mut change_happened = false;

        // for o in o_positions, try to move it up. if there is a # above it, the move
        // cannot be made. if there is a . above it, the move can be made, and the o
        // position is updated.

        for o in o_positions.iter_mut() {
            let Pos { x, y } = o;
            if *y == 0 {
                continue;
            }

            let above = &mut grid[*y - 1][*x];
            if *above != '.' {
                continue;
            }

            *above = 'O';
            let current = &mut grid[*y][*x];
            *current = '.';
            
            *y -= 1;
            change_happened = true;
        }


        if !change_happened {
            break;
        }
    }

    let y_len = grid.len();
    let part_1_solution: usize = o_positions.iter().map(|pos| y_len-pos.y).sum();

    println!("Part 1: {}", part_1_solution);


    // print_grid_with_o(&grid, &o_positions);
}
