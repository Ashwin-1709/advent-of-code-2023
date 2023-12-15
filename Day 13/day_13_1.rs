use std::fs;
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn solve(grid: &Vec<&str>) -> usize {
    let row_cnt = grid.len();
    let col_cnt = grid[0].len();
    let mut g: Vec<Vec<char>> = Vec::with_capacity(row_cnt + 1);
    g.resize(row_cnt + 1, Vec::with_capacity(col_cnt + 1));
    for x in 0..row_cnt {
        g[x].resize(col_cnt + 1, '?');
        let row: Vec<char> = grid[x].chars().collect();
        g[x] = row;
    }

    let mut vert: usize = 0;
    let mut hori: usize = 0;
    for y in 0..(grid[0].len() - 1) {
        let mut is_mirror: bool = true;
        let mut l = y;
        let mut r = y + 1;
        while r < grid[0].len() {
            for x in 0..grid.len() {
                is_mirror &= g[x][l] == g[x][r];
            }
            if l == 0 {
                break;
            }
            l -= 1;
            r += 1;
        }
        if is_mirror {
            vert += y + 1;
        }
    }

    for x in 0..(grid.len() - 1) {
        let mut is_mirror: bool = true;
        let mut l = x;
        let mut r = x + 1;
        while r < grid.len() {
            for y in 0..grid[0].len() {
                is_mirror &= g[l][y] == g[r][y];
            }
            if l == 0 {
                break;
            }
            l -= 1;
            r += 1;
        }
        if is_mirror {
            hori += x + 1;
        }
    }
    return vert + 100 * hori;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let grid: Vec<&str> = input.lines().collect();
    let mut ans: usize = 0;
    let mut cur_mountain: Vec<&str> = Vec::new();
    for row in grid {
        if row.is_empty() {
            ans += solve(&cur_mountain);
            cur_mountain.clear();
        } else {
            cur_mountain.push(row);
        }
    }

    if !cur_mountain.is_empty() {
        ans += solve(&cur_mountain);
    }

    println!("{ans}");
}
