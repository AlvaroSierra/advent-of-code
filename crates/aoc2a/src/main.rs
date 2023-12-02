use std::time::Instant;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;



fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");

    let mut result = 0;

    for line in input.lines() {
        let a: Vec<&str> = line.split(":").collect();

        if !main_game(a[1]){
            result += a[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap()
        }
    }

    println!("{:?}", start.elapsed());
    println!("{}", result);
}

fn main_game(game: &str) -> bool {
    for i in game.split(";"){
        if main_show(i) {
            return true
        }
    }
    false
}

fn main_show(show: &str) -> bool{
    for i in show.split(","){
        let a = i.split_whitespace().collect::<Vec<&str>>();

        match a[1]{
            "red" => { if a[0].parse::<u32>().unwrap() > MAX_RED { return true; } },
            "blue" => if a[0].parse::<u32>().unwrap() > MAX_BLUE { return true; },
            "green" => if a[0].parse::<u32>().unwrap() > MAX_GREEN { return true; },
            &_ => {panic!("FUCK")}
        };
    }

    false
}

// TRUE means game INVALID and MUST NOT COUNT