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

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut grid: Vec<Vec<char>> = file.lines().map(|lines| lines.chars().collect()).collect();

    // if there are rows without any '#', insert another row of just '.'
    let mut i = 0;
    while i < grid.len() {
        if grid[i].iter().all(|c| *c == '.') {
            grid.insert(i, vec!['.'; grid[i].len()]);
            i += 1;
        }
        i += 1;
    }

    // if there are columns without any '#', insert another column of just '.'
    let mut i = 0;
    while i < grid[0].len() {
        if grid.iter().all(|line| line[i] == '.') {
            for line in &mut grid {
                line.insert(i, '.');
            }
            i += 1;
        }
        i += 1;
    }

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


    let mut sum = 0;

    // when iterating through galaxies_coordinates, only compare against items that
    // are next in the vector, to prevent duplicates
    for (i, (curr_x, curr_y)) in galaxies_coordinates.iter().enumerate() {
        for (other_x, other_y) in &galaxies_coordinates[i+1..] {
            let distance = curr_x.diff(other_x) + curr_y.diff(other_y);
            sum += distance;
        }
    }

    println!("sum: {}", sum);
}
