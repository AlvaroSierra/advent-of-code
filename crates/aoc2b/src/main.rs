use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let a: i32 = input.lines().map(|x| main_game(x.split_once(": ").unwrap().1)).collect::<Vec<i32>>().iter().sum();
    let m = start.elapsed();
    println!("{:?}", m);
    println!("{}", a);
}

fn main_game(game: &str) -> i32{
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for i in game.replace("; ", ", ").split(", ") {
        let a = i.split_once(" ").unwrap();
        let count = a.0.parse::<i32>().unwrap();

        match a.1 {
            "red" => {if count > red { red = count } },
            "blue" => {if count > blue { blue = count } },
            "green" => {if count > green { green = count } },
            _ => {}
        }
    }

    red * green * blue
}
