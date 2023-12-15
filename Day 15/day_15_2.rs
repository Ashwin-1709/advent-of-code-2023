use std::fs;
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn update_box(boxes: &mut Vec<Vec<(String, u64)>>, i: usize, key: &str, val: u64) {
    let mut found = false;
    for j in 0..boxes[i].len() {
        if boxes[i][j].0 == key {
            boxes[i][j].1 = val;
            found = true;
            break;
        }
    }
    if !found {
        boxes[i].push((key.to_owned(), val));
    }
}

fn hash(str: &Vec<char>) -> u64 {
    let mut ans: u64 = 0;
    for c in str {
        assert!(c.is_ascii());
        if *c == '\n' || *c == '\r' {
            continue;
        }
        ans += *c as u64;
        ans *= 17;
        ans %= 256;
    }
    return ans;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let mut ans: u64 = 0;
    let hash_str: Vec<&str> = input.split(',').collect();
    let mut boxes: Vec<Vec<(String, u64)>> = Vec::with_capacity(256);
    boxes.resize(256, Vec::new());
    for cur_hash in hash_str {
        let mut char_vec: Vec<char> = cur_hash.chars().collect();
        let n = char_vec.len();
        if char_vec[n - 1] == '-' {
            char_vec.pop();
            let id: usize = hash(&char_vec) as usize;
            let key: String = char_vec.into_iter().collect();
            let mut del: Vec<(String, u64)> = Vec::new();
            for data in boxes[id].clone() {
                if data.0 != key {
                    del.push(data);
                }
            }
            std::mem::swap(&mut del, &mut boxes[id]);
        } else if char_vec[n - 2] == '=' {
            let focal_len = (char_vec[n - 1] as u64) - 48;
            char_vec.pop();
            char_vec.pop();
            let id: usize = hash(&char_vec) as usize;
            let key: String = char_vec.into_iter().collect();
            update_box(&mut boxes, id, &key, focal_len);
        }
    }

    for box_num in 0..(256 as usize) {
        let mut box_slot: u64 = 1;
        for (_name, focal_len) in boxes[box_num].clone() {
            ans += focal_len * box_slot * ((box_num + 1) as u64);
            box_slot += 1;
        }
    }
    println!("{ans}");
}
