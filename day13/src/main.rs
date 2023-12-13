use std::collections::{HashSet, HashMap};

type Grid = Vec<Vec<char>>;

const VERTICAL_MULTIPLICATION: u32 = 100;

fn main() {
    // part1();
    part2();
}

fn part2() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let grids: Vec<Grid> = file.split("\n\n").map(|grid| grid.lines().map(|line| line.chars().collect()).collect()).collect();

    let mut part_2_result = 0;
    for grid in grids {
        // println!("grid: {:?}", grid);
        
        // check for horizontal reflections
        let mut common_horizontal_reflections: HashMap<u32, u32> = HashMap::new();
        for row in grid.iter() {
            let current_row_reflections = get_possible_reflections(row);
            current_row_reflections.iter().for_each(|&i| {
                *common_horizontal_reflections.entry(i).or_insert(0) += 1;
            });
        }

        // print if there is a value of 1
        // println!("horizontal reflections: {:#?}", common_horizontal_reflections);

        // if there is an element with value of len - 1, then it's the answer
        if let Some((key, _value)) = common_horizontal_reflections.iter().find(|(_, &value)| value == (grid.len() - 1) as u32) {
            // println!("found horizontal reflection: {}", key);
            part_2_result += key;
            continue;
        }

        let columns: Vec<Vec<char>> = (0..grid[0].len()).map(|x| grid.iter().map(|row| row[x]).collect()).collect();
        let mut common_vertical_reflections: HashMap<u32, u32> = HashMap::new();

        for column in columns.iter() {
            let current_column_reflections = get_possible_reflections(column);
            current_column_reflections.iter().for_each(|&i| {
                *common_vertical_reflections.entry(i).or_insert(0) += 1;
            });
        }

        // print if there is a value of 1
        // println!("vertical reflections: {:#?}", common_vertical_reflections);

        // if there is an element with value of len - 1, then it's the answer
        if let Some((key, _value)) = common_vertical_reflections.iter().find(|(_, &value)| value == (grid[0].len() - 1) as u32) {
            // println!("found vertical reflection: {}", key);
            part_2_result += key * VERTICAL_MULTIPLICATION;
            continue;
        }
    }

    println!("part 2 result: {}", part_2_result);
}

fn part1() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let grids: Vec<Grid> = file.split("\n\n").map(|grid| grid.lines().map(|line| line.chars().collect()).collect()).collect();

    let mut part_1_result = 0;
    for grid in grids {
        // println!("grid: {:?}", grid);
        
        // check for horizontal reflections
        let mut common_horizontal_reflections: HashSet<u32> = HashSet::from_iter(get_possible_reflections(&grid[0]));
        for row in grid.iter().skip(1) {
            let current_row_reflections = HashSet::from_iter(get_possible_reflections(row));
            common_horizontal_reflections = common_horizontal_reflections.intersection(&current_row_reflections).map(|x| *x).collect();

            // if there are no common reflections, then there is no point in continuing
            if common_horizontal_reflections.is_empty() {
                break;
            }
        }

        // if there are common horizontal reflections, then there must be no vertical ones
        if !common_horizontal_reflections.is_empty() {
            // println!("horizontal reflections: {:?}", common_horizontal_reflections);

            if cfg!(debug_assertions) {
                // assert that ther is only one common reflection
                assert_eq!(common_horizontal_reflections.len(), 1);
            }

            // there should only ever be one common reflection
            part_1_result += common_horizontal_reflections.iter().next().unwrap();
            continue;
        }

        let columns: Vec<Vec<char>> = (0..grid[0].len()).map(|x| grid.iter().map(|row| row[x]).collect()).collect();
        let mut common_vertical_reflections: HashSet<u32> = HashSet::from_iter(get_possible_reflections(&columns[0]));

        for column in columns.iter().skip(1) {
            let current_column_reflections = HashSet::from_iter(get_possible_reflections(column));
            // println!("current column reflections: {:?}", current_column_reflections);
            common_vertical_reflections = common_vertical_reflections.intersection(&current_column_reflections).map(|x| *x).collect();            
        }

        if cfg!(debug_assertions) {
            // assert that ther is only one common reflection
            assert_eq!(common_vertical_reflections.len(), 1);
        }
        
        // println!("vertical reflections: {:?}", common_vertical_reflections);
        part_1_result += common_vertical_reflections.iter().next().unwrap() * VERTICAL_MULTIPLICATION;
    }

    println!("part 1 result: {}", part_1_result);
}

fn get_possible_reflections(line: &Vec<char>) -> Vec<u32> {
    let mut vec = Vec::new();

    for i in 1..line.len() {
        // get 2 iterators - of all items to the right of the current (i, i+1) - no middle element
        // left iterator goes from i to 0
        // right iterator goes from i+1 to line.len()
        let left_iter = line.iter().take(i).rev();
        let right_iter = line.iter().skip(i);

        let is_reflection = left_iter.zip(right_iter).all(|(l,r)| l == r);

        if is_reflection {
            vec.push(i as u32);
        }
    }

    vec
}
