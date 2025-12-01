#![feature(portable_simd)]
use std::{simd::{i32x64, num::SimdInt}, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let input_str = include_str!("../input.txt");

    let mut left_column: Vec<i32> = vec![0; 1024];
    let mut right_column: Vec<i32> = vec![0; 1024];

    for (inx, line) in input_str.lines().enumerate() {
        left_column[inx] = line[0..=4].parse()?;
        right_column[inx] = line[8..=12].parse()?;
    }

    let mut result: i32 = 0;

    left_column.sort();
    right_column.sort();

    for (left_chunk, right_chunk) in left_column.chunks(64).zip(right_column.chunks(64)) {
        let left_simd_vec = i32x64::from_slice(left_chunk);
        let right_simd_vec = i32x64::from_slice(right_chunk);
        let subtract = left_simd_vec - right_simd_vec;
        let absolute = subtract.abs();
        result += absolute.to_array().iter().sum::<i32>();
    }

    println!("{:?}", start.elapsed());
    println!("{}", result);

    Ok(())
}
