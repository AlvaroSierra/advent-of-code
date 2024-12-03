#![feature(iter_intersperse)]

use regex::Regex;

fn main() {
    let re1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = include_str!("../input.txt");

    let initial_string = input
        .split("do()")
        .map(|x| x.split("don't()").next().unwrap())
        .fold(String::new(), |a, b| a + b + ",");

    let a: isize = re1.captures_iter(&initial_string)
    .map(|c| c.extract())
    .map(|(_, [a, b])| a.parse::<isize>().unwrap() * b.parse::<isize>().unwrap())
    .sum();

    println!("{}", a)

}
