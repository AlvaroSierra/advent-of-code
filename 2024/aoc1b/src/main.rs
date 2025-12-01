use std::time::Instant;
use hashbrown::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start = Instant::now();
    let input_str = include_str!("../input.txt");

    let mut left_column: HashSet<u32> = HashSet::with_capacity(1000);
    let mut right_column: Vec<u32> = vec![0; 1024];

    for (inx, line) in input_str.lines().enumerate() {
        left_column.insert(line[0..=4].parse()?);
        right_column[inx] = line[8..=12].parse()?;
    }

    let mut result: u32 = 0;
    for i in right_column.into_iter() {
        if left_column.contains(&i) {
            result += i
        }
    }

    println!("{:?}", start.elapsed());
    println!("{}", result);

    Ok(())
}
