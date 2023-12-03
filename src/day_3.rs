use crate::util;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let rows = util::input_lines_as_strings();
    //plus 2 for sentinel rows
    let grid_width = rows[0].len() + 2;
    let grid_height = rows.len() + 2;
    let mut matrix = vec![vec!['.'; grid_width]; grid_height];

    for (y,line) in rows.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            matrix[y+1][x+1] = c;
        }
    }
    let mut cur_number:Vec<char> = vec![];
    let mut part_numbers:Vec<String> = vec![];
    let mut cur_has_successful_test = false;
    let mut parts = HashMap::new();
    for y in 0..grid_height {
        for x in 0..grid_width {
            if matrix[y][x].is_numeric() {
                cur_number.push(matrix[y][x]);
                if !cur_has_successful_test {
                    cur_has_successful_test = is_part_of_part(&matrix, x,y)
                }
            } else {
                if !cur_number.is_empty() {
                    if cur_has_successful_test {
                        let num_str:String = cur_number.clone().into_iter().collect();
                        let part_id = format!("{},{} {}", y, x, num_str);
                        for i in x-cur_number.len()..x {
                            parts.insert(format!("{},{}",y,i), part_id.clone());
                        }
                        part_numbers.push(cur_number.into_iter().collect())
                    }
                }
                cur_has_successful_test = false;
                cur_number = vec![];
            }
        }
        cur_has_successful_test = false;
        cur_number = vec![];
    }
    let sum:i32 = part_numbers.iter().map(String::to_i32).sum();
    println!("sum is {sum}");
    let mut ratio_sum = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if matrix[y][x] == '*' {
                let neighbour_parts = get_neighbours(&parts, y,x);
                if neighbour_parts.len() == 2 {
                    let product:i32 = neighbour_parts.iter().map(|p| p.split_whitespace().next_back().unwrap().to_string().to_i32()).product();
                    ratio_sum += product;
                }
            }
        }
    }
    println!("gear ratio is {ratio_sum}")
}

fn get_neighbours(parts:&HashMap<String,String>, y:usize, x:usize) -> Vec<String> {
    let mut neighbours = HashSet::new();
    let keys: Vec<String> = vec![
        format!("{},{}",y-1,x-1), format!("{},{}",y-1,x), format!("{},{}",y-1,x+1), 
        format!("{},{}",y,x-1), format!("{},{}",y,x+1), 
        format!("{},{}",y+1,x-1), format!("{},{}",y+1,x), format!("{},{}",y+1,x+1), 
    ];
    for k in keys {
        if parts.contains_key(&k) {
            neighbours.insert(parts.get(&k).unwrap().to_string());
        }
    }
    return neighbours.into_iter().collect();
}

fn is_part_of_part(matrix:&Vec<Vec<char>>, x:usize,y:usize) -> bool {
    //the test needs to check all spaces of the matrix around x,y, so that means 8 places:
    if sym(matrix[y-1][x-1]) {
        return true;
    }
    if sym(matrix[y-1][x]) {
        return true;
    }
    if sym(matrix[y-1][x+1]) {
        return true;
    }
    if sym(matrix[y][x-1]) {
        return true;
    }
    if sym(matrix[y][x+1]) {
        return true;
    }
    if sym(matrix[y+1][x-1]) {
        return true;
    }
    if sym(matrix[y+1][x]) {
        return true;
    }
    if sym(matrix[y+1][x+1]) {
        return true;
    }
    return false;
}

fn sym (c:char) -> bool {
    return c != '.' && !c.is_numeric()
}

pub trait OptNumStr {
    fn to_i32(&self) -> i32;
}

impl OptNumStr for String {
    fn to_i32(&self) -> i32 { 
        return self.to_string().parse::<i32>().unwrap()
    }
}