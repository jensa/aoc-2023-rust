use crate::util::{self, OptNumStr, to_i64_vec};
use crate::util::to_i32_vec;

pub fn solve() {
    let rows = util::input_lines_as_strings();
    let times = to_i64_vec(rows.get(0).unwrap().split(":").last().unwrap().to_string());
    let distances = to_i64_vec(rows.get(1).unwrap().split(":").last().unwrap().to_string());

    let races:Vec<Race> = times.clone().into_iter().enumerate().map(|(i,time)| Race {time, distance:distances.get(i).unwrap().clone()}).collect();

    println!("part 1 total: {:?}", total_wins(&races));

    let race_time:String = times.iter().map(|t| t.to_string()).collect();
    let race_dist:String = distances.iter().map(|t| t.to_string()).collect();
    
    let race = Race {time:race_time.to_i64(), distance:race_dist.to_i64()};
    let big_races = &vec![race];
    println!("part 2 total: {:?}", total_wins(&big_races));

}

fn total_wins(races:&Vec<Race>) -> i32 {
    let mut total_wins = 0;
    for race in races {
        let mut wins = 0;
        let mut speed = 0;
        for t in 0..race.time-1 {
            
            let distance_now = speed * (race.time-t);
            if distance_now > race.distance {
                wins += 1;
            }
            speed += 1;
        }
        total_wins = if total_wins == 0 { wins } else  {total_wins * wins };
    }
    return total_wins;
}

#[derive(Debug)]
#[derive(Clone)]
struct Race {
    time:i64,
    distance:i64
}