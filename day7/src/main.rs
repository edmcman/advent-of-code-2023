use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct Hand {
    cards: [char; 5]
}

impl Hand {
    fn from_string(str: &str) -> Self {
        Hand{cards: str.chars().collect::<Vec<_>>().try_into().unwrap()}
    }
}

type HandBid = (Hand, usize);

const RANK: &'static str = "AKQJT98765432";

#[derive(Debug)]
enum HandType {
    FiveOfAKind(char), // Five of a kind, where all five cards have the same label: AAAAA
    FourOfAKind(char), // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FullHouse(char, char), // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    ThreeOfAKind(char), // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    TwoPair(char, char), // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    OnePair(char), // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    HighCard,      // High card, where all cards' labels are distinct: 23456
}

impl HandType {

    fn get_num(&self) -> char {
        match self {
            Self::FiveOfAKind(c) => '6',
            Self::FourOfAKind(c) => '5',
            Self::FullHouse(c1, _c2) => '4',
            Self::ThreeOfAKind(c) => '3',
            Self::TwoPair(c1, _c2) => '2',
            Self::OnePair(c) => '1',
            Self::HighCard => '0'
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.get_num().cmp(&other.get_num())
    }

    fn of_hand(h: &Hand) -> Self {
        let mut counter = HashMap::new();

        // Populate counter
        h.cards.iter().for_each(|c| *counter.entry(c).or_insert(0) += 1);

        let mut rev_counter = counter.iter().map(|(k, v)| (v, k)).collect::<Vec<(_, _)>>();
        rev_counter.sort_by_key(|(k, _v)| -**k);

        //dbg!(&rev_counter);

        match rev_counter.as_slice() {
            [(5, c)] => Self::FiveOfAKind(***c),
            [(4, c), ..] => Self::FourOfAKind(***c),
            [(3, c1), (2, c2)] => Self::FullHouse(***c1, ***c2),
            [(3, c), ..] => Self::ThreeOfAKind(***c),
            [(2, c1), (2, c2), ..] => Self::TwoPair(***c1, ***c2),
            [(2, c), ..] => Self::OnePair(***c),
            [(1, _), ..] => Self::HighCard,
            _ => panic!("format error"),
        }
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        HandType::of_hand(self)
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => {
                // find first different card
                match self.cards.iter().zip(other.cards.iter()).find(|(c1, c2)| c1 != c2).map(|(c1, c2)| RANK.find(*c2).unwrap().cmp(&RANK.find(*c1).unwrap())) {
                    Some(o) => o,
                    None => Ordering::Equal
                }
            }
            o => o
        }
    }
}

fn parse(stdin: std::io::Stdin) -> Vec<HandBid> {
    stdin
        .lines()
        .map(|line| line.unwrap())
        .map(
            |l| match l.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
                [hand, bid] => (Hand::from_string(hand), bid.parse::<usize>().unwrap()),
                _ => panic!("format error"),
            },
        )
        .collect()
}

fn main() {
    let mut handbids = parse(std::io::stdin());

    /*
    let hand_types = handbids
        .iter()
        .map(|(h, _)| h.get_type())
        .collect::<Vec<_>>();

    println!("hand types: {:?}", hand_types);
    */

    handbids.sort_by(|(h1,_), (h2,_)| h1.cmp(h2));

    println!("hands: {:?}", handbids);

    let handbids = handbids.iter().enumerate().map(|(i, (h, b))| (h, b*(i+1))).collect::<Vec<_>>();

    println!("total winnings: {}", handbids.iter().map(|(_, b)| b).sum::<usize>())
}
