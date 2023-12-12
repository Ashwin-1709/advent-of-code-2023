use std::fs;

fn check(x: i32, y: i32, n: i32, m: i32) -> bool {
    return x >= 0 && y >= 0 && x < n && y < m;
}

fn getchar(x: i32, y: i32, grid: &Vec<&str>) -> u8 {
    let c = grid[x as usize].as_bytes()[y as usize];
    return c;
}

fn is_special(c: u8) -> bool {
    return c < 48 || c > 57;
}

fn is_adjacent(x: i32, y: i32, grid: &Vec<&str>, n: i32, m: i32) -> bool {
    let mut flag = false;
    let dx = vec![0, 0, -1, -1, -1, 1, 1, 1];
    let dy = vec![-1, 1, -1, 0, 1, -1, 0, 1];
    for nx in dx {
        for ny in dy.iter() {
            let newx = x + nx;
            let newy = y + ny;
            if !check(newx, newy, n, m) {
                continue;
            }
            flag |= (is_special(getchar(newx, newy, grid)) && getchar(newx, newy, grid) != 46);
        }
    }
    return flag;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let grid: Vec<&str> = input.lines().collect();
    let mut ans: i32 = 0;
    let n: i32 = grid.len() as i32;
    let m: i32 = grid[0].len() as i32;
    let mut vis: Vec<Vec<bool>> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let mut row: Vec<bool> = Vec::with_capacity(m as usize);
        for _ in 0..m {
            row.push(false);
        }
        vis.push(row);
    }

    for x in 0..n {
        for y in 0..m {
            let mut j = y;
            if vis[x as usize][y as usize] {
                continue;
            }
            if is_special(getchar(x, y, &grid)) {
                continue;
            }
            vis[x as usize][j as usize] = true;
            let mut flag = false;
            while check(x, j, n, m) && !is_special(getchar(x, j, &grid)) {
                vis[x as usize][j as usize] = true;
                flag |= is_adjacent(x, j, &grid, n, m);
                j += 1;
            }
            let num = grid[x as usize]
                .get(y as usize..j as usize)
                .and_then(|x| x.parse::<i32>().ok()).unwrap();
            if flag {
                ans += num;
            }
        }
    }

    println!("{}", ans)
}
