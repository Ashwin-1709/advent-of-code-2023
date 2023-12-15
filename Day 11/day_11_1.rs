use std::fs;

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let observatory: Vec<&str> = input.lines().collect();
    let grid: Vec<Vec<char>> = observatory
        .into_iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut expand_row: Vec<usize> = Vec::new();
    let mut expand_col: Vec<usize> = Vec::new();
    let mut galaxy: Vec<(usize, usize)> = Vec::new();
    for x in 0..grid.len() {
        let mut expand: bool = true;
        for y in 0..grid[x].len() {
            expand &= (grid[x][y] == '.');
            if grid[x][y] == '#' {
                galaxy.push((x, y))
            }
        }
        if expand {
            expand_row.push(x);
        }
    }
    for y in 0..grid[0].len() {
        let mut expand: bool = true;
        for x in 0..grid.len() {
            expand &= (grid[x][y] == '.');
        }
        if expand {
            expand_col.push(y);
        }
    }

    let mut ans: usize = 0;
    let mut cnt: usize = 0;
    for i in 0..galaxy.len() {
        for j in (i + 1)..galaxy.len() {
            let (x1, y1) = galaxy[i];
            let (x2, y2) = galaxy[j];
            ans += x1.abs_diff(x2) + y1.abs_diff(y2);
            for row in expand_row.clone() {
                if std::cmp::min(x1, x2) <= row && row <= std::cmp::max(x2, x1) {
                    cnt += 1;
                }
            }
            for col in expand_col.clone() {
                if std::cmp::min(y1, y2) <= col && col <= std::cmp::max(y2, y1) {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", ans + cnt);
}
