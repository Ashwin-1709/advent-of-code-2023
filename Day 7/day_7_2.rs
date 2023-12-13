use std::cmp::Ordering;
use std::{collections::HashMap, fs};

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum play {
    five_of_a_kind,
    four_of_a_kind,
    full_house,
    three_of_a_kind,
    two_pair,
    one_pair,
    high_card,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct hand {
    cards: String,
    bid: i32,
    property: Option<play>,
}

fn compute_property(cards: &String) -> play {
    let mut freq: HashMap<char, i32> = HashMap::new();
    for (_index, c) in cards.char_indices() {
        freq.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    let mut four = 0;
    let mut five = 0;
    let mut joker_cnt = 0;
    for (key, value) in freq.iter() {
        if key == &'J' {
            joker_cnt += value;
        }
        match value {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            4 => four += 1,
            5 => five += 1,
            _ => (),
        }
    }

    match freq.len() {
        1 => {
            return play::five_of_a_kind;
        }
        2 => {
            if joker_cnt > 0 {
                return play::five_of_a_kind;
            } else {
                if four > 0 {
                    return play::four_of_a_kind;
                } else {
                    return play::full_house;
                }
            }
        }
        3 => {
            if three > 0 {
                if joker_cnt > 0 {
                    return play::four_of_a_kind;
                } else {
                    return play::three_of_a_kind;
                }
            } else {
                if joker_cnt == 2 {
                    return play::four_of_a_kind;
                } else if joker_cnt == 1 {
                    return play::full_house;
                } else {
                    return play::two_pair;
                }
            }
        }
        4 => {
            if joker_cnt > 0 {
                return play::three_of_a_kind;
            } else {
                return play::one_pair;
            }
        }
        5 => {
            if joker_cnt > 0 {
                return play::one_pair;
            } else {
                return play::high_card;
            }
        }

        (_) => (),
    };
    return play::high_card;
}

impl Ord for hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.property, &other.property) {
            (Some(property1), Some(property2)) => property1.cmp(property2),
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

fn card_value(card: char) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 1,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn compare_cards(card1: &str, card2: &str) -> Ordering {
    let iter1 = card1.chars();
    let iter2 = card2.chars();

    for (c1, c2) in iter1.zip(iter2) {
        let value1 = card_value(c1);
        let value2 = card_value(c2);
        match value1.cmp(&value2) {
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Less,
            Ordering::Less => return Ordering::Greater,
        };
    }

    return Ordering::Equal;
}

fn compare_hands(hand1: &hand, hand2: &hand) -> Ordering {
    if hand1.property == hand2.property {
        return compare_cards(&hand1.cards, &hand2.cards);
    }
    return hand1.cmp(&hand2);
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let games: Vec<&str> = input.lines().collect();
    let mut hands: Vec<hand> = Vec::new();
    for game in games {
        let cur_hand: Vec<&str> = game.split_ascii_whitespace().collect();
        let mut cur = hand {
            cards: cur_hand[0].to_string(),
            bid: cur_hand[1].parse().expect(PARSE_ERROR),
            property: None,
        };

        cur.property = Some(compute_property(&cur.cards));
        hands.push(cur);
    }

    let mut ans: i32 = 0;
    let mut rank: i32 = 1;
    hands.sort_by(|x, y| compare_hands(x, y));
    let ranked: Vec<hand> = hands.into_iter().rev().collect();
    for hand in ranked {
        ans += hand.bid * rank;
        rank += 1;
    }
    println!("{}", ans);
}
