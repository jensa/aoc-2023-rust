use std::{collections::{HashSet, HashMap}, thread, time};

use crate::util::{self, split_in_two, OptNumStr};

pub fn solve() {
  let rows = util::input_lines_as_strings();
  let init_grid_width = rows[0].len() + 2;
  let init_grid_height = rows.len() + 2;
  let mut matrix: Vec<Vec<char>> = vec![vec!['.'; init_grid_width]; init_grid_height];
  for (y,line) in rows.iter().enumerate() {
    for (x, c) in line.chars().enumerate() {
      matrix[y+1][x+1] = c;
    }  
  }
  let mut start_poses = vec![];


  
  //NW corner
  start_poses.push(Beam {direction:Direction::EAST, coord: (1,0), moving:true});
  start_poses.push(Beam {direction:Direction::SOUTH, coord: (0,1), moving:true});
  for x in 2..init_grid_width - 2 {
    start_poses.push(Beam {direction:Direction::SOUTH, coord: (0,x), moving:true});
  }
  //NE corner
  start_poses.push(Beam {direction:Direction::WEST, coord: (1,init_grid_width - 1), moving:true});
  start_poses.push(Beam {direction:Direction::SOUTH, coord: (0,init_grid_width - 2), moving:true});

  for y in 2..init_grid_height - 2 {
    start_poses.push(Beam {direction:Direction::WEST, coord: (y, init_grid_width-1), moving:true})
  }

  //SE corner
  start_poses.push(Beam {direction:Direction::WEST, coord: (init_grid_height-2,init_grid_width - 1), moving:true});
  start_poses.push(Beam {direction:Direction::NORTH, coord: (init_grid_height-1,init_grid_width - 2), moving:true});
  for x in (2..init_grid_width - 2).rev() {
    start_poses.push(Beam {direction:Direction::NORTH, coord: (init_grid_height -1,x), moving:true})
  }
  //SW corner
  start_poses.push(Beam {direction:Direction::EAST, coord: (init_grid_height-2,0), moving:true});
  start_poses.push(Beam {direction:Direction::NORTH, coord: (init_grid_height-1,1), moving:true});

  for y in (2..init_grid_height - 2).rev() {
    start_poses.push(Beam {direction:Direction::EAST, coord: (y, 0), moving:true});
  }

  let mut start_display: Vec<Vec<char>> = vec![vec!['.'; init_grid_width]; init_grid_height];
  for s in &start_poses {
    start_display[s.coord.0][s.coord.1] = char_for_dir(&s.direction);
  }
  
  print_full_matrix(&start_display);
  let mut max_energy = 0;
  for p in start_poses {
    println!("we're checking {:?}", p);
    let energized = find_for_pos(&matrix, p);
    if energized > max_energy {
      max_energy = energized;
    }
  }
  println!("Energized: {}", max_energy)
}

fn char_for_dir(d:&Direction) -> char {
  return match d {
      Direction::NORTH => '^',
      Direction::EAST => '>',
      Direction::SOUTH => 'v',
      Direction::WEST => '<'
  }
}

