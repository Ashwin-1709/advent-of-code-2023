use std::fs;
const RED_COUNT: i32 = 12;
const GREEN_COUNT: i32 = 13;
const BLUE_COUNT: i32 = 14;

fn get_game_id(game: &str) -> u32{
    let mut cur_id = 0;
    let mut shift: u32 = 1;
    for j in (0..(game.len()-1)).rev() {
        let cur = (game.as_bytes()[j] - 48) as u32;
        cur_id = cur_id + shift * cur;
        shift = shift * 10;
        
    }
    return cur_id;
}

fn valid_game(game_config: &Vec<&str>) -> bool {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for i in (0..(game_config.len())).step_by(2) {
        let box_cnt : i32 = game_config[i].parse().expect("cannot convert");
        match game_config.get(i + 1) {
            Some(&"red,") => red += box_cnt,
            Some(&"blue,") => blue += box_cnt,
            Some(&"green,") => green += box_cnt,
            Some(&"red;") => red += box_cnt,
            Some(&"blue;") => blue += box_cnt,
            Some(&"green;") => green += box_cnt,
            Some(&"red") => red += box_cnt,
            Some(&"blue") => blue += box_cnt,
            Some(&"green") => green += box_cnt,
            Some(_) => (),
            None => (),
        };
    }
    dbg!(red, blue, green);
    return red <= RED_COUNT && blue <= BLUE_COUNT && green <= GREEN_COUNT;
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let games: Vec<&str> = input.lines().collect();
    let mut ans : u32 = 0;
    for game in games {
        let cur_game : Vec<&str> = game.split_ascii_whitespace().collect();
        let mut game_config = Vec::<&str>::new();
        let mut valid : bool = true;
        for i in (2..cur_game.len()).step_by(2) {
            game_config.push(cur_game[i]);
            game_config.push(cur_game[i + 1]);
            if cur_game[i + 1].contains(";") {
                valid &= valid_game(&game_config);
                game_config.clear();
            }
        }

        if !game_config.is_empty() {
            valid &= valid_game(&game_config);
        }

        if valid {
            ans += get_game_id(cur_game[1]);
        }
    }

    println!("{}", ans)
}