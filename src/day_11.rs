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
  print_matrix(&matrix);
}

fn print_matrix (matrix: &Vec<Vec<char>>) {
  for y in 0..matrix.len() {
    print!("{}:", y);
    for x in 0..matrix[y].len() {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}