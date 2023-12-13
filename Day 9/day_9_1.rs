use std::collections::HashSet;
use std::{collections::HashMap, fs};

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn f(a: Vec<i32>) -> i32 {
    if a.len() == 1 {
        return *a.last().unwrap();
    }
    let mut nxt: Vec<i32> = Vec::new();
    for i in 1..a.len() {
        nxt.push(a[i] - a[i - 1]);
    }
    let ans: i32 = *nxt.last().unwrap();
    return ans + f(nxt);
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let arrays: Vec<&str> = input.lines().collect();
    let mut ans: i32 = 0;
    for array in arrays {
        let cur: Vec<&str> = array.split_ascii_whitespace().collect();
        let mut sequence: Vec<i32> = Vec::new();
        for num in cur {
            sequence.push(num.parse().expect(PARSE_ERROR));
        }
        ans += *sequence.last().unwrap();
        ans += f(sequence);
    }
    println!("{}", ans);
}
