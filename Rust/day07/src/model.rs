use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum HandType {
    None,
    FiveOfKind,  // where all five cards have the same label: AAAAA
    FourOfKind,  // where four cards have the same label and one card has a different label: AA8AA
    FullHouse, // where three cards have the same label, and the remaining two cards share a different label: 23332
    ThreeOfKind, //where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    TwoPair, // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    OnePair, // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    HighCard, // where all cards' labels are distinct: 23456
}

#[derive(Debug, Eq, Clone)]
pub struct Hand {
    pub cards: String,
    pub bid: i32,
    pub hand_type: HandType,
}

impl Hand {
    const VALID_CARDS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
}

impl Hand {
    pub fn cmp_part1(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            // println!("Same hand type: {:?}", self.hand_type);
            let l_cards = self.cards.chars().collect::<Vec<char>>();
            let r_cards = other.cards.chars().collect::<Vec<char>>();
            for i in 0..self.cards.len() {
                if l_cards[i] == r_cards[i] {
                    continue;
                }
                let l_pos = Hand::VALID_CARDS
                    .iter()
                    .position(|&x| x == l_cards[i])
                    .unwrap();
                let r_pos = Hand::VALID_CARDS
                    .iter()
                    .position(|&x| x == r_cards[i])
                    .unwrap();
                // println!("Found positions (lPos/rPos): {} / {}", lPos, rPos);
                if l_pos < r_pos {
                    return Ordering::Less;
                } else if l_pos > r_pos {
                    return Ordering::Greater;
                } else {
                    println!("EERRRROOOORRR");
                    return Ordering::Equal;
                }
            }
            return Ordering::Equal;
        } else {
            return self.hand_type.cmp(&other.hand_type);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Hand {
    pub fn parse(line: &String) -> Hand {
        let tokens = line
            .split_whitespace()
            .map(|t| t.trim())
            .collect::<Vec<&str>>();
        let res = Hand {
            cards: tokens[0].to_string(),
            bid: tokens[1].parse::<i32>().unwrap(),
            hand_type: Hand::determine_type(tokens[0]),
        };

        res
    }

    pub fn determine_type(cards: &str) -> HandType {
        let mut counter = HashMap::new();
        for vc in Hand::VALID_CARDS {
            counter.insert(vc, 0);
        }

        for c in cards.chars() {
            *counter.get_mut(&c).unwrap() += 1;
        }

        let mut stat = counter.values().filter(|&v| *v > 0).collect::<Vec<_>>();
        // println!("Card values: {:?}", counter.values());
        // println!("Card statistics: {:?}", stat);
        stat.sort_by(|a, b| b.cmp(a));
        // println!("Sorted: {:?}", stat);

        match stat.len() {
            1 => HandType::FiveOfKind,
            2 if *stat[0] == 4 => HandType::FourOfKind,
            2 => HandType::FullHouse,
            3 if *stat[0] == 3 => HandType::ThreeOfKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => HandType::None,
        }
    }
}
