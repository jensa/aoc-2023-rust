use crate::util::{self, split_in_two, OptNumStr};

pub fn solve() {
  
  let rows = util::input_lines_as_strings();
  let mut total_valid = 0;
  for row in rows {
    let (spring_str, group_str) = split_in_two(&row, " ");
    let groups = group_str.split(",").map(|e| e.to_string().to_i32() as usize).collect::<Vec<usize>>();

    let new_spring_str = format!("{}?{}?{}?{}?{}", spring_str, spring_str, spring_str, spring_str, spring_str);

    let mut new_groups = vec![];
    for i in 0..5 {
      for g in &groups {
        new_groups.push(*g);
      }
    }
    println!("trying {} {:?}", new_spring_str, new_groups);
    let theos = make_theoreticals(new_spring_str.len(), &new_groups);
    let valid = test_theoreticals(&theos.iter().map(|e| e.chars().collect::<Vec<char>>()).collect(), &new_spring_str.chars().collect());
    println!("{:?}", valid);
    total_valid += valid;
  
  }
  println!("total valid: {}", total_valid);
}

fn test_theoreticals(theos:&Vec<Vec<char>>, str:&Vec<char>) -> i64 {
  let mut valid = 0;
  for theo in theos {
    let mut is_valid = true;
    for i in 0..theo.len() {
      if str[i] == '#' &&  theo[i] != '#' {
        is_valid = false;
        break;
      }
      if theo[i] == '#' && str[i] == '.' {
        is_valid = false;
        break;
      }
    }
    if is_valid {
      valid +=1;
    }
  }
  return valid;
}

fn make_theoreticals(len:usize, grps:&Vec<usize>) -> Vec<String> {

  let mut ts = vec![];
  if grps.len() == 1 {
    let template = (0..len).map(|_e| ".").collect::<String>();
    let grp = grps[0];
    if len < grp {
      return vec![];
    }
    for i in 0..len {
      if i + grp > len {
        break;
      }
      let mut s: String = template.clone();
      let grp_str = (0..grp).map(|_e| '#').collect::<String>();
      s.replace_range(i..i+grp, &grp_str);
      ts.push(s);
    }
  } else {
      let grp = grps[0];
      let str = (0..grp).map(|_e| '#').collect::<String>();
      let sub_grps = grps.iter().skip(1).map(|e| *e).collect();
      for j in 0..len {
        let prefix = (0..j).map(|_e| ".").collect::<String>();
        let sub_len = j + grp +1;
        if sub_len < len {
          let sub_theos = make_theoreticals(len - j - grp - 1, &sub_grps);
          for su in sub_theos {
            ts.push(format!("{}{}.{}", prefix, str, su));
          }
        }
      }

  }

  return ts;
}