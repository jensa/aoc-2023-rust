use crate::util;
use std::collections::HashSet;
use std::vec;

pub fn solve() {
    let cards = util::input_lines_as_strings();

    
    let mut total = 0;
    let mut card_structs: Vec<Card> = vec![];
    for card in cards {
        //let mut winners = HashSet::new();
        // Card   1: 13  5 40 15 21 61 74 55 32 56 | 21 57 74 56  7 84 37 47 75 66 68  8 55 22 53 61 40 13 15 41 32 46 95 65  5
        let mut split = card.split(":");
        let id_str = split.next();
        let id:usize = id_str.unwrap().split_whitespace().last().unwrap().to_string().to_i32() as usize;
        let mut nums_str = split.next().unwrap().split("|");
        let win_str = nums_str.next().to_owned().unwrap();
        let our_str = nums_str.next().to_owned().unwrap();

        let win_list:Vec<i32> = win_str.split_whitespace().map(str::to_string).map(|s| s.to_i32()).collect();
        let win_set:HashSet<i32> = HashSet::from_iter(win_list.iter().cloned());

        let our_list:Vec<i32> = our_str.split_whitespace().map(str::to_string).map(|s| s.to_i32()).collect();

        let mut card_point = 0;
        let mut matches = 0;
        for n in our_list {
            if win_set.contains(&n) {
                card_point = if card_point == 0 { 1 } else { card_point *2 };
                matches += 1;
            }
        }
        total += card_point;
        card_structs.push(Card {id, matches})
    }

    let mut total_cards = 0;
    for card in &card_structs {
        total_cards += process_card(&card_structs, &card)
    }

    println!("points: {:?}", total);
    println!("cards: {:?}", total_cards);
    
}

struct Card {
    id:usize,
    matches:usize

}

fn process_card(cards:&Vec<Card>, card:&Card) -> i32 {
    let mut sum_children = 0;
    for i in 0..card.matches {
        sum_children += process_card(cards, cards.get(card.id + i).unwrap());
    }
    return 1 + sum_children;
}

pub trait OptNumStr {
    fn to_i32(&self) -> i32;
}

impl OptNumStr for String {
    fn to_i32(&self) -> i32 { 
        return self.to_string().parse::<i32>().unwrap()
    }
}