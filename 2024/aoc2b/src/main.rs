use std::time::Instant;

use rayon::prelude::*;

fn check(digits: &Vec<isize>) -> bool {
    let mut multipier = 1;

    if digits[0] > digits[1]{
        multipier = -1            
    }

    let mut iterator1 = digits.iter().map(|x| x * multipier);
    let mut iterator = iterator1.clone().zip(iterator1.skip(1));

    iterator.find(|(a, b)| *a >= *b || a.abs_diff(*b) > 3) == None
}


fn main() {

    let input_str: &'static str = include_str!("../input.txt");

    let start = Instant::now();

    let a: usize = input_str.lines().filter( |line| {
        let digits: Vec<isize> = line.split(" ").map(|x| x.parse::<isize>().unwrap()).collect();
        if check(&digits) {
            return true
        }

        for i in 0..digits.len() {
            let mut subset = digits.clone();
            subset.remove(i);

            if check(&subset) {
                return true
            }
        }
        false
    }).count();

    println!("{:?}", start.elapsed());
    println!("{}", a)

}
