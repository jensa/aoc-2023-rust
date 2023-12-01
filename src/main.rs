use std::collections::HashMap;
use std::env;
use std::fs;
use std::i32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vals = contents.split("\n");
    let mut running_total = 0;
    for part in vals {
        let chars: Vec<i32> = get_digits(part.to_string());
        let mut first = chars.first().unwrap().to_string();
        let last = chars.last().unwrap().to_string();
        first.push_str(&last);
        let number = first.parse::<i32>().unwrap();
        running_total += number;
    }
    println!("{running_total}");

}


fn get_digits(part: String) -> Vec<i32>{
    let words: HashMap<&str, i32> = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7) ,("eight", 8) ,("nine", 9)]);
    let mut i =0;
    let mut returnVec: Vec<i32> = Vec::new();
    let chars:Vec<char> = part.chars().collect();
    while i < chars.len() {
        if chars[i].is_numeric() {
            returnVec.push(chars[i].to_string().parse::<i32>().unwrap());
            i +=1;
            continue;
        }
        
        if i + 5 <= chars.len() {
            let five_slice = &part[i..i+5];
            if words.contains_key(five_slice) {
                returnVec.push(*words.get(five_slice).unwrap());
                i+=1;
                continue;
            }
        }
        if i + 4 <= chars.len() {
            let four_slice = &part[i..i+4];
            if words.contains_key(four_slice) {
                returnVec.push(*words.get(four_slice).unwrap());
                i+=1;
                continue;
            }
        }
        if i + 3 <= chars.len() {
            let three_slice = &part[i..i+3];
            if words.contains_key(three_slice) {
                returnVec.push(*words.get(three_slice).unwrap());
            }
        }
        i += 1;
    }
    return returnVec;

}