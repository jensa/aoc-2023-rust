use std::collections::HashMap;

use crate::util::{self, split_in_two, OptNumStr};

pub fn solve() {
  let rows = util::input_lines_as_strings();

    //plus 2 for sentinel rows
    let mut map: Vec<Vec<char>> = vec![];

    for (y,line) in rows.iter().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            map[y].push(c);
        }
    }
  //mutate the matrix to roll the rocks, start from the top:
  // first row - nothing can happen
  // second row - move eligble up until it hits a rock or 0

  for y in 0..map.len() {
    for x in 0..map[y].len() {
      if map[y][x] == 'O' {
        // move up until y == 1 or map[y][x] = '#'
        let mut cur_y = y;
        while north_is_empty(cur_y,x, &map) {
          map[cur_y-1][x] = 'O';
          map[cur_y][x] = '.';
          cur_y -= 1;
        }
      }
    }
  }

  let mut load = rows[0].len() - 1;

  let mut total_load = 0;
  for y in 0..map.len() {
    for x in 0..map[y].len() {
      if map[y][x] == 'O' {
        println!("load at {},{} is {}", y,x,load);
        total_load += load;
      }
    }
    load -= 1;
  }

  print_matrix(&map);
  println!("total_load: {}", total_load);

}

fn north_is_empty(y:usize, x:usize, map:&Vec<Vec<char>>) -> bool {
  //can we even check
  if y < 1 {
    return false;
  }
  if map[y-1].len() < x + 1 {
    return false;
  }
  //let c = map[y-1][x];
  //println!("char at [{}][{}]", y, x);
  return y > 0 && map[y-1][x] == '.'
}

fn print_matrix (matrix: &Vec<Vec<char>>) {
  for y in 0..matrix.len() {
    if y > 8 {
      print!("{}:", y+1);
    } else {
      print!("{} :", y+1);
    }
    for x in 0..matrix[y].len() {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}