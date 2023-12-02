use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default, Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn new(red: usize, green: usize, blue: usize) -> Set {
        Set {
            red,
            green,
            blue,
        }
    }

    fn getSetFromString(input: &str) -> Set {
        let unparsed_colors = input.split(", ");

        let mut new_set = Set::default();

        for unparsed_color in unparsed_colors {
            let (amount, color) = unparsed_color
                .split_once(" ")
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b))
                .unwrap();

            match color {
                "red" => new_set.red = amount,
                "green" => new_set.green = amount,
                "blue" => new_set.blue = amount,
                _ => panic!("Unknown color!"),
            }
        }

        return new_set;
    }
}



// a game is an array of sets
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: usize) -> Game {
        Game {
            id,
            sets: Vec::new(),
        }
    }

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn getGameFromLine(line: &str) -> Game {
        // read from 5th character to the :
        let id_start = 5;
        let id_end = line.find(":").unwrap();

        let id = line[id_start..id_end].parse::<usize>().unwrap();

        let unparsed_sets = line[id_end+2..line.len()].split("; ");

        let sets = unparsed_sets.map(Set::getSetFromString).collect();

        Game { id, sets }
    }

    fn getMinCubes(&self) -> Set {
        let min_set = Set::new(usize::MIN, usize::MIN, usize::MIN);

        self.sets.iter().fold(min_set, |prev, curr| {
            Set {
                red: prev.red.max(curr.red),
                green: prev.green.max(curr.green),
                blue: prev.blue.max(curr.blue),
            }
        })
    }
}



const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let games: Vec<Game> = read_lines("input.txt").unwrap().flatten().map(|line| Game::getGameFromLine(&line)).collect();

    let filtered_games = games.iter().filter(|game| {
        game.sets.iter().all(|set| {
            set.red <= MAX_RED && set.green <= MAX_GREEN && set.blue <= MAX_BLUE
        })
    });

    let game_ids_sum: usize = filtered_games.map(|game| game.id).sum();

    let game_powers_sum: usize = games.iter().map(|game| {
        let min = game.getMinCubes();
        println!("Game {} min: {:?}", game.id, min);

        min
    }).map(|set| set.blue * set.green * set.red).sum();

    println!("Game ids sum: {}", game_ids_sum);
    println!("Game powers sum: {}", game_powers_sum);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}