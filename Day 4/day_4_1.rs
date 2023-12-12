use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let games: Vec<&str> = input.lines().collect();
    let mut ans: i64 = 0;
    for game in games {
        let card: Vec<&str> = game.split_ascii_whitespace().collect();
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
            match_cnt -= 1;
            ans += 1 << match_cnt;
        }
    }
    println!("{}", ans);
}
