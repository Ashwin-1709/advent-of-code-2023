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
    tilt(&mut grid_chars);
    println!("{}", score(&mut grid_chars));
}
