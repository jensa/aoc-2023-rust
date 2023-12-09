use std::collections::HashMap;

use crate::util::{self};

pub fn solve() {

    let mut left:HashMap<String, String> = HashMap::new();
    let mut right:HashMap<String, String> = HashMap::new();
    let lines = util::input_lines_as_strings();

    let path:Vec<char> = lines[0].chars().collect();
    
    let mut positions = vec![];
    for i in 2..lines.len() {
        let line = &lines[i];
        let mut s = line.split(" = ");
        let key = s.next().unwrap().to_string();
        let m = s.next().unwrap().to_string();
        let mut s2 = m.split(", ");
        let left_s:String = s2.next().unwrap().to_string().replace("(", "");
        let right_s:String = s2.next().unwrap().to_string().replace(")", "");

        left.insert(key.clone(), left_s);
        right.insert(key.clone(), right_s);
        if key.ends_with("A") {
            positions.push(key);
        }
    }

    let mut paths = vec![];
    for pos in positions {
        let mut cur_pos = pos;
        let mut i = 0;
        let mut steps = 0;
        loop {
            steps += 1;
            if path[i] == 'L' {
                cur_pos = left.get(&cur_pos).unwrap().to_string();
            } else {
                cur_pos = right.get(&cur_pos).unwrap().to_string();
            }
            if cur_pos.ends_with("Z") {
                paths.push(steps.clone() as i64);
                break;
            }
            i = (i + 1) % path.len();
        }
        
    }

    println!("{:?}", lcm(&paths));

}

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}