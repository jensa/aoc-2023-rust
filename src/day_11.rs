use std::{collections::HashSet};

use crate::util::{self};

pub fn solve() {
  let rows = util::input_lines_as_strings();
  //plus 2 for sentinel rows
  let init_grid_width = rows[0].len() + 2;
  let init_grid_height = rows.len() + 2;
  let mut matrix = vec![vec!['.'; init_grid_width]; init_grid_height];
  print_matrix(&matrix);
  let mut extra_rows = 1;
  for (y,line) in rows.iter().enumerate() {
    if line.chars().all(|c| c == '.') {
      //this row is double
      matrix.insert(y+extra_rows, vec!['.'; init_grid_width]);
      extra_rows += 1;
    } else {
      for (x, c) in line.chars().enumerate() {
          matrix[y+extra_rows][x+1] = c;
      }
    }
  }

  let mut extra_columns = 0;
  let mut finished_matrix = matrix.clone();
  for x in 1..matrix[0].len() -1 {
    let mut empty = true;
    for y in 1..matrix.len() - 1 {
      empty = empty && matrix[y][x] == '.';
    }
    if empty {
      //add a column, this means adding an element to each vec, this gonna be expensive? nah
      println!("Adding empty column at {}", x + extra_columns);
       for y2 in 0..matrix.len() {
        finished_matrix[y2].insert(x + extra_columns, '.');
       }
       extra_columns +=1;
    }
  }

  print_matrix(&finished_matrix);
}

fn print_matrix (matrix: &Vec<Vec<char>>) {
  for y in 1..matrix.len()-1 {
    if y > 9 {
      print!("{}:", y);
    } else {
      print!("{} :", y);
    }
    for x in 1..matrix[y].len()-1 {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}