use std::collections::HashMap;

use crate::util::{self, split_in_two, OptNumStr};

/*
Determine the ASCII code for the current character of the string.
Increase the current value by the ASCII code you just determined.
Set the current value to itself multiplied by 17.
Set the current value to the remainder of dividing itself by 256.
 */
pub fn solve() {
  let rows = util::input_lines_as_strings();
  let seq = rows[0].split(",").map(|e| e.to_string()).collect::<Vec<String>>();

  let mut boxes:Vec<Vec<lens_box>> = vec![vec![]; 256];
  for s in seq {
    let is_eq = s.contains("=");
    let (label, num) = if is_eq { split_in_two(&s, "=") } else { split_in_two(&s, "-")};
    let lab = label.clone();
    let lens_box = hash(label);
    if !is_eq {
      
      let index = index_of(&lab, &boxes[lens_box]);
      if index.is_some() {
        let _remove = &boxes[lens_box].remove(index.unwrap());
      }
      
    } else {
      let index = index_of(&lab, &boxes[lens_box]);
      if index.is_some() {
        boxes[lens_box][index.unwrap()] = lens_box{label:lab.clone(), focal:num.to_i32() as usize};
      } else {
        boxes[lens_box].push(lens_box{label:lab.clone(), focal:num.to_i32() as usize});
      }
    }
  }

  let mut tot = 0;
  for i in 0..boxes.len() {
    let box_num = i + 1;
    for (j, lens) in (&boxes[i]).iter().enumerate() {
      tot += box_num * (j+1) * lens.focal;
    }
  }

  println!("{}", tot)

}

fn index_of(label:&String, the_box: &Vec<lens_box>) -> Option<usize> {
  for i in 0..the_box.len() {
    if the_box[i].label == *label {
      return Some(i);
    }
  }
  return None
}


#[derive(Debug)]
#[derive(Clone)]
struct lens_box {
  label:String,
  focal: usize
}

fn hash(str:String) -> usize {
  let mut cur_num = 0;
  for c in str.chars() {
    let ascii = c as usize;
    cur_num += ascii;
    cur_num *= 17;
    cur_num = cur_num % 256 
  }
  return cur_num;
}