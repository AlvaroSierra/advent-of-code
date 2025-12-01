fn main() {
    let data: Vec<_> = include_str!("../index.txt")
        .lines()
        .map(|x| x.split_once(':').unwrap().1.split_once('|').unwrap())
        .map(|(winners, mine)| {
            let a = winners.split(" ").collect::<Vec<&str>>();
            mine.split(" ")
                .filter(|x| a.contains(x) && *x != "")
                .collect::<Vec<&str>>()
                .len() as u32
        })
        .collect();

    let mut multiplier: Vec<u32> = vec![1; data.len()];

    for (n, i) in data.iter().enumerate() {
        for j in 1..=*i {
            multiplier[n + j as usize] += multiplier[n]
        }
    }

    dbg!(&data);
    dbg!(&multiplier.iter().sum::<u32>());
}
