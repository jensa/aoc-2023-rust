use std::env;
use std::fs;


pub fn input_lines_as_strings() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vals = contents.split("\n");
    return vals.map(|f| f.to_string()).collect();
}

pub fn to_i64_vec(s:String) -> Vec<i64> {
    let split = s.trim().split_whitespace();
    return split.map(|f| f.to_string().to_i32()).collect();
}

pub trait OptNumStr {
    fn to_i32(&self) -> i64;
}

impl OptNumStr for String {
    fn to_i32(&self) -> i64 { 
        return self.to_string().parse::<i64>().unwrap()
    }
}