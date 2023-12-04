use std::{collections::HashSet, cmp::Ordering};

#[derive(Debug)]
struct Card {
    user_picks: Vec<u32>,
    winning_picks: Vec<u32>,
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
impl Card {
    fn from_string(s: &str) -> Card {
        let picks_start = s.find(':').unwrap() + 2;

        let convert_to_vec: fn(&str) -> Vec<u32> = |s: &str| {
            s.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        };

        let (user_picks, winning_picks) = s[picks_start..].trim().split_once(" | ").unwrap();

        let user_picks = convert_to_vec(user_picks);
        let winning_picks = convert_to_vec(winning_picks);

        Card {
            user_picks,
            winning_picks,
        }
    }

    // get won amount by sorting the picks arrays, and marching through them
    // to find common elements
    fn get_won_amount(&self) -> u32 {
        let mut user_picks = self.user_picks.clone();
        let mut winning_picks = self.winning_picks.clone();

        user_picks.sort_unstable();
        winning_picks.sort_unstable();
        
        // iterative approach
        // let mut won_amount = 0;
        // let mut iu = 0;
        // let mut iw = 0;

        // while iu < user_picks.len() && iw < winning_picks.len() {
        //     match user_picks[iu].cmp(&winning_picks[iw]) {
        //         Ordering::Equal => {
        //             won_amount += 1;
        //             iu += 1;
        //             iw += 1;
        //         }
        //         Ordering::Greater => {
        //             iw += 1;
        //         }
        //         Ordering::Less => {
        //             iu += 1;
        //         }
        //     }
        // }

        // slightly more functional approach
        let mut won_amount = 0;
        let mut user_pick_iter = user_picks.iter();
        let mut winning_pick_iter = winning_picks.iter();

        let mut user_pick = user_pick_iter.next();
        let mut winning_pick = winning_pick_iter.next();

        while let (Some(user_pick_val), Some(winning_pick_val)) = (user_pick, winning_pick) {
            match user_pick_val.cmp(winning_pick_val) {
                Ordering::Equal => {
                    won_amount += 1;
                    user_pick = user_pick_iter.next();
                    winning_pick = winning_pick_iter.next();
                }
                Ordering::Greater => {
                    winning_pick = winning_pick_iter.next();
                }
                Ordering::Less => {
                    user_pick = user_pick_iter.next();
                }
            }
        }

        

        won_amount
    }

    fn get_points_from_won_amount(won_amount: u32) -> u32 {
        if won_amount == 0 {
            return 0;
        }

        let points = 2_u32.pow(won_amount - 1);

        points
    }

    fn get_points(&self) -> u32 {
        Card::get_points_from_won_amount(self.get_won_amount())
    }
}

fn main() {
    // read file input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    let cards: Vec<Card> = input.lines().map(Card::from_string).collect();

    // part 1 solution:
    let sum: u32 = cards.iter().map(|c| c.get_points()).sum();
    println!("Sum: {}", sum);

    // part 2 solution:
    let mut total_cards = cards.len() as u32;
    let mut copies: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let won_amount: u32 = card.get_won_amount();
        let start_i = i + 1;
        // does not exceed the length of the cards vector
        let end_i = i + won_amount as usize;

        for pos in start_i..=end_i {
            copies[pos] += copies[i];
            total_cards += copies[i];
        }
    }

    println!("Total cards: {}", total_cards);
}
