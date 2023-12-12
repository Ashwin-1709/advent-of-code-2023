use std::{fs, collections::{HashSet, HashMap}};

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

fn is_adjacent(x: i32, y: i32, grid: &Vec<&str>, n: i32, m: i32) -> (HashSet<(i32, i32)>, bool) {
    let mut flag = false;
    let dx = vec![0, 0, -1, -1, -1, 1, 1, 1];
    let dy = vec![-1, 1, -1, 0, 1, -1, 0, 1];
    let gear : char = '*';
    let mut gears : HashSet<(i32, i32)> = HashSet::new();
    for nx in dx {
        for ny in dy.iter() {
            let newx = x + nx;
            let newy = y + ny;
            if !check(newx, newy, n, m) {
                continue;
            }
            flag |= is_special(getchar(newx, newy, grid)) && getchar(newx, newy, grid) != 46;
            if is_special(getchar(newx, newy, grid)) && getchar(newx, newy, grid) == gear as u8 {
                gears.insert((newx, newy));
            }
        }
    }

    return (gears, flag);
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

    let mut gear_to_num: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
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
            let mut gears : HashSet<(i32, i32)> = HashSet::new();
            while check(x, j, n, m) && !is_special(getchar(x, j, &grid)) {
                vis[x as usize][j as usize] = true;
                let (gear_set, _upd) = is_adjacent(x, j, &grid, n, m);
                gears.extend(gear_set);
                j += 1;
            }
            let num = grid[x as usize]
                .get(y as usize..j as usize)
                .and_then(|x| x.parse::<i32>().ok()).unwrap();

            if !gears.is_empty() {
                for (nx, ny) in gears {
                    if !gear_to_num.contains_key(&(nx, ny)) {
                        gear_to_num.insert((nx, ny), Vec::new());
                    }
                    let values = gear_to_num.get_mut(&(nx, ny)).unwrap();
                    values.push(num);
                }
            }
        }
    }

    for(_, values) in gear_to_num.iter() {
        if values.len() != 2 {
            continue;
        }
        let s1 = values.get(0).unwrap();
        let s2 = values.get(1).unwrap();
        ans += s1 * s2;
    }

    println!("{}", ans)
}
