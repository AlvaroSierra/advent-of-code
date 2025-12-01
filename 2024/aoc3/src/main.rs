use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = include_str!("../input.txt");

    let a: isize = re.captures_iter(input)
    .map(|c| c.extract())
    .map(|(_, [a, b])| a.parse::<isize>().unwrap() * b.parse::<isize>().unwrap())
    .sum();

    println!("{}", a)

}
