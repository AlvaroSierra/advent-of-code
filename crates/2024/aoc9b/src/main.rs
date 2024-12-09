#[derive(Clone, Copy, Debug)]
struct File {
    id: usize,
    length: usize,
}

#[derive(Clone, Copy, Debug)]
enum MemoryBlock {
    File(File),
    Empty { length: usize },
}

impl MemoryBlock {
    fn length(&self) -> usize {
        match self {
            MemoryBlock::File(a) => a.length,
            MemoryBlock::Empty { length } => *length,
        }
    }
}

fn fill_empty_space<'a>(
    length: usize,
    rev_iterator: &(impl Iterator<Item = &'a File> + Clone),
    used_ids: &mut Vec<usize>,
) -> Vec<MemoryBlock> {
    match rev_iterator
        .clone()
        .filter(|x| x.length <= length && !used_ids.contains(&x.id))
        .next()
    {
        Some(file) => {
            let mut result = vec![MemoryBlock::File(*file)];
            used_ids.push(file.id);
            if file.length < length {
                result.extend(fill_empty_space(
                    length - file.length,
                    rev_iterator,
                    used_ids,
                ))
            }
            result
        }
        None => vec![MemoryBlock::Empty { length: length }],
    }
}

fn reorder(memory_block: Vec<MemoryBlock>) -> Vec<MemoryBlock> {
    let rev_iterator = memory_block.iter().rev().filter_map(|x| match x {
        MemoryBlock::File(a) => Some(a),
        MemoryBlock::Empty { .. } => None,
    });

    let mut used_ids: Vec<usize> = vec![];

    let a: Vec<MemoryBlock> = memory_block
        .iter()
        .inspect(|x| {dbg!(x);})
        .map(|x| match x {
            MemoryBlock::File { .. } => vec![*x],
            MemoryBlock::Empty { length } => {
                fill_empty_space(*length, &rev_iterator, &mut used_ids)
            }
        })
        .flatten()
        .collect();
    a
}

fn main() {
    let input = include_str!("../input.txt");

    let memory_block: Vec<MemoryBlock> = input
        .chars()
        .filter_map(|x| x.to_digit(10).and_then(|x| Some(x as usize)))
        .enumerate()
        .map(|(inx, x)| {
            if inx as f32 % 2.0 == 0.0 {
                return MemoryBlock::File(File {
                    id: inx / 2,
                    length: x,
                });
            }
            MemoryBlock::Empty { length: x }
        })
        .collect();

    let mut already_counted_ids: Vec<usize> = vec![];
    let mut inx = 0;
    let mut total = 0;

    let a = reorder(memory_block);

    for i in a.iter() {
        for _ in 0..i.length() {
            if let MemoryBlock::File(x) = i {
                if !already_counted_ids.contains(&x.id) {
                    total += x.id * inx
                }
            }
            inx += 1;
        }

        if let MemoryBlock::File(x) = i {
            already_counted_ids.push(x.id)
        }
    }

    println!("{}", total);
}
