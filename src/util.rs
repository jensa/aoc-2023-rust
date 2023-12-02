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