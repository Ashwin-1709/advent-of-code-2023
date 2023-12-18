use std::fs;
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";
fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let instructions: Vec<&str> = input.lines().collect();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut ext: i32 = 0;
    let mut points: Vec<(i32, i32)> = Vec::new();
    points.push((x, y));
    for seq in instructions {
        let data: Vec<&str> = seq.split_ascii_whitespace().collect();
        let dis: i32 = data[1].parse::<i32>().expect(PARSE_ERROR);
        match data[0] {
            "R" => y += dis,
            "L" => y -= dis,
            "U" => x -= dis,
            "D" => x += dis, 
            _ => ()
        };

        points.push((x, y));
        ext += dis;
    }

    // shoelace formula for area under polygon: https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area: i32 = 0;
    let N = points.len();
    for i in 0..N {
        area += (points[i].1 + points[(i + 1) % N].1) * (points[i].0 - points[(i + 1) % N].0);
    }

    area = area.abs();
    area /= 2;

    // picks theorem: https://en.wikipedia.org/wiki/Pick%27s_theorem
    println!("{}", area + ext/2 + 1);
    
}
