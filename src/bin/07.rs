use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands_bids = input.split("\n")
        .map(|h| h.split(" ").collect::<Vec<&str>>())
        .map(|h| {
            let grouped: HashMap<char, u32> = h[0]
                .to_lowercase()
                .chars()
                .into_group_map_by(|&x| x)
                .into_iter()
                .map(|(k, v)| (k, v.len() as u32))
                .collect();

            (h[0], h[1], grouped.clone(), get_hand_strength(&grouped))
        })
        .collect::<Vec<(&str, &str, HashMap<char, u32>, u32)>>();
    hands_bids.sort_by(compare);
    hands_bids.reverse();

    let mut r = 0u32;
    for (rank, hand_bid) in hands_bids.iter().enumerate() {
        r += (rank + 1) as u32 * hand_bid.1.parse::<u32>().unwrap()
    }

    Some(r)
}

fn compare(h1: &(&str, &str, HashMap<char, u32>, u32), h2: &(&str, &str, HashMap<char, u32>, u32)) -> Ordering {
    let card_strengths = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let card_strengths_iter = card_strengths.iter();

    let length_test = h2.3.cmp(&h1.3);
    if length_test == Ordering::Equal {
        let h1_1 = h1.0.chars().next().unwrap();
        let h1_2 = h1.0.chars().collect::<Vec<char>>()[1];
        let h1_3 = h1.0.chars().collect::<Vec<char>>()[2];
        let h1_4 = h1.0.chars().collect::<Vec<char>>()[3];
        let h1_5 = h1.0.chars().collect::<Vec<char>>()[4];
        let h1_1_strength = card_strengths_iter.clone().position(|&r| r == h1_1).unwrap();
        let h1_2_strength = card_strengths_iter.clone().position(|&r| r == h1_2).unwrap();
        let h1_3_strength = card_strengths_iter.clone().position(|&r| r == h1_3).unwrap();
        let h1_4_strength = card_strengths_iter.clone().position(|&r| r == h1_4).unwrap();
        let h1_5_strength = card_strengths_iter.clone().position(|&r| r == h1_5).unwrap();

        let h2_1 = h2.0.chars().next().unwrap();
        let h2_2 = h2.0.chars().collect::<Vec<char>>()[1];
        let h2_3 = h2.0.chars().collect::<Vec<char>>()[2];
        let h2_4 = h2.0.chars().collect::<Vec<char>>()[3];
        let h2_5 = h2.0.chars().collect::<Vec<char>>()[4];
        let h2_1_strength = card_strengths_iter.clone().position(|&r| r == h2_1).unwrap();
        let h2_2_strength = card_strengths_iter.clone().position(|&r| r == h2_2).unwrap();
        let h2_3_strength = card_strengths_iter.clone().position(|&r| r == h2_3).unwrap();
        let h2_4_strength = card_strengths_iter.clone().position(|&r| r == h2_4).unwrap();
        let h2_5_strength = card_strengths_iter.clone().position(|&r| r == h2_5).unwrap();

        return if h1_1 != h2_1 {
            h2_1_strength.cmp(&h1_1_strength)
        } else if h1_2 != h2_2 {
            h2_2_strength.cmp(&h1_2_strength)
        } else if h1_3 != h2_3 {
            h2_3_strength.cmp(&h1_3_strength)
        } else if h1_4 != h2_4 {
            h2_4_strength.cmp(&h1_4_strength)
        } else {
            h2_5_strength.cmp(&h1_5_strength)
        };
    }

    length_test
}

fn get_hand_strength(map: &HashMap<char, u32>) -> u32 {
    let values = map.values().clone().collect::<Vec<&u32>>();

    return if values.contains(&&5u32) {
        7
    } else if values.contains(&&4u32) {
        6
    } else if values.contains(&&3u32) && values.contains(&&2u32) {
        5
    } else if values.contains(&&3u32) {
        4
    } else if values.clone().into_iter().filter(|v| *v == &2u32).collect::<Vec<&u32>>().len() == 2usize {
        3
    } else if values.contains(&&2u32) {
        2
    } else {
        1
    };
}

struct Hand {
    bid: u16,
    values: (u8, u8, u8, u8, u8),
    strength: StrengthType,
}

impl Hand {
    fn new(cards: &str, bids: &str, part2: bool) -> Self {
        let v: Vec<u8> = cards
            .chars()
            .map(|char| {
                if part2 && char == 'J' {
                    0
                } else {
                    CHARS
                        .iter()
                        .rev()
                        .position(|x| x == &char.to_string())
                        .unwrap() as u8
                }
            })
            .collect();

        Self {
            bid: bids.parse::<u16>().unwrap(),
            values: (v[0], v[1], v[2], v[3], v[4]),
            strength: Self::calc_strength(cards, part2),
        }
    }

    fn calc_strength(cards: &str, part2: bool) -> StrengthType {
        let mut dist: HashMap<char, u16> = HashMap::new();
        for char in cards.chars() {
            dist.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut scores: Vec<u16> = dist
            .keys()
            .map(|key| {
                if part2 && key == &'J' {
                    0_u16
                } else {
                    dist.get(key).unwrap().to_owned()
                }
            })
            .collect();
        scores.sort_by(|a, b| b.cmp(a));

        if part2 {
            scores[0] += cards.chars().filter(|a| a == &'J').count() as u16;
        }

        let strength_type = match scores[0] {
            5 => StrengthType::FiveOfAKind,
            4 => StrengthType::FourOfAKind,
            3 => {
                if scores[1] == 2 {
                    StrengthType::FullHouse
                } else {
                    StrengthType::ThreeOfAKind
                }
            }
            2 => {
                if scores[1] == 2 {
                    StrengthType::TwoPair
                } else {
                    StrengthType::OnePair
                }
            }
            _ => StrengthType::HighCard,
        };

        strength_type
    }
}

const CHARS: [&'static str; 14] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1",
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum StrengthType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let s = line.split_once(" ").unwrap();
            Hand::new(s.0, s.1, true)
        })
        .collect();

    hands.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid as usize * (i + 1)));

    Some(winnings as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6640));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6839));
    }
}
