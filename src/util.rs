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

pub fn split_in_two(s:&str, pat:&str) -> (String, String) {
    let split = s.split(pat).collect::<Vec<&str>>();
    return (split[0].to_string(), split[1].to_string())
}

pub fn to_i64_vec(s:String) -> Vec<i64> {
    let split = s.trim().split_whitespace();
    return split.map(|f| f.to_string().to_i64()).collect();
}

pub fn to_i32_vec(s:String) -> Vec<i32> {
    let split = s.trim().split_whitespace();
    return split.map(|f| f.to_string().to_i32()).collect();
}

pub trait OptNumStr {
    fn to_i64(&self) -> i64;
    fn to_i32(&self) -> i32;
}

pub trait UtilStr {

    fn substring(&self, index:usize) -> String;
}

impl UtilStr for &str {
    fn substring(&self, index:usize) -> String {
        return self.chars().skip(index).collect();
    }
}

impl OptNumStr for String {
    fn to_i32(&self) -> i32 { 
        return self.to_string().parse::<i32>().unwrap()
    }

    fn to_i64(&self) -> i64 { 
        return self.to_string().parse::<i64>().unwrap()
    }
}