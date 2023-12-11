fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

trait Diff {
    fn diff(&self, other: &Self) -> usize;
}

impl Diff for usize {
    fn diff(&self, other: &Self) -> usize {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

const GAP_SIZE: usize = 1000000;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = file.lines().map(|lines| lines.chars().collect()).collect();

    let gap_postitions_rows: Vec<usize> = grid.iter().enumerate().filter_map(|(y, row)| {
        if row.iter().all(|c| *c == '.') {
            return Some(y)
        }

        None
    }).collect();

    let gap_postitions_cols : Vec<usize> = (0..grid[0].len()).filter_map(|x| {
        if grid.iter().all(|row| row[x] == '.') {
            return Some(x)
        }

        None
    }).collect();


    println!("gap cols: {:?}", gap_postitions_cols);
    println!("gap rows: {:?}", gap_postitions_rows);


    let galaxies_coordinates: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == '#' { Some((x, y)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    // let expected_pair_count = galaxies_coordinates * (galaxies_coordinates - 1) / 2;
    // println!("{:?}", galaxies_coordinates);

    let mut sum: usize = 0;

    // when iterating through galaxies_coordinates, only compare against items that
    // are next in the vector, to prevent duplicates
    for (i, (curr_x, curr_y)) in galaxies_coordinates.iter().enumerate() {
        for (other_x, other_y) in &galaxies_coordinates[i+1..] {
            let gaps_encountered_col = gap_postitions_cols.iter().filter(|&pos| pos > curr_x.min(other_x) && pos < curr_x.max(other_x)).count();
            let gaps_encountered_row = gap_postitions_rows.iter().filter(|&pos| pos > curr_y.min(other_y) && pos < curr_y.max(other_y)).count();

            let distance = curr_x.diff(other_x) + curr_y.diff(other_y);
            sum += distance;
            sum += (gaps_encountered_col + gaps_encountered_row) * (GAP_SIZE-1)
        }
    }

    println!("sum: {}", sum);
}
