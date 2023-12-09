use std::collections::HashMap;

use crate::util::{self, to_i32_vec, to_i64_vec};

pub fn solve() {

    let mut left:HashMap<String, String> = HashMap::new();
    let mut right:HashMap<String, String> = HashMap::new();
    let lines = util::input_lines_as_strings();
    let histories:Vec<Vec<i64>> = lines.iter().map(|l| to_i64_vec(l.to_string())).collect();

    let mut sum:i64 = 0;
    for h in histories {
        let mut sequences = vec![h.clone()];
        let mut cur_seq = h.clone();
        while !all_zeros(&cur_seq) {
            let next = new_seq(&cur_seq);
            sequences.push(next.clone());
            cur_seq = next;
        }

        let nextVal = get_prev_val(&sequences);
        println!("val: {}", nextVal);
        print_seq_history(&sequences);
        // println!("next val: {}", get_next_val(&sequences));
        sum += nextVal;
    }
    println!("sum: {}", sum)

}

fn get_prev_val(h:&Vec<Vec<i64>>) -> i64 {
    //go through backwards start from next to last 
    let mut res = 0;
    let mut last_val:i64 = 0;
    for i in (0..h.len() - 1).rev() {
        last_val = h[i].first().unwrap() - last_val;
        println!("next value up is {}", last_val);
    }
    return last_val;
}


fn get_next_val(h:&Vec<Vec<i64>>) -> i64 {
    //go through backwards start from next to last 
    let mut res = 0;
    let mut last_val:i64 = 0;
    for i in (0..h.len() - 1).rev() {
        last_val = h[i].last().unwrap() + last_val;
        println!("next value up is {}", last_val);
    }
    return last_val;
}

fn print_seq_history (h:&Vec<Vec<i64>>) {
    for hi in 0..h.len() {
        let ws = (0..hi).map(|_| " ").collect::<String>();
        println!("{} {:?}", ws, h[hi]);
    }
}

fn all_zeros(seq:&Vec<i64>) -> bool {
    let mut ret = true;
    for s in seq {
        ret = ret && *s == 0;
    }
    return ret
}

fn new_seq(seq:&Vec<i64>) -> Vec<i64> {
    let mut s = vec![];
    for i in 0..seq.len()-1 {
        s.push(diff(seq[i], seq[i+1]))
    }
    return s;

}

fn diff(a:i64, b:i64) -> i64 {
    return b - a;
    /*if a > b {
        return a - b;
    } else if a < b {
        return b - a;
    }
    return 0;*/
}