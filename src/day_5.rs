use crate::util;
use std::vec;
use std::ops::Range;
use crate::util::to_i64_vec;

pub fn solve() {
    let rows = util::input_lines_as_strings();

    let seeds_row = rows.get(0).unwrap();
    let seeds_list = seeds_row.split(":").last().unwrap().to_string().clone();
    let seeds = to_i64_vec(seeds_list);
    let mut pair = 0;
    let mut last = 0;
    let mut seed_ranges = vec![];
    for s in &seeds {
        if pair == 0 {
            last = s.clone();
        } else {
            seed_ranges.push(last..last + s);
        }
        pair = (pair + 1) % 2
    }
    println!("{:?}", seed_ranges);

    let mut range_map_maps:Vec<Vec<RangeMap<i64>>> = vec![];
    let mut cur_range_map:Vec<RangeMap<_>> = vec![];
    for row in rows {
        if row.is_empty() {
            if !cur_range_map.is_empty() {
                range_map_maps.push(cur_range_map.to_vec());
                cur_range_map = vec![];
            }
        } else if !row.contains(":") {
            let nums:Vec<i64> = to_i64_vec(row);
            let src_start = *nums.get(1).unwrap();
            let src_end = nums.get(1).unwrap() + nums.get(2).unwrap();

            let out_start = *nums.get(0).unwrap();
            let out_end = nums.get(0).unwrap() + nums.get(2).unwrap();

        
            cur_range_map.push(RangeMap {
                src_range: src_start..src_end,
                out_range: out_start..out_end,
                offset_key: out_start - src_start
            })
        }
    }
    range_map_maps.push(cur_range_map.to_vec());
    let mut locations = vec![];
    for seed in seeds {
        let mut key = seed;
        for range_maps in &range_map_maps {
            for range_map in range_maps {
                if range_map.src_range.contains(&key) {
                    let index: i64 = key - range_map.src_range.start;
                    key = range_map.out_range.start + index;
                    break;
                }
            }
        }
        locations.push(key.clone());
    }

    let mut location_ranges = vec![];
    for range in &seed_ranges {
        let mut sub_ranges: Vec<Range<i64>> = vec![range.clone()];
        for range_maps in &range_map_maps {
            //here we go through the sub ranges from previous loop
            let mut new_ranges:Vec<Range<i64>> = vec![]; 
            for sub_range in &sub_ranges {

                let mut mapped_src_ranges = vec![];
                for range_map in range_maps {
                    //here we build sub ranges for the above loop
                    if range_map.src_range.start > sub_range.end || range_map.src_range.end < sub_range.start {
                        //none of the range bits match
                        continue;
                    }
                    if range_map.src_range.start < sub_range.start {
                        // range starts before sub_range
                        if range_map.src_range.end > sub_range.end {
                            //case 1: sub range is fully included in range map
                            // new out range is just src range offset by index
                            let new_range = sub_range.start + range_map.offset_key.. sub_range.end + range_map.offset_key;
                            new_ranges.push(new_range);
                            mapped_src_ranges.push(sub_range.clone());
                        } else {
                            // case 2: sub range starts in range but runs longer than range, only map up to end of range
                            let new_range = sub_range.start + range_map.offset_key..range_map.src_range.end + range_map.offset_key;
                            new_ranges.push(new_range);
                            mapped_src_ranges.push((sub_range.start..range_map.src_range.end).clone());
                        }
                    } else {
                        // range starts after or on sub_range
                        if range_map.src_range.end < sub_range.end {
                            // case 3: sub_range includes the whole of this range, just map sub_range:
                            let new_range = sub_range.start + range_map.offset_key..sub_range.end + range_map.offset_key;
                            new_ranges.push(new_range);
                            mapped_src_ranges.push((sub_range.start..sub_range.end).clone());
                        } else {
                            // case 4: sub range starts outside of range but ends within. Map from start of range to end of sub range
                            let new_range = range_map.src_range.start + range_map.offset_key..sub_range.end + range_map.offset_key;
                            new_ranges.push(new_range);
                            mapped_src_ranges.push((range_map.src_range.start..sub_range.end).clone());
                        }
                    }
                    
                }
                mapped_src_ranges.sort_by(|r1: &Range<i64>,r2: &Range<i64>| r1.start.cmp(&r2.start));

                let mut i = sub_range.start;
                for mapped in &mapped_src_ranges {
                    if mapped.contains(&i) {
                        if mapped.end >= sub_range.end {
                            break;
                        } else {
                            new_ranges.push(i..mapped.end);
                            i = mapped.end;
                            break;
                        }
                    } else {
                        i = mapped.end;
                    }
                }
            }
            sub_ranges = new_ranges.clone();
        }
        sub_ranges.sort_by(|r1: &Range<i64>,r2: &Range<i64>| r1.start.cmp(&r2.start));
        location_ranges.push(sub_ranges.clone());
    }

    locations.sort();
    println!("{}", locations.first().unwrap());

    let mut filtered:Vec<Vec<Range<i64>>> = location_ranges.into_iter().filter(|r| !r.is_empty()).collect();
    filtered.sort_by(|r1,r2| r1.get(0).unwrap().start.cmp(&r2.get(0).unwrap().start));
    println!("{}", filtered.first().unwrap().first().unwrap().start);

    println!("filtered {:?}", filtered);
}

#[derive(Debug)]
#[derive(Clone)]
struct RangeMap<Idx> {
    src_range: Range<Idx>,
    out_range: Range<Idx>,
    offset_key: i64
}
pub trait OptNumStr {
    fn to_i32(&self) -> i64;
}

impl OptNumStr for String {
    fn to_i32(&self) -> i64 { 
        return self.to_string().parse::<i64>().unwrap()
    }
}