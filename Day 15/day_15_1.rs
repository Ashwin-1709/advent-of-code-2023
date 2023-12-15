use std::{fs, hash};
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn solve(str: &Vec<char>) -> u64 {
    let mut ans: u64 = 0;
    for c in str {
        assert!(c.is_ascii());
        if *c == '\n' || *c == '\r' {
            continue;
        }
        ans += *c as u64;
        ans *= 17;
        ans %= 256;
    }
    return ans;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let mut ans: u64 = 0;
    let hash_str: Vec<&str> = input.split(',').collect();
    for cur_hash in hash_str {
        let char_vec: Vec<char> = cur_hash.chars().collect();
        ans += solve(&char_vec);
    }
    println!("{ans}");
}
