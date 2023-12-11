use crate::util::{self};

pub fn solve() {
  let factor:i64 = 999999;
  let rows = util::input_lines_as_strings();
  //plus 2 for sentinel rows
  let init_grid_width = rows[0].len() + 2;
  let init_grid_height = rows.len() + 2;
  let mut matrix: Vec<Vec<char>> = vec![vec!['.'; init_grid_width]; init_grid_height];
  let mut galaxy_coords:Vec<(i64,i64)> = vec![];
  let mut extra_rows = 1;
  for (y,line) in rows.iter().enumerate() {
    if line.chars().all(|c: char| c == '.') {
      extra_rows += factor;
    } else {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          galaxy_coords.push(((y as i64 + extra_rows) as i64, (x+1) as i64));
        }
        matrix[y+1][x+1] = c;
      }
    }
  }

  let mut extra_columns = 0;
  for x in 1..matrix[0].len() -1 {
    let mut empty: bool = true;
    for y in 1..matrix.len() - 1 {
      empty = empty && matrix[y][x] == '.';
    }
    if empty {
      for c in 0..galaxy_coords.len() {
        if galaxy_coords[c].1 > (x as i64) + extra_columns {
          galaxy_coords[c].1 = galaxy_coords[c].1 + factor;
        }
      }
      extra_columns += factor;
    }
  }

  let mut sum: i64 = 0;
  for i in 0..galaxy_coords.len() {
    for j in i..galaxy_coords.len() {
      sum += dist(galaxy_coords[i], galaxy_coords[j]);
    }
  }

  println!("{:?}", sum);

}

fn dist(i: (i64, i64), j: (i64, i64)) -> i64 {
    return (j.1-i.1).abs() + (j.0-i.0).abs();
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