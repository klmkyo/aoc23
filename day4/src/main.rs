use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u32,
    user_picks: Vec<u32>,
    winning_picks: Vec<u32>
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
impl Card {
    fn from_string(s: &str) -> Card {
        let start_i = 5;
        let end_i = s.find(':').unwrap();

        let id = s[start_i..end_i].trim().parse::<u32>().unwrap();

        let convert_to_vec: fn(&str) -> Vec<u32> = |s: &str| {
            s.split_whitespace().map(|num| {
                num.parse::<u32>().unwrap()
            }).collect()
        };

        let (user_picks, winning_picks) = s[end_i+2..].trim().split_once(" | ").unwrap();

        let user_picks = convert_to_vec(user_picks);
        let winning_picks = convert_to_vec(winning_picks);

        Card {
            id,
            user_picks,
            winning_picks
        }
    }

    fn calculate_points(&self) -> u32 {
        let user_picks_set: HashSet<u32> = self.user_picks.iter().cloned().collect();
        let winning_picks_set: HashSet<u32> = self.winning_picks.iter().cloned().collect();

        let won_picks_amount = user_picks_set.intersection(&winning_picks_set).fold(0, |acc, _| acc + 1);

        if won_picks_amount == 0 {
            return 0
        }

        let points = 2_u32.pow(won_picks_amount - 1);

        points
    }
}

fn main() {
    // read file input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    let cards: Vec<Card> = input.lines().map(Card::from_string).collect();

    let sum: u32 = cards.iter().map(|c| c.calculate_points()).sum();

    println!("Sum: {}", sum);
}
