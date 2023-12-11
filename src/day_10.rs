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


  //figure out start points
  let start_points = figure_out_connecting_points(start_point, &matrix);
  let mut p1: (usize, usize) = start_points[0];
  let p2 = start_points[1];
  let mut visited:HashSet<(usize, usize)> = HashSet::from([start_point, p1]);
  if p1.0 == start_point.0 && p2.0 == start_point.0 {
    matrix[start_point.0][start_point.1] = '-';
  } else if p1.1 == start_point.1 && p2.1 == start_point.1  {
    matrix[start_point.0][start_point.1] = '|';
  } else {

    if p1.0 < start_point.0 {
      //one of the points is above, so it has to be L or J
      if p2.1 < start_point.1 {
        matrix[start_point.0][start_point.1] = 'J';
      } else {
        matrix[start_point.0][start_point.1] = 'L';
      }
    } else if p2.0 < start_point.0 {
      //one of the points is above, so it has to be L or J
      if p1.1 < start_point.1 {
        matrix[start_point.0][start_point.1] = 'J';
      } else {
        matrix[start_point.0][start_point.1] = 'L';
      }
    } else if p1.0 > start_point.0 {
      //one of the points is below, so it has to be F or 7
      if p2.1 < start_point.1 {
        matrix[start_point.0][start_point.1] = '7';
      } else {
        matrix[start_point.0][start_point.1] = 'F';
      }
    }
    else if p2.0 > start_point.0 {
      //one of the points is below, so it has to be F or 7
      if p1.1 < start_point.1 {
        matrix[start_point.0][start_point.1] = '7';
      } else {
        matrix[start_point.0][start_point.1] = 'F';
      }
    }
  }
  loop {
    let new_p = get_points(&p1, &visited, &matrix);
    if new_p.is_some() {
      visited.insert(new_p.unwrap());
      p1 = new_p.unwrap();
    } else {
      break;
    }
  }

  let mut included = 0;
  
  for y in 1..matrix.len() - 1 {
    let mut inside = false;
    let mut last_opened_char = '.';
    for x in 1..matrix[y].len() - 1 {
      if visited.contains(&(y,x)) {
        if matrix[y][x] == '|' {
          inside = !inside;
          last_opened_char = '|';
        } else if matrix[y][x] == 'L' || matrix[y][x] == 'F' {
          inside = !inside;
          last_opened_char = matrix[y][x];
        } else if matrix[y][x] == '7' {
          if last_opened_char != 'L' {
            inside = !inside;
            last_opened_char = '7';
          }
        } else if matrix[y][x] == 'J' {
          if last_opened_char != 'F' {
            inside = !inside;
            last_opened_char = 'J';
          }
        }
      } else if inside {
        included += 1;
      
      }
  }

  }

  print_matrix(&matrix);
  println!("{}", included);

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

  let s_chars:HashSet<char> = HashSet::from(['|', '7', 'F', 'S']);
  let n_chars:HashSet<char> = HashSet::from(['|', 'L', 'J', 'S']);
  let w_chars:HashSet<char> = HashSet::from(['-', 'J', '7', 'S']);
  let e_chars:HashSet<char> = HashSet::from(['-', 'L', 'F', 'S']);
  let y = start_point.0;
  let x = start_point.1;
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

fn get_points(p: &(usize,usize), visited: &HashSet<(usize,usize)>, matrix: &Vec<Vec<char>>) -> Option<(usize,usize)> {
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
  let unvisited:Vec<(usize, usize)> = points.iter().filter(|p| !visited.contains(p)).map(|p| *p).collect();

  return if unvisited.is_empty() { Option::None} else { unvisited.first().copied()}

}