fn find_for_pos(matrix: &Vec<Vec<char>>, beam: Beam) -> usize {
  let init_grid_width = matrix[0].len();
  let init_grid_height = matrix.len();
  let mut energy_matrices = HashMap::from([
    (Direction::NORTH, vec![vec!['.'; init_grid_width]; init_grid_height]),
    (Direction::EAST, vec![vec!['.'; init_grid_width]; init_grid_height]),
    (Direction::SOUTH, vec![vec!['.'; init_grid_width]; init_grid_height]),
    (Direction::WEST, vec![vec!['.'; init_grid_width]; init_grid_height]),
    ]);
  let mut curs = vec![beam];

  let mut has_moving = true;
  while has_moving {
    let mut new_beams = vec![];
    for i in 0..curs.len() {
      let mut c = curs[i];
      if !c.moving {
        continue;
      }
      let next_square = index_for_direction(c.coord, &c.direction);
      if next_square.0 < 1 || next_square.0 >= matrix.len() 
      || next_square.1 < 1 
      || next_square.1 >= matrix[0].len() 
      || energy_matrices.get(&c.direction).unwrap()[next_square.0][next_square.1] == '#' // need to check that this dir has been taken
      {
        c.moving = false;
        new_beams.push(c.clone());
        continue;
      }
      let old_direction = c.direction.clone();
      let next_char = matrix[next_square.0][next_square.1];
      match next_char {
        '.' => c.direction = c.direction,
        '/' => c.direction = match c.direction {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH},
        '\\' => c.direction = match c.direction {
          Direction::NORTH => Direction::WEST,
          Direction::EAST => Direction::SOUTH,
          Direction::SOUTH => Direction::EAST,
          Direction::WEST => Direction::NORTH},
        '|' => { match c.direction {
            Direction::EAST | Direction::WEST => {
              new_beams.push(Beam {direction:Direction::NORTH, coord: next_square, moving:true});
              c.direction = Direction::SOUTH
            }
            _ => {} }},
        '-' => { match c.direction {
          Direction::NORTH | Direction::SOUTH => {
            new_beams.push(Beam {direction:Direction::WEST, coord: next_square, moving:true});
            c.direction = Direction::EAST
          }
          _ => {}}},
        _ => {}
      }
      c.coord = next_square;
      new_beams.push(c.clone());
      energy_matrices.get_mut(&old_direction).unwrap()[c.coord.0][c.coord.1] = '#';
    }
    //print_matrices_and_return_energized(&energy_matrices);
    has_moving = new_beams.iter().any(|c| c.moving);
    curs = new_beams;
  }

  let energized = print_matrices_and_return_energized(&energy_matrices);
  let mut energized_total = 0;
  let mut final_m: Vec<Vec<char>> = vec![vec!['.'; init_grid_width]; init_grid_height];

  for y in 1..energized.len() - 1 {
    for x in 1..energized[y].len() - 1 {
      if energized[y][x] != '.' {
        final_m[y][x] = energized[y][x];
        energized_total += 1;
      }
      if matrix[y][x] != '.' {
        final_m[y][x] = matrix[y][x];
      }
    }
    //println!();
  }

  //print_matrix(&final_m);
  return energized_total;
}



fn index_for_direction(c: (usize, usize), d:&Direction) -> (usize, usize) {
  match d {
      Direction::NORTH => return (c.0 - 1, c.1),
      Direction::EAST => return (c.0, c.1 + 1),
      Direction::SOUTH => return (c.0 + 1, c.1),
      Direction::WEST => return (c.0, c.1  - 1)

  }
}

fn print_matrices_and_return_energized (matrices: &HashMap<Direction, Vec<Vec<char>>>) -> Vec<Vec<char>> {
  //clearScreen();
  //println!("-----------------------");
  let north = matrices.get(&Direction::NORTH).unwrap();
  let mut matrix: Vec<Vec<char>> = vec![vec!['.'; north[0].len()]; north.len()];

  mod_matrix(&mut matrix,  matrices.get(&Direction::NORTH).unwrap(), '^');
  mod_matrix(&mut matrix,  matrices.get(&Direction::EAST).unwrap(), '>');
  mod_matrix(&mut matrix,  matrices.get(&Direction::SOUTH).unwrap(), 'v');
  mod_matrix(&mut matrix,  matrices.get(&Direction::WEST).unwrap(), '<');
  for y in 1..matrix.len() - 1 {
    for x in 1..matrix[y].len() - 1 {
      //print!("{}", matrix[y][x]);
      if matrix[y][x] != '.' {
      }
    }
    //println!();
  }
  return matrix;
}

fn mod_matrix(matrix: &mut Vec<Vec<char>>, from:&Vec<Vec<char>>, found:char) {
  for y in 1..from.len() - 1 {
    for x in 1..from[y].len() - 1 {
      if from[y][x] == '#' {
        if matrix[y][x] == '.' {
          matrix[y][x] = found;
        } else {
          if matrix[y][x].is_numeric() {
            matrix[y][x] = char::from_digit(matrix[y][x].to_digit(10).unwrap() + 1, 10).unwrap();
          } else {
            matrix[y][x] = '2';
          }
        }
      }
    }
  }
}


fn print_matrix (matrix: &Vec<Vec<char>>) {
  for y in 1..matrix.len() - 1 {
    for x in 1..matrix[y].len() - 1 {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}

fn print_full_matrix (matrix: &Vec<Vec<char>>) {
  for y in 0..matrix.len() {
    for x in 0..matrix[y].len() {
      print!("{}", matrix[y][x]);
    }
    println!();
  }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum Direction {
  NORTH, EAST, SOUTH, WEST
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Beam {
  direction: Direction,
  coord: (usize, usize),
  moving:bool
}


fn clearScreen() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}