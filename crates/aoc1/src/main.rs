const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];



fn main() {
    let input = include_str!("../input.txt");

    println!("{}", &input
        .lines()
        .map(
        |x|
            {
                let mut data: Vec<char> = x.chars().filter(|x| x.is_numeric()).collect();
                format!("{}{}", &data.first().unwrap(), &data.last().unwrap()).parse::<u32>().unwrap()
            }
    ).sum::<u32>());
}
