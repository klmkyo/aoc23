use std::{collections::BinaryHeap, cmp::Reverse};

const MAX_STRAIGHT_LINE: i64 = 3;

fn print_grid(grid: &Vec<Vec<i64>>) {
    for row in grid {
        for col in row {
            if *col == 0 {
                print!("\x1b[31m{}\x1b[0m ", col); // Print 0 in red
            } else {
                print!("{} ", col);
            }
        }
        println!();
    }
}

// work of a gippity
fn dijkstra(grid: &Vec<Vec<i64>>) -> (i64, Vec<(usize, usize)>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![i64::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();
    let mut predecessors = vec![vec![None; cols]; rows];

    // Initial state
    dist[0][0] = grid[0][0];
    heap.push(Reverse((grid[0][0], 0, 0, -1, 0))); // (cost, row, col, last_direction, straight_line_count)

    while let Some(Reverse((cost, row, col, last_dir, straight_count))) = heap.pop() {
        if row == rows - 1 && col == cols - 1 {
            let mut path = vec![];
            let mut current = Some((row, col));
            while let Some((r, c)) = current {
                path.push((r, c));
                current = predecessors[r][c];
            }
            path.reverse();
            return (cost, path);
        }

        if cost > dist[row][col] {
            continue;
        }

        let directions = [(0, 1, 0), (0, -1, 1), (1, 0, 2), (-1, 0, 3)];
        for (dr, dc, dir) in directions.iter() {
            let new_row = row as i64 + dr;
            let new_col = col as i64 + dc;
            if new_row >= 0 && new_row < rows as i64 && new_col >= 0 && new_col < cols as i64 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let new_cost = cost + grid[new_row][new_col];
                let new_straight_count = if *dir == last_dir { straight_count + 1 } else { 1 };

                if new_straight_count <= MAX_STRAIGHT_LINE && new_cost < dist[new_row][new_col] {
                    dist[new_row][new_col] = new_cost;
                    predecessors[new_row][new_col] = Some((row, col));
                    heap.push(Reverse((new_cost, new_row, new_col, *dir, new_straight_count)));
                }
            }
        }
    }

    (i64::MAX, Vec::new()) // No path found
}



fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<i64>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    print_grid(&grid);

    let (part_1_result, path) = dijkstra(&grid);

    // display path
    let mut grid_with_path = grid.clone();
    for (row, col) in path {
        grid_with_path[row][col] = 0;
    }
    println!();
    print_grid(&grid_with_path);


    println!("Part 1: {:?}", part_1_result);
}
