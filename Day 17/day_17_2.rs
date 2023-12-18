use ::std::fs;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct state {
    dist: i32,
    x: i32,
    y: i32,
    last_dir: usize,
    step: usize,
}

impl Ord for state {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dist < other.dist {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for state {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(&other));
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let grid: Vec<&str> = input.lines().collect();
    let g: Vec<Vec<i32>> = grid
        .into_iter()
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let n = g.len();
    let m = g[0].len();

    let dir: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut heap: BinaryHeap<state> = BinaryHeap::new();
    let mut dis: HashMap<(i32, i32, usize, usize), i32> = HashMap::new();
    heap.push(state {
        dist: 0,
        x: 0,
        y: 0,
        last_dir: 5,
        step: 0,
    });

    while let Some(state {
        dist,
        x,
        y,
        last_dir,
        step,
    }) = heap.pop()
    {
        if dis.contains_key(&(x, y, last_dir, step)) {
            continue;
        }
        dis.entry((x, y, last_dir, step)).or_insert(dist);
        for i in 0..4 {
            let (dx, dy) = dir[i];
            let nx = dx + x;
            let ny = dy + y;
            if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                continue;
            }
            let is_perpendicular: bool =
                last_dir == 5 || (dir[last_dir].0 * dx + dir[last_dir].1 * dy == 0);
            let new_step_cnt: usize = if i == last_dir { step + 1 } else { 1 };
            if last_dir == 5
                || (i == last_dir && new_step_cnt <= 10)
                || (i != last_dir && is_perpendicular && step >= 4)
            {
                let nxt_x = nx as usize;
                let nxt_y = ny as usize;
                heap.push(state {
                    dist: dist + g[nxt_x][nxt_y],
                    x: nx,
                    y: ny,
                    last_dir: i,
                    step: new_step_cnt,
                });
            }
        }
    }

    let mut ans: i32 = i32::MAX;
    for ((x, y, last_dir, step), dist) in dis {
        if x == (n - 1) as i32 && y == (m - 1) as i32 {
            ans = std::cmp::min(ans, dist);
        }
    }

    println!("{ans}");
}
