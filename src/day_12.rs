use std::collections::HashMap;

use crate::util::{self, split_in_two, OptNumStr};

pub fn solve() {
  
  let mut cache = HashMap::new();
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
    /*
    println!("trying {} {:?}", new_spring_str, new_groups);
    let theos = make_theoreticals(new_spring_str.len(), &new_groups, &mut cache);
    let valid = test_theoreticals(&theos.iter().map(|e| e.chars().collect::<Vec<char>>()).collect(), &new_spring_str.chars().collect());
    println!("{:?}", valid);
    total_valid += valid;
    */
    let combos = get_combos(&new_spring_str, &new_groups, &mut cache);
    total_valid += combos;
  }
  println!("total valid: {}", total_valid);
}



fn get_combos(str:&str, grps:&Vec<usize>, cache:&mut HashMap<String, i64>) -> i64 {
  //println!("combos for {} and {:?}", str, grps);
  if cache.contains_key(&format!("{};{:?}",str, grps)) {
    return cache.get(&format!("{};{:?}",str, grps)).unwrap().to_owned();
  }
  if grps.len() == 0 {
    if !str.contains("#") {
      let answer = 1;
      cache.insert(format!("{};{:?}",str, grps), answer);
      return answer;
    }
    let answer = 0;
    cache.insert(format!("{};{:?}",str, grps), answer);
    return answer;
  } 
  if str.len() == 0 {
    let answer = 0;
    cache.insert(format!("{};{:?}",str, grps), answer);
    return answer;
  }

  let char = str.chars().nth(0).unwrap();
  let grp = grps[0];


  if str.len() < grp {
    let answer = 0;
    cache.insert(format!("{};{:?}",str, grps), answer);
    return answer;
  }

  if char == '.' {
    let answer = get_combos(str.get(1..).unwrap(), grps, cache);
    cache.insert(format!("{};{:?}",str, grps), answer);
    return answer;
  } else if char == '#' {
    let answer = deal_with_hash(grp, str, grps, cache);
    cache.insert(format!("{};{:?}",str, grps), answer);
    return answer;
  } else {
    let answer = get_combos(str.get(1..).unwrap() , grps, cache) + deal_with_hash(grp, str, grps, cache);
    cache.insert(format!("{};{:?}",str, grps), answer);
    return answer;
  }
}

fn deal_with_hash(grp:usize, str:&str, grps:&Vec<usize>, cache:&mut HashMap<String, i64>) -> i64 {
    //calc hash

    // we take grp len substr and check if it can fit in the str
    let substr = str.get(0..grp).unwrap();
    if substr.contains(".") {
      return 0;
    }
    if str.len() == grp {
      if grps.len() == 1 {
        //last grp
        return 1;
      }
      return 0;
    }
    let next_char = str.chars().nth(grp).unwrap();
    if next_char == '?' || next_char == '.' {
      // recurse with substr
      // calc(record[next_group+1:], groups[1:])
      return get_combos(str.split_at(grp + 1).1, &grps.split_at(1).1.to_vec(), cache);
    }
    return 0;
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

fn cache_key(len:usize, grps:&Vec<usize>) -> String {
  return format!("{};{:?}", len, grps);
}

fn make_theoreticals(len:usize, grps:&Vec<usize>,  cache:&mut HashMap<String, Vec<String>>) -> Vec<String> {
  let cache_key = cache_key(len, grps);
  if cache.contains_key(&cache_key) {
    return cache.get(&cache_key).unwrap().to_vec();
  }
  //println!("recursing {}, {:?}", len, grps);

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
          let sub_theos = make_theoreticals(len - j - grp - 1, &sub_grps, cache);
          for su in sub_theos {
            println!("{}{}.{}", prefix, str, su);
            ts.push(format!("{}{}.{}", prefix, str, su));
          }
        }
      }

  }
  cache.insert(cache_key, ts.clone());
  return ts;
}