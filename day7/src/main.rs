use std::{fs, cmp::Ordering};

fn main() {
    // part 2 required structure code changes that would break part 1, so the
    // solution can be seen by viewing the git history for this file
    part2();
}

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2 (but not 1)
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Card {
    J, N2, N3, N4, N5, N6, N7, N8, N9, T, Q, K, A
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            _ => panic!("Invalid card"),
        }
    }
}

// display for card
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::T => 'T',
            Card::N9 => '9',
            Card::N8 => '8',
            Card::N7 => '7',
            Card::N6 => '6',
            Card::N5 => '5',
            Card::N4 => '4',
            Card::N3 => '3',
            Card::N2 => '2',
        };

        write!(f, "{}", c)
    }
}


// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Cards([Card; 5]);

impl Cards {
    fn from_string(s: &str) -> Result<Cards, &'static str> {
        if s.len() != 5 {
            return Err("Invalid number of cards");
        }

        let cards: Vec<Card> = s.chars().map(|c| Card::from(c)).collect();

        Ok(Cards(cards.try_into().unwrap()))
    }

    fn get_type(&self) -> HandType {
        let mut counts: Vec<(Card, u8)> = Vec::with_capacity(4);

        let cards = &self.0;

        let mut joker_count = 0;

        // count all the cards
        for card in cards {
            if *card == Card::J {
                joker_count += 1;
                continue;
            }

            let maybe_card_in_counts = counts.iter_mut().find(|c| c.0 == *card);
            if let Some(card_in_counts) = maybe_card_in_counts {
                card_in_counts.1 += 1;
            } else {
                // if there are already 4 different cards, and this is the 5th, then it's a high card
                if counts.len() == 4 {
                    return HandType::HighCard;
                }

                counts.push((card.clone(), 1));
            }
        }

        if counts.len() == 1 {
            return HandType::FiveOfAKind
        }

        let mut counts = counts.into_iter().map(|x| x.1).collect::<Vec<u8>>();
        counts.sort();
        counts.reverse();

        // this means that Cards consisted of only jokers
        if counts.len() == 0 {
            return HandType::FiveOfAKind
        }

        // as part 2 suggests - jokers increase the count of the first card
        counts[0] += joker_count;

        // this is nicer too look at, but in theory slower
        // if a wildcard can be used in here for matching,
        // i guess it would be just as fast, but i'm not aware
        // of such a thing
        // match counts.as_slice() {
        //     [4, 1] => HandType::FourOfAKind,
        //     [3, 2] => HandType::FullHouse,
        //     [3, 1, 1] => HandType::ThreeOfAKind,
        //     [2, 2, 1] => HandType::TwoPair,
        //     [2, 1, 1, 1] => HandType::OnePair,
        //     _ => HandType::HighCard,
        // }

        if counts[0] == 4 {
            return HandType::FourOfAKind
        }

        if counts[0] == 3 && counts[1] == 2 {
            return HandType::FullHouse
        }

        if counts[0] == 3 {
            return HandType::ThreeOfAKind
        }

        if counts[0] == 2 && counts[1] == 2 {
            return HandType::TwoPair
        }

        return HandType::OnePair
    }
}

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        let cards = &self.0;
        let other_cards = &other.0;

        cards.iter().zip(other_cards.iter()).all(|(a, b)| a == b)
    }
}

impl Eq for Cards {}


impl std::fmt::Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cards = &self.0;
        write!(f, "{}{}{}{}{}", cards[0], cards[1], cards[2], cards[3], cards[4])
    }
}

// debug is the same as display
impl std::fmt::Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cards = &self.0;
        write!(f, "{}{}{}{}{}", cards[0], cards[1], cards[2], cards[3], cards[4])
    }
}



#[derive(Debug)]
struct Hand {
    cards: Cards,
    t: HandType
}

impl Hand {
    fn new(cards: Cards) -> Self {
        let t = cards.get_type();
        Hand { cards, t }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // type is most important for comparison, but if
        // they are equal, then we need to compare the cards
        match &other.t.cmp(&self.t) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Equal => (),
        }

        let our_cards = &self.cards.0;
        let their_cards = &other.cards.0;

        for (our_card, their_card) in our_cards.iter().zip(their_cards.iter()) {
            if our_card > their_card {
                return Some(Ordering::Greater);
            } else if our_card < their_card {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn part2() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut hands_with_bids: Vec<(Hand, u32)> = file.lines().map(|line| {
        let (hand, bid) = line.split_at(line.find(' ').unwrap());

        let hand = Cards::from_string(hand).unwrap();
        let bid = bid.trim().parse::<u32>().unwrap();

        (Hand::new(hand), bid)
    }).collect();

    // sort the hands by their ranks
    hands_with_bids.sort_by(|(a, _), (b, _)| a.cmp(b));

    // multiply bid by index + 1
    
    let winnings: u64  = hands_with_bids.iter().enumerate().map(|(i, (_, bid))| {
        (*bid as u64) * (i as u64 + 1)
    }).sum();

    println!("{}", winnings);
}
