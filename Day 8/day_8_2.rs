use std::{fs, collections::HashMap};
use std::cmp::{max, min};

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";


fn gcd(a: i64, b: i64) -> i64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn is_source(s: &str) -> bool{
    let x = s.len();
    s.chars().nth(x - 1) == Some('A')
}

fn is_dest(s: &str) -> bool {
    let x = s.len();
    s.chars().nth(x - 1) == Some('Z')
}

fn find_cycle_len(node: &str, graph: &HashMap<&str, (&str, &str)>, seq: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut i: usize = 0;

    let mut src = node;
    while !is_dest(src) {
        let (l, r) = graph.get(src).unwrap();
        let mut nxt: usize = i + 1;
        if nxt == seq.len() {
            nxt = 0;
        }
        if seq.chars().nth(i).unwrap() == 'L' {
            src = l;
        } else {
            src = r;
        }
        i = nxt;
        ans += 1;
    }
    return ans;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let instruction: Vec<&str> = input.lines().collect();
    let seq = instruction[0];
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut source: Vec<&str> = Vec::new();
    let mut ans: i64 = 1;
    for i in 2..instruction.len() {
        let adj: Vec<&str> = instruction[i].split_ascii_whitespace().collect();
        let node = adj[0];
        let l = adj[2].get(1..4).unwrap();
        let r = adj[3].get(0..3).unwrap();
        graph.entry(node).or_insert((l, r));
        if is_source(node) {
            source.push(node);
        }
    }

    for ppl in source {
        ans = lcm(ans, find_cycle_len(ppl, &graph, &seq))
    }
    println!("{}", ans);
}
