use std::collections::HashMap;

use crate::util::{self};

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
  let mut cache = HashMap::new();
  //keep track of our rotation;
  let mut i = 0;
  let factor = 1000000000-1;
  let mut found = false;
  let mut found_cycle = "".to_string();
  let mut found_cycle_at = 0;
  let mut cycle_len = 1;

  //rotate so it starts right (we start with a roll)
  map = rotate_matrix_90_right(&map);
  map = rotate_matrix_90_right(&map);
  map = rotate_matrix_90_right(&map);
  while i < factor {
    map = cycle(map);
    let cache_key = matrix_key(&map);
    if cache_key == found_cycle && cycle_len == 1 {
      cycle_len = i - found_cycle_at;
      let left = factor - i;
      let rest = left % cycle_len;
      let to_add = left - rest;
      i += to_add -1 ;
    }
    if !found && cache.contains_key(&cache_key) {
      found_cycle = cache_key.to_string();
      found_cycle_at = i;
      found = true;
    }
    cache.insert(cache_key, map.clone());
    i += 1;
    
  }
  map = rotate_matrix_90_right(&map); //since we start with a rot
  let mut total_load = 0;
  for y in 0..map.len() {
    for x in 0..map[y].len() {
      if map[y][x] == 'O' {
        total_load += map.len()-y;
      }
    }
  }

  print_matrix(&map);
  println!("total_load: {}", total_load);

}

fn cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut new_map = map.clone();
  for i in 0..4 {
    new_map = rotate_matrix_90_right(&new_map);
    roll_north(&mut new_map);
  }
  return new_map;
}


fn roll_north(map: &mut Vec<Vec<char>>) {

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
}

fn rotate_matrix_90_right(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut new_vec = vec![];
  for x in 0..p[0].len() { //for x
    let mut new_row: Vec<char> = vec![];
    for y in (0..p.len()).rev() { // for y
      let row = &p[y];
      new_row.push(*row.iter().nth(x).unwrap());
    }
    new_vec.push(new_row);
  }
  return new_vec;
}

fn matrix_key(p: &Vec<Vec<char>>) -> String {
  let mut r = vec![];
  for v in p {
    r.push(v.iter().collect::<String>());
  }

  return r.join(" ");
}

fn north_is_empty(y:usize, x:usize, map:&Vec<Vec<char>>) -> bool {
  if y < 1 {
    return false;
  }
  if map[y-1].len() < x + 1 {
    return false;
  }
  return y > 0 && map[y-1][x] == '.'
}

fn print_matrix (matrix: &Vec<Vec<char>>) {
  for y in 0..matrix.len() {
    if y > 98 {
      print!("{}:", y+1);
    } else if y > 8 {
      print!("{} :", y+1);
    } else {
      print!("{}  :", y+1);
    }
    for x in 0..matrix[y].len() {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}