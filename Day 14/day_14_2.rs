use std::clone;
use std::collections::HashSet;
use std::mem::swap;
use std::{collections::HashMap, fs};

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn print_grid(grid: &mut Vec<Vec<char>>) -> () {
    for x in 0..grid.len() {
        let row: String = grid[x].clone().into_iter().collect::<String>();
        println!("{}", row);
    }
}

fn valid(x: i32, y: i32, n: usize, m: usize) -> bool {
    return x >= 0 && x < (n as i32) && y >= 0 && y < (m as i32);
}

fn valid_swap(x1: usize, y1: usize, x2: usize, y2: usize, n: usize, m: usize, grid: &mut Vec<Vec<char>>) -> bool{
    if !valid(x1 as i32, y1 as i32, n, m) {
        return false;
    }
    if !valid(x2 as i32, y2 as i32, n, m) {
        return false;
    }

    if grid[x1][y1] != '.' {
        return false;
    }
    return true;
}

fn tilt(grid: &mut Vec<Vec<char>>) -> () {
    let n = grid.len();
    let m = grid[0].len();
    for y in 0..m {
        for x in 1..n {
            if grid[x][y] != 'O' {
                continue;
            }
            let mut up = x - 1;
            let mut cur = x;
            while valid_swap(up, y, cur, y, n, m, grid) {
                let temp = grid[cur][y];
                grid[cur][y] = grid[up][y];
                grid[up][y] = temp;
                if up == 0 {
                    break;
                }
                cur -= 1;
                up -= 1;
            }
        }
    }
}

fn rotate(grid: &mut Vec<Vec<char>>) -> () {
    let n = grid.len();
    let m = grid[0].len();
    let mut rotated_grid = grid.clone();
    for x in 0..n {
        for y in 0..m {
            rotated_grid[y][n - 1 - x] = grid[x][y];
        }
    }

    for x in 0..n {
        for y in 0..m {
            grid[x][y] = rotated_grid[x][y];
        }
    }
}

fn score(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut ans: i32 = 0;
    let mut damage: i32 = grid.len() as i32;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 'O' {
                ans += damage;
            } 
        }
        damage -= 1;
    }
    return ans;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let grid: Vec<&str> = input.lines().collect();
    let mut grid_chars: Vec<Vec<char>> = grid.into_iter().map(|row| row.chars().collect()).collect();
    let mut hash_grid: HashMap<Vec<Vec<char>>, i64> = HashMap::new();
    let T: i64 = 1000000000;
    for mut x in 1..=T {
        for cycle in 0..4 {
            tilt(&mut grid_chars);
            rotate(&mut grid_chars);
        }
        if hash_grid.contains_key(&grid_chars) {
            let cycle_len = x - *hash_grid.get(&grid_chars).unwrap();
            let nxt = (T - x) / (cycle_len);
            x += nxt * cycle_len;
        }
        hash_grid.insert(grid_chars.clone(), x);
        if x >= T {
            break;
        }
    }

    println!("{}", score(&mut grid_chars));
}
