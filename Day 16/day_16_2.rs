use std::{collections::HashMap, fs};
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn dfs(
    x: usize,
    y: usize,
    g: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<bool>>,
    dx: i32,
    dy: i32,
    cache: &mut Vec<Vec<HashMap<(i32, i32), bool>>>,
) -> () {
    vis[x][y] = true;
    if cache[x][y].contains_key(&(dx, dy)) {
        return;
    }
    let mut curx = x as i32;
    let mut cury = y as i32;
    curx += dx;
    cury += dy;
    if curx < 0 || cury < 0 || curx >= (g.len() as i32) || cury >= (g[0].len() as i32) {
        return;
    }
    cache[x][y].entry((dx, dy)).or_insert(true);
    let nx = curx as usize;
    let ny = cury as usize;
    match g[nx][ny] {
        '.' => dfs(nx, ny, g, vis, dx, dy, cache),
        '|' => {
            if dy == 0 {
                dfs(nx, ny, g, vis, dx, dy, cache);
            } else {
                dfs(nx, ny, g, vis, -1, 0, cache);
                dfs(nx, ny, g, vis, 1, 0, cache);
            }
        }
        '-' => {
            if dx == 0 {
                dfs(nx, ny, g, vis, dx, dy, cache);
            } else {
                dfs(nx, ny, g, vis, 0, 1, cache);
                dfs(nx, ny, g, vis, 0, -1, cache);
            }
        }
        '/' => {
            if dx == 0 {
                if dy == 1 {
                    dfs(nx, ny, g, vis, -1, 0, cache);
                } else {
                    assert!(dy == -1);
                    dfs(nx, ny, g, vis, 1, 0, cache);
                }
            } else {
                assert!(dy == 0);
                if dx == 1 {
                    dfs(nx, ny, g, vis, 0, -1, cache);
                } else {
                    dfs(nx, ny, g, vis, 0, 1, cache);
                }
            }
        }
        '\\' => {
            if dx == 0 {
                if dy == 1 {
                    dfs(nx, ny, g, vis, 1, 0, cache);
                } else {
                    assert!(dy == -1);
                    dfs(nx, ny, g, vis, -1, 0, cache);
                }
            } else {
                assert!(dy == 0);
                if dx == 1 {
                    dfs(nx, ny, g, vis, 0, 1, cache);
                } else {
                    dfs(nx, ny, g, vis, 0, -1, cache);
                }
            }
        }
        (_) => (),
    };
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let grid: Vec<&str> = input.lines().collect();
    let g: Vec<Vec<char>> = grid.into_iter().map(|row| row.chars().collect()).collect();
    let n = g.len();
    let m = g[0].len();
    let mut cnt: i32 = 0;
    let pos = n * m;
    let mut done = 0;

    let row_st = vec![0, n - 1];
    let row_col = vec![0, m - 1];

    for stx in row_st {
        for sty in 0..m {
            let mut vis: Vec<Vec<bool>> = Vec::with_capacity(n + 1);
            let mut cache: Vec<Vec<HashMap<(i32, i32), bool>>> = Vec::with_capacity(n + 1);
            cache.resize(n + 1, Vec::with_capacity(m + 1));
            vis.resize(n + 1, Vec::with_capacity(m + 1));
            for row in 0..n {
                vis[row].resize(m + 1, false);
                cache[row].resize(m + 1, HashMap::new());
            }
            let mut dxx = 1;
            if stx == n - 1 {
                dxx = -1;
            }
            dfs(stx, sty, &g, &mut vis, dxx, 0, &mut cache);
            let mut ans: i32 = 0;
            for x in 0..n {
                for y in 0..m {
                    if vis[x][y] {
                        ans += 1;
                    }
                }
            }
            cnt = std::cmp::max(cnt, ans);
        }
    }

    for stx in 0..n {
        for sty in row_col.iter() {
            let mut vis: Vec<Vec<bool>> = Vec::with_capacity(n + 1);
            let mut cache: Vec<Vec<HashMap<(i32, i32), bool>>> = Vec::with_capacity(n + 1);
            cache.resize(n + 1, Vec::with_capacity(m + 1));
            vis.resize(n + 1, Vec::with_capacity(m + 1));
            for row in 0..n {
                vis[row].resize(m + 1, false);
                cache[row].resize(m + 1, HashMap::new());
            }
            let mut dyy = 1;
            if *sty == m - 1 {
                dyy = -1;
            }
            dfs(stx, *sty, &g, &mut vis, 0, dyy, &mut cache);
            let mut ans: i32 = 0;
            for x in 0..n {
                for y in 0..m {
                    if vis[x][y] {
                        ans += 1;
                    }
                }
            }
            cnt = std::cmp::max(cnt, ans);
        }
    }

    println!("{cnt}");
}
