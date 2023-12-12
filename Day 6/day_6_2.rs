use std::fs;

fn get_ways(t: i64, d: i64) -> i64{
    let mut min_t: i64 = i64::MAX;
    let mut max_t: i64 = i64::MIN;
    for hold in 1..=t {
        if hold * (t - hold) > d {
            min_t = std::cmp::min(min_t, hold);
            max_t = std::cmp::max(max_t, hold);
        }
    }
    return max_t - min_t + 1;
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let data: Vec<&str> = input.lines().collect();
    let time: Vec<&str> = data[0].split_ascii_whitespace().collect();
    let dist: Vec<&str> = data[1].split_ascii_whitespace().collect();

    assert!(time.len() == dist.len());

    let mut time_s = String::new();
    let mut dist_s = String::new();
    for i in 1..time.len() {
        time_s += time[i];
        dist_s += dist[i]; 
    }

    let ac_time: i64 = time_s.parse().expect("failed to parse");
    let ac_dist: i64 = dist_s.parse().expect("failed to parse");
    println!("{}", get_ways(ac_time, ac_dist));
}
