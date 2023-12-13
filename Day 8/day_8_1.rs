use std::{fs, collections::HashMap};

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let instruction: Vec<&str> = input.lines().collect();
    let seq = instruction[0];
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    for i in 2..instruction.len() {
        let adj: Vec<&str> = instruction[i].split_ascii_whitespace().collect();
        let node = adj[0];
        let l = adj[2].get(1..4).unwrap();
        let r = adj[3].get(0..3).unwrap();
        graph.entry(node).or_insert((l, r));
    }
    
    let mut node = "AAA";
    let dest = "ZZZ";
    let mut i: usize = 0;
    let mut ans: i32 = 0;
    while !node.eq(dest) {
        let (l, r) = graph.get(node).unwrap();
        let mut nxt: usize = i + 1;
        if nxt == seq.len() {
            nxt = 0;
        }
        if seq.chars().nth(i).unwrap() == 'L' {
            node = l;
        } else {
            node = r;
        }
        i = nxt;
        ans += 1;
    }

    println!("{}", ans);
}
