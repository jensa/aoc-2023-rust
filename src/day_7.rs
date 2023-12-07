use std::{collections::HashMap, cmp::Ordering};

use crate::util::{self, OptNumStr};

pub fn solve() {

    let card_map:HashMap<char, i32> = HashMap::from([('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)]);
    let hand_strings = util::input_lines_as_strings();

    let mut hands = vec![];
    for hand_s in &hand_strings {
        let mut it = hand_s.split_whitespace();
        let cards_s = it.next().unwrap();
        let bet_s = it.next().unwrap().to_string();
        let cards = cards_s.chars().map(|c| if c.is_numeric() { c.to_string().to_i32() } else { card_map.get(&c).unwrap().clone() } ).collect();
        hands.push(Hand {
            hand_type: get_type(&cards),
            cards: cards,
            bet: bet_s.to_i32(),
        })
    }

    hands.sort();
    let mut winnings = 0;
    for (i,h) in hands.iter().enumerate() {
        let rank = i as i32 + 1;
        winnings += h.bet * rank;
    }

    println!("{}", winnings);


}

fn get_type (cards:&Vec<i32>) -> i32 {
    // group the numbers
    // count groups
    let mut buckets:HashMap<i32, i32> = HashMap::new();
    for i in 1..15 {
        if i != 11 {
            buckets.insert(i, 0);
        }
    }
    for card in cards {
        buckets.insert(*card, buckets.get(card).unwrap() + 1);
    }
    let joker_size = *buckets.get(&1).unwrap();
    buckets.remove(&1);
    let mut groups:Vec<i32> = buckets.iter().filter(|(_,v)| **v > 0).map(|(_,v)| *v).collect();
    if groups.len() == 0 {
        return 7;
    }
    groups.sort();
    let last = groups.last_mut().unwrap();
    *last += joker_size;
    return get_regular_type(&groups);
}

fn get_regular_type (groups: &Vec<i32>) -> i32 {
    if groups.len() == 1 {
        //five of a kind
        return 7;
    } else if groups.len() == 2 {
        if *groups.get(1).unwrap() == 4 {
            //four of a kind
            return 6;
        } else {
            //full house
            return 5;
        }
    } else if groups.len() == 3 {
        if *groups.get(2).unwrap() == 3 {
            //three of a kind
            return  4;
        }
        else {
            // two pair
            return 3;
        }
    } else if groups.len() == 4 {
        //one pair
        return 2;
    } else {
        //high card
        return 1;
    }
}


#[derive(Debug)]
#[derive(Clone)]
struct Hand {
    cards:Vec<i32>,
    bet:i32,
    hand_type: i32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        //just compare vals 
        for i in 0..self.cards.len() {
            if self.cards.get(i).unwrap() > other.cards.get(i).unwrap() {
                return Ordering::Greater;
            } else if self.cards.get(i).unwrap() < other.cards.get(i).unwrap() {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() || self.bet != other.bet || self.hand_type != other.hand_type {
            return false;
        }
        for i in 0..self.cards.len() {
            if self.cards.get(i).unwrap() != other.cards.get(i).unwrap() {
                return false;
            }
        }
        return true
    
    }
}

impl Eq for Hand { }
