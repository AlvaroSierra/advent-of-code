


fn main() {
    let data: Vec<_> = include_str!("../index.txt")
        .lines()
        .map(|x| x.split_once(':').unwrap().1.split_once('|').unwrap())
        // .collect::<Vec<(&str, &str)>>()
        .map(|(winners, mine)|
            {
                let a = winners.split(" ").collect::<Vec<&str>>();
                let b = mine.split(" ").filter(|x| a.contains(x) && *x != "").collect::<Vec<&str>>().len() as u32;
                if b == 0 {
                    return 0
                }
                2_i32.pow(b - 1)
            }
        ).collect();

    dbg!(&data.iter().sum::<i32>());

}
