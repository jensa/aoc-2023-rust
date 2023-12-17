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