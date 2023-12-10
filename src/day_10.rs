use std::{collections::HashSet};

use crate::util::{self};

pub fn solve() {
  let rows = util::input_lines_as_strings();
  let visited:HashSet<(i32, i32)> = HashSet::new();
  //plus 2 for sentinel rows
  let grid_width = rows[0].len() + 2;
  let grid_height = rows.len() + 2;
  let mut matrix = vec![vec!['.'; grid_width]; grid_height];

  let mut start_point:(i32,i32) = (0,0);

  for (y,line) in rows.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
          matrix[y+1][x+1] = c;
          if c == 'S' {
            start_point = ((y+1) as i32, (x+1) as i32);
          }
      }
  }

  let mut p1 = start_point.clone();
  let mut p2 = start_point.clone();
  let mut steps = 0;
  loop {
    //figure out the two directions from p1 and p2
    //we have to keep track of where we've been in visited

    // move p1 first, put in visited, then move p2
    let p1 = get_points(p1, &visited, &matrix);
    if p1.is_some() {
      visited.insert(p1.unwrap());
    } else {
      // we have nowhere to go, we reached the mid point furthest away from the start
      break;
    }
    let p2 = get_points(p2, &visited, &matrix);
    if p2.is_some() {
      visited.insert(p2.unwrap());
    } else {
      // we have nowhere to go, we reached the mid point furthest away from the start
      break;
    }
    steps += 1;
  }
  println!("{}", steps);
/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */
  /*
  .....
  .S-7.
  .|.|.
  .L-J.
  .....
   */

}

fn get_points(point: &(i32,i32), visited: &HashSet<(i32,i32)>, matrix: &Vec<Vec<char>>) -> Option<(i32,i32)> {
  return Option::Some((0,0));
}