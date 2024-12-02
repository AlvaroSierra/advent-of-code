use std::time::Instant;

use rayon::prelude::*;

fn main() {

    let input_str: &'static str = include_str!("../input.txt");

    let start = Instant::now();

    let a: usize = input_str.lines().map( |line| {
        let digits: Vec<isize> = line.split(" ").map(|x| x.parse::<isize>().unwrap()).collect();
        let mut multipier = 1;

        if digits[0] > digits[1]{
            multipier = -1            
        }

        let mut iterator1 = digits.iter().map(|x| x * multipier);
        let mut iterator = iterator1.clone().zip(iterator1.skip(1));

        if iterator.find(|(a, b)| *a >= *b || a.abs_diff(*b) > 3) == None {
            return 1
        }
        return 0
    }).sum();

    println!("{:?}", start.elapsed());
    println!("{}", a)

}
