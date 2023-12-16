use std::{collections::HashSet, fs};
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let grid: Vec<&str> = input.lines().collect();
    let g: Vec<Vec<char>> = grid.into_iter().map(|row| row.chars().collect()).collect();
    let n = g.len();
    let m = g[0].len();
    let mut sx: usize = 0;
    let mut sy: usize = 0;
    for x in 0..n {
        for y in 0..m {
            if g[x][y] == 'S' {
                sx = x;
                sy = y;
            }
        }
    }

    let dx = vec![0, 0, 1, -1];
    let dy = vec![1, -1, 0, 0];
    for i in 0..4 {
        let mut init_dx = dx[i];
        let mut init_dy = dy[i];
        let mut cycle = 0;
        let mut found: bool = false;
        let mut x = sx;
        let mut y = sy;
        while !found {
            let nx = (x as i32) + init_dx;
            let ny = (y as i32) + init_dy;
            if nx < 0 || ny < 0 || nx >= (n as i32) || ny >= (m as i32) {
                break;
            }
            cycle += 1;
            let nxt_x = nx as usize;
            let nxt_y = ny as usize;
            if nxt_x == sx && nxt_y == sy {
                found = true;
                break;
            }

            match g[nxt_x][nxt_y] {
                '.' => break,
                'F' => {
                    if init_dx == -1 && init_dy == 0 {
                        init_dx = 0;
                        init_dy = 1;
                    } else if init_dx == 0 && init_dy == -1 {
                        init_dx = 1;
                        init_dy = 0;
                    } else {
                        break;
                    }
                }
                '7' => {
                    if init_dx == -1 && init_dy == 0 {
                        init_dx = 0;
                        init_dy = -1;
                    } else if init_dx == 0 && init_dy == 1 {
                        init_dx = 1;
                        init_dy = 0;
                    } else {
                        break;
                    }
                }
                'J' => {
                    if init_dx == 1 && init_dy == 0 {
                        init_dx = 0;
                        init_dy = -1;
                    } else if init_dx == 0 && init_dy == 1 {
                        init_dx = -1;
                        init_dy = 0;
                    } else {
                        break;
                    }
                }
                'L' => {
                    if init_dx == 1 && init_dy == 0 {
                        init_dx = 0;
                        init_dy = 1;
                    } else if init_dx == 0 && init_dy == -1 {
                        init_dx = -1;
                        init_dy = 0;
                    } else {
                        break;
                    }
                }
                '-' => {
                    if init_dx != 0 {
                        break;
                    }
                }
                '|' => {
                    if init_dy != 0 {
                        break;
                    }
                }
                (_) => (),
            };

            x = nxt_x;
            y = nxt_y;
        }

        if found {
            println!("{}", (cycle + 1) / 2);
        }
    }
}
