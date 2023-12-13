use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct Card {
    cards: [char; 5],
    bid: u64,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn solve() -> SolutionPair {
    let input_string: String = read_to_string("input/input07.txt").unwrap();

    let mut input = parse(&input_string);

    // Your solution here...
    input.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let sol1: u64 = input.iter()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i+1) as u64*card.bid);

    input.sort_by(order_part2);

    let sol2: u64 = input.iter()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i+1) as u64*card.bid);

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse(input_string: &str) -> Vec<Card> {
    let mut input = Vec::new();

    for line in input_string.lines() {
        let mut cards: [char; 5] = [' '; 5];

        for (i, card) in line[0..5].chars().enumerate() {
            cards[i] = card;
        }

        let bid = line[6..].parse().unwrap();

        input.push(Card { cards, bid });
    }

    input
}

fn get_type(cards: [char; 5]) -> Type {
    let mut card_count = [0; 13];

    for card in cards.iter() {
        card_count[card_to_index(*card)] += 1;
    }

    let mut card_count = card_count.to_vec();
    card_count.sort_by(|a, b| b.cmp(a));

    match card_count[0] {
        5 => Type::FiveOfAKind,
        4 => Type::FourOfAKind,
        3 => {
            if card_count[1] == 2 {
                Type::FullHouse
            } else {
                Type::ThreeOfAKind
            }
        }
        2 => {
            if card_count[1] == 2 {
                Type::TwoPairs
            } else {
                Type::OnePair
            }
        }
        _ => Type::HighCard,
    }
}

fn card_to_index(card: char) -> usize {
    match card {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'J' => 3,
        'T' => 4,
        _ => 14 - card.to_digit(10).unwrap() as usize,
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card1 = self.cards;
        let card2 = other.cards;

        let type1 = get_type(card1);
        let type2 = get_type(card2);

        if type1 > type2 {
            Some(std::cmp::Ordering::Greater)
        } else if type1 == type2 {
            let mut aux = Some(std::cmp::Ordering::Equal);
            // Let's hope there is not equal cards
            for (c1, c2) in card1.iter().zip(card2.iter()) {
                match card_to_index(*c1).partial_cmp(&card_to_index(*c2)) {
                    Some(std::cmp::Ordering::Equal) => continue,
                    Some(std::cmp::Ordering::Greater) => {
                        aux = Some(std::cmp::Ordering::Less);
                        break;
                    }
                    Some(std::cmp::Ordering::Less) => {
                        aux = Some(std::cmp::Ordering::Greater);
                        break;
                    }
                    None => continue,
                }
            }
            return aux;
        } else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

fn get_type2(cards: [char; 5]) -> Type {
    let mut card_count: HashMap<char, u64> = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(*card).or_insert(0) += 1;
        acc
    });

    let mut jokers = 0;
    if let Some(num) = card_count.get(&'J') {
            jokers = *num;
            card_count.remove(&'J');
    }

    let mut card_count = card_count.values().copied().collect::<Vec<u64>>();

    card_count.sort_by(|a, b| b.cmp(a));

    if card_count.is_empty() {
        return Type::FiveOfAKind;
    }
    card_count[0] += jokers;

    match card_count[0] {
        5 => Type::FiveOfAKind,
        4 => Type::FourOfAKind,
        3 => {
            if card_count[1] == 2 {
                Type::FullHouse
            } else {
                Type::ThreeOfAKind
            }
        }
        2 => {
            if card_count[1] == 2 {
                Type::TwoPairs
            } else {
                Type::OnePair
            }
        }
        _ => Type::HighCard,
    }
}

fn card_to_index2(card: char) -> usize {
    match card {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'J' => 14,
        'T' => 4,
        _ => 14 - card.to_digit(10).unwrap() as usize,
    }
}

fn order_part2(play1: &Card, play2: &Card) -> std::cmp::Ordering {
    let type1 = get_type2(play1.cards);
    let type2 = get_type2(play2.cards);

    if type1 > type2 {
        std::cmp::Ordering::Greater
    } else if type1 == type2 {
        let mut aux = std::cmp::Ordering::Equal;
        // Let's hope there is not equal cards
        for (c1, c2) in play1.cards.iter().zip(play2.cards.iter()) {
            match card_to_index2(*c1).partial_cmp(&card_to_index2(*c2)) {
                Some(std::cmp::Ordering::Equal) => continue,
                Some(std::cmp::Ordering::Greater) => {
                    aux = std::cmp::Ordering::Less;
                    break;
                }
                Some(std::cmp::Ordering::Less) => {
                    aux = std::cmp::Ordering::Greater;
                    break;
                }
                None => continue,
            }
        }
        return aux;
    } else {
        return std::cmp::Ordering::Less;
    }
}
