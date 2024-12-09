fn main() {
    let input = include_str!("../input.txt");

    let memory_block: Vec<Option<usize>> = input
        .chars()
        .filter_map(|x| x.to_digit(10).and_then(|x| Some(x as usize)))
        .enumerate()
        .map(|(inx, x)| {
            if inx as f32 % 2.0 == 0.0 {
                return vec![Some(inx / 2); x];
            }
            vec![None; x]
        })
        .flatten()
        .collect();

    let mut rev_iterator = memory_block
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, x)| x.is_some())
        .map(|x| (x.0, x.1.unwrap()));

    let mut max_index = 20000000;

    let a: usize = memory_block
        .iter()
        .enumerate()
        .filter_map(|(inx, x)| {
            if inx >= max_index {
                return None
            }

            Some(x.unwrap_or_else(|| {
                let val = rev_iterator.next().unwrap();
                max_index = val.0;
                val.1
            }))
        })
        .enumerate()
        .map(|(inx, x)| inx * x)
        .sum();

    println!("{}", a)
}
