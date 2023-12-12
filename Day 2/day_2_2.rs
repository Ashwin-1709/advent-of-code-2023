use std::fs;

fn get_cnt(game_config: &Vec<&str>) -> (i32, i32, i32) {
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
    
    return (red, blue, green);
}
fn main() {

    let input = fs::read_to_string("src/input.txt").expect("cannot read");
    let games: Vec<&str> = input.lines().collect();
    let mut ans : i64 = 0;
    for game in games {
        let cur_game : Vec<&str> = game.split_ascii_whitespace().collect();
        let mut game_config = Vec::<&str>::new();
        let mut reds: Vec<i32> = Vec::new();
        let mut blues: Vec<i32> = Vec::new();
        let mut greens: Vec<i32> = Vec::new();
        for i in (2..cur_game.len()).step_by(2) {
            game_config.push(cur_game[i]);
            game_config.push(cur_game[i + 1]);
            if cur_game[i + 1].contains(";") {
                let need = get_cnt(&game_config);
                game_config.clear();
                reds.push(need.0);
                blues.push(need.1);
                greens.push(need.2);
            }
        }

        if !game_config.is_empty() {
            let need = get_cnt(&game_config);
            reds.push(need.0);
            blues.push(need.1);
            greens.push(need.2);
            game_config.clear();
        }

        let mut power : i64 = 1;
        let min_red = *reds.iter().max().unwrap() as i64;
        let min_blue = *blues.iter().max().unwrap() as i64;
        let min_greens = *greens.iter().max().unwrap() as i64;
        power = power * min_red * min_greens * min_blue;
        ans += power;
    }

    println!("{}", ans)
}
