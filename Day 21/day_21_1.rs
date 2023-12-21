use std::collections::HashMap;
use std::fs;
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

const dx: [i32; 4] = [1, -1, 0, 0];
const dy: [i32; 4] = [0, 0, 1, -1];

fn dfs(
    x: i32,
    y: i32,
    g: &Vec<Vec<char>>,
    step: i32,
    cache: &mut Vec<Vec<HashMap<i32, bool>>>,
    n: i32,
    m: i32,
) -> () {
    let cx = x as usize;
    let cy = y as usize;

    if cache[cx][cy].contains_key(&(step)) || step == 64 {
        cache[cx][cy].entry(step).or_insert(true);
        return;
    }

    cache[cx][cy].entry(step).or_insert(true);
    for i in 0..4 {
        let nx = x + dx[i];
        let ny = y + dy[i];
        if nx < 0 || ny < 0 || nx >= n || ny >= m {
            continue;
        }
        if g[nx as usize][ny as usize] == '#' {
            continue;
        }
        dfs(nx, ny, g, step + 1, cache, n, m);
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let grid: Vec<&str> = input.lines().collect();
    let g: Vec<Vec<char>> = grid.into_iter().map(|row| row.chars().collect()).collect();
    let n = g.len();
    let m = g[0].len();
    let mut sx = 0;
    let mut sy = 0;
    for x in 0..n {
        for y in 0..m {
            if g[x][y] == 'S' {
                sx = x;
                sy = y;
            }
        }
    }

    let mut cache: Vec<Vec<HashMap<i32, bool>>> = Vec::with_capacity(n + 1);
    cache.resize(n + 1, Vec::with_capacity(m + 1));
    for row in 0..n {
        cache[row].resize(m + 1, HashMap::new());
    }

    dfs(sx as i32, sy as i32, &g, 0, &mut cache, n as i32, m as i32);

    let mut ans = 0;
    for x in 0..n {
        for y in 0..m {
            if cache[x][y].contains_key(&64) {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
