use std::{collections::HashSet};

use crate::util::{self};

pub fn solve() {
  let rows = util::input_lines_as_strings();
  //plus 2 for sentinel rows
  let grid_width = rows[0].len() + 2;
  let grid_height = rows.len() + 2;
  let mut matrix = vec![vec!['.'; grid_width]; grid_height];

  let mut start_point:(usize,usize) = (0,0);

  for (y,line) in rows.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
          matrix[y+1][x+1] = c;
          if c == 'S' {
            start_point = ((y+1) as usize, (x+1) as usize);
          }
      }
  }

  print_matrix(&matrix);


  //figure out start points
  let start_points = figure_out_connecting_points(start_point, &matrix);
  let mut p1 = start_points[0];
  let mut p2 = start_points[1];
  let mut visited:HashSet<(usize, usize)> = HashSet::from([start_point, p1, p2]);
  println!("startpoint: {:?}", start_point);
  println!("startpoints: {:?} and {:?}", p1,p2);
  let mut steps = 1;
  loop {

    steps += 1;
    //figure out the two directions from p1 and p2
    //we have to keep track of where we've been in visited

    // move p1 first, put in visited, then move p2
    let first_p = get_points(&p1, &visited, &matrix);
    if first_p.is_some() {
      //println!("p1: {:?} ({})", first_p, matrix[first_p.unwrap().0][first_p.unwrap().1]);
      visited.insert(first_p.unwrap());
      p1 = first_p.unwrap();
    } else {
      // we have nowhere to go, we reached the mid point furthest away from the start
      break;
    }
    let second_p = get_points(&p2, &visited, &matrix);
    if second_p.is_some() {
      //println!("p2: {:?} ({})", second_p, matrix[second_p.unwrap().0][second_p.unwrap().1]);
      visited.insert(second_p.unwrap());
      p2 = second_p.unwrap();
    } else {
      // we have nowhere to go, we reached the mid point furthest away from the start
      break;
    }
  }
  println!("{}", steps);

  /*
  .....
  .S-7.
  .|.|.
  .L-J.
  .....
   */

}

fn print_matrix (matrix: &Vec<Vec<char>>) {
  for y in 0..matrix.len() {
    for x in 0..matrix[y].len() {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}

fn figure_out_connecting_points(start_point: (usize, usize), matrix: &Vec<Vec<char>>) -> Vec<(usize,usize)> {

  let s_chars:HashSet<char> = HashSet::from(['|', '7', 'F']);
  let n_chars:HashSet<char> = HashSet::from(['|', 'L', 'J']);
  let w_chars:HashSet<char> = HashSet::from(['-', 'J', '7']);
  let e_chars:HashSet<char> = HashSet::from(['-', 'L', 'F']);
  let y = start_point.0;
  let x = start_point.1;

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
  let mut points = vec![];
  if s_chars.contains(&matrix[y-1][x]) { // N
    points.push((y-1,x));
  }
  if e_chars.contains(&matrix[y][x-1]) { // W
    points.push((y,x-1));
  }
  if w_chars.contains(&matrix[y][x+1]) { // E
    points.push((y,x+1));
  }
  if n_chars.contains(&matrix[y+1][x]) { //S
    points.push((y+1,x));
  }
  return points;
}

fn is_part_of_part(matrix:&Vec<Vec<char>>, x:usize,y:usize) -> bool {
  //the test needs to check all spaces of the matrix around x,y, so that means 8 places:

  return false;
}

//we just return one point here because we don't care about multiple
fn get_points(p: &(usize,usize), visited: &HashSet<(usize,usize)>, matrix: &Vec<Vec<char>>) -> Option<(usize,usize)> {

  //check the squares around the point, only certain pipes fit into certain others

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
  let c = matrix[p.0][p.1];
  let mut points = vec![];
  if c == '|' {
    points = vec![(p.0-1, p.1), (p.0+1, p.1)];
  }
  if c == '-' {
    points = vec![(p.0, p.1-1), (p.0, p.1+1)];
  }
  if c == 'L' {
    points = vec![(p.0-1, p.1), (p.0, p.1+1)];
  }
  if c == 'J' {
    points = vec![(p.0-1, p.1), (p.0, p.1-1)];
  }
  if c == '7' {
    points = vec![(p.0+1, p.1), (p.0, p.1-1)];
  }
  if c == 'F' {
    points = vec![(p.0+1, p.1), (p.0, p.1+1)];
  }

  //none should be dots, but maybe they can break?
  let unvisited:Vec<(usize, usize)> = points.iter().filter(|p| !visited.contains(p)).map(|p| *p).collect();

  return if unvisited.is_empty() { Option::None} else { unvisited.first().copied()}

}