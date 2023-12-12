use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let data: Vec<&str> = input.lines().collect();
    let time: Vec<&str> = data[0].split_ascii_whitespace().collect();
    let dist: Vec<&str> = data[1].split_ascii_whitespace().collect();

    assert!(time.len() == dist.len());

    let mut ans: i32 = 1;
    for i in 1..time.len() {
        let t: i32 = time[i].parse().expect("parsing issue");
        let d: i32 = dist[i].parse().expect("parsing issue");
        let mut min_t: i32 = i32::MAX;
        let mut max_t: i32 = i32::MIN;
        for hold in 1..=t {
            if hold * (t - hold) > d {
                min_t = std::cmp::min(min_t, hold);
                max_t = std::cmp::max(max_t, hold);
            }
        }
        ans = ans * (max_t - min_t + 1);
    }
    println!("{}", ans);
}
