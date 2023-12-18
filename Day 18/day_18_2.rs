use std::fs;
const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let instructions: Vec<&str> = input.lines().collect();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut ext: i64 = 0;
    let mut points: Vec<(i64, i64)> = Vec::new();
    points.push((x, y));
    for seq in instructions {
        let data: Vec<&str> = seq.split_ascii_whitespace().collect();
        let mut hexa: Vec<char> = data[2]
            .get(2..(data[2].len() - 1))
            .unwrap()
            .chars()
            .into_iter()
            .collect();

        let dir = hexa[5];
        hexa.pop();

        let pass: String = hexa.iter().collect();

        let dis = i64::from_str_radix(&pass, 16).unwrap();
        match dir {
            '0' => y += dis,
            '1' => x += dis,
            '2' => y -= dis,
            '3' => x -= dis,
            _ => (),
        };

        points.push((x, y));
        ext += dis;
    }

    // shoelace formula for area under polygon: https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area: i64 = 0;
    let N = points.len();
    for i in 0..N {
        area += (points[i].1 + points[(i + 1) % N].1) * (points[i].0 - points[(i + 1) % N].0);
    }

    area = area.abs();
    area /= 2;

    // picks theorem: https://en.wikipedia.org/wiki/Pick%27s_theorem
    println!("{}", area + ext / 2 + 1);
}
