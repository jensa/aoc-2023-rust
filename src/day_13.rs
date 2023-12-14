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

    let vertical = vertical_reflection(&p, None);
    if vertical.is_some() {
      let new_one = generate_new_matrices(&p);
      for n in new_one {
        let vert = vertical_reflection(&n, vertical);
        let hori = horizontal_reflection(&n, None);
        if vert.is_some() && vert.unwrap() != vertical.unwrap() {
          total += vert.unwrap();
          break;
        } else if hori.is_some() {
          total += 100 * hori.unwrap();
          break;
        }
      }

    } else {
      let horizontal = horizontal_reflection(&p, None);
      if horizontal.is_some() {
        let new_one = generate_new_matrices(&p);
        for n in new_one {
          let vert = vertical_reflection(&n, None);
          let hori = horizontal_reflection(&n, horizontal);
          if vert.is_some() {
            total += vert.unwrap();
            break;
          } else if hori.is_some() && hori.unwrap() != horizontal.unwrap() {
            total += 100 * hori.unwrap();
            break;
          }
        }
      }
    }
  }
  println!("total: {}", total);

}

fn generate_new_matrices(p: &Vec<String>) ->  Vec<Vec<String>> {
  let mut ret = vec![];
  for r in 0..p.len() {
    let row = &p[r];
    for (i,c) in row.chars().enumerate() {
      let mut new_row = row.clone().chars().collect::<Vec<char>>();
      new_row[i] = convert(c);
      let mut new_matrix = p.clone();
      new_matrix[r] = new_row.iter().collect::<String>();
      ret.push(new_matrix);
    }
  }
  return ret;
}

fn convert(s:char) -> char {
  return if s == '#' { '.' } else { '#' }
}

fn rotate_matrix(p: &Vec<String>) -> Vec<String> {
  let mut new_vec = vec![];
  for i in 0..p[0].len() {
    let mut new_row = vec![];
    for row in p {
      new_row.push(row.chars().nth(i).unwrap());
    }
    new_vec.push(new_row.iter().collect::<String>());
  }
  return new_vec;
}

fn vertical_reflection(p: &Vec<String>, dont_return_if:Option<usize>) -> Option<usize> {
  let rotated = rotate_matrix(&p);
    return horizontal_reflection(&rotated, dont_return_if);
}

fn horizontal_reflection(p: &Vec<String>, dont_return_if:Option<usize>) -> Option<usize> {
    let mut last_row = "";
    let mut candidate = None;
    for row in 0..p.len() {
      if last_row == p[row] {
        //test reflection here
        let mut up = row-1;
        let mut down = row;
        let mut failed = false;
        loop {
          if up == 0 || down + 1 == p.len() {
            break;
          } 
          up -= 1;
          down += 1;
          if p[up] != p[down] {
            failed = true;
            break;
          }
        }
        if !failed {
          // we _could still find other reflections
          if row > p.len() && dont_return_if.unwrap() != row {
            return Some(row)
          } else {
            if dont_return_if.is_some() && dont_return_if.unwrap() != row {
              candidate = Some(row)
            } else if dont_return_if.is_none() {
              candidate = Some(row)
            }
            
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