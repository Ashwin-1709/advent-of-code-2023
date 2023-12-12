use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let games: Vec<&str> = input.lines().collect();
    let mut dp : Vec<i32> = Vec::with_capacity(games.len());
    dp.resize(games.len(), 1);
    
    let mut ans: i32 = 0;
    for id in 0..games.len() {
        let card: Vec<&str> = games[id].split_ascii_whitespace().collect();
        let mut win_num: HashSet<i32> = HashSet::new();
        let mut match_cnt: i32 = 0;
        let mut seen: bool = false;
        for i in 2..card.len() {
            if card[i] == "|" {
                seen = true;
                continue;
            } 
            let number = card[i].parse::<i32>().expect("parsing issue");
            if !seen {
                win_num.insert(number);
            } else if win_num.contains(&number){
                match_cnt += 1;
            }
        }
        if match_cnt > 0 {
            for nxt in 1..=match_cnt {
                let push = nxt as usize + id;
                if push >= games.len() {
                    continue;
                }
                dp[push] += dp[id];
            }
        }
    }

    for i in 0..dp.len() {
        ans += dp[i];
    }
    println!("{}", ans);
}
