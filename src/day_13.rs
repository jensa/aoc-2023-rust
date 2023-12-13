use std::collections::HashMap;

use crate::util::{self, split_in_two, OptNumStr};

pub fn solve() {
  let rows = util::input_lines_as_strings();

  let mut patterns = vec![];
  let mut cur_pattern = vec![];
  for row in rows {
    if row.is_empty() {
      patterns.push(cur_pattern.clone());
      cur_pattern = vec![];
    } else {
      cur_pattern.push(row);
    }
  }
  patterns.push(cur_pattern.clone());



  let mut total = 0;
  for p in patterns {
    let vertical = vertical_reflection(&p, false);
    if vertical.is_some() {
      //print_matrix_vertical(&p, vertical.unwrap());
      total += vertical.unwrap();
    } else {
      let horizontal = horizontal_reflection(&p, false);
      if horizontal.is_some() {
        //print_matrix_horizontal(&p, horizontal.unwrap());
        total += 100 * horizontal.unwrap()
      } else {
        print_matrix(&p);
        println!("               ");
        vertical_reflection(&p, true);
        println!("~~~~~~~~~~~~~");
        horizontal_reflection(&p, true);
        println!("--------------")
      }
    }
  }
  println!("total: {}", total);

}

fn vertical_reflection(p: &Vec<String>, print:bool) -> Option<usize> {
  //spin it 90 deg and call horiz
  let mut new_vec = vec![];
  for i in 0..p[0].len() {
    let mut new_row = vec![];
    for row in p {
      new_row.push(row.chars().nth(i).unwrap());
    }
    new_vec.push(new_row.iter().collect::<String>())
  }
    return horizontal_reflection(&new_vec, print);
}

fn horizontal_reflection(p: &Vec<String>, print:bool) -> Option<usize> {
    let mut last_row = "";
    let mut candidate = None;
    for row in 0..p.len() {
      if print {
        println!("{} and {} at {}", last_row, p[row], row);
      }
      if last_row == p[row] {
        //test reflection here
        let mut up = row-1;
        let mut down = row;
        let mut failed = false;
        loop {
          if up == 0 || down + 1 == p.len() {
            if print {
              println!("we broke immedately");
            }
            break;
          } 
          up -= 1;
          down += 1;
          if p[up] != p[down] {
            failed = true;
            break;
          }
        }
        if print {
          println!("{} , {} is where we stopped", up, down);
        }
        if !failed {
          if print{
            println!("well ge got row {}", row);
          }
          // we _could still find other reflections
          if row > p.len() {
            return Some(row)
          } else {
            candidate = Some(row)
          }
        }
      }
      last_row = &p[row];
      
    }
    return candidate;
}

fn print_matrix (matrix: &Vec<String>) {
  for y in 0..matrix.len() {
    println!("{}", matrix[y]);
  }
}


fn print_matrix_vertical (matrix: &Vec<String>, v:usize) {
  let prefix = (0..v-1).map(|e| " ").collect::<String>();
  println!("{}><", prefix);
  for y in 0..matrix.len() {
    println!("{}", matrix[y]);
  }
  println!("{}><", prefix);
}

fn print_matrix_horizontal (matrix: &Vec<String>, v:usize) {
  for y in 0..matrix.len() {
    if y == v-1 {
      print!("v ")
    } else if y == v{
      print!("^ ")
    } else {
      print!("  ")
    }
    println!("{}", matrix[y]);
  }
}