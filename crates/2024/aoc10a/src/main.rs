use std::{collections::HashSet, time::Instant};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Pos {
    line: usize,
    column: usize,
}

const N_COLS: usize = 45 + 1;
const N_LINES: usize = 45;

impl Pos {
    #[inline]
    fn array_point(&self) -> usize {
        self.line * N_COLS + self.column
    }
}

const INCREASE_BY: u8 = 1;




fn calc(map: &[u8], coord: &Pos) -> HashSet<usize> {
    let current_val = map[coord.array_point()];

    if current_val == 9 + 48 {
        let mut a = HashSet::new();
        a.insert(coord.array_point());
        return a
    };

    let a: HashSet<usize> = [
        Pos {
            line: coord.line + 1,
            column: coord.column,
        },
        Pos {
            line: coord.line.overflowing_sub(1).0,
            column: coord.column,
        },
        Pos {
            line: coord.line,
            column: coord.column.overflowing_sub(1).0,
        },
        Pos {
            line: coord.line,
            column: coord.column + 1,
        },
    ]
    .iter()
    .filter(|new_coord| new_coord.line < N_LINES && new_coord.column < N_COLS - 1)
    .filter(|new_coord| current_val + INCREASE_BY == map[new_coord.array_point()])
    .map(|new_coord| calc(map, new_coord))
    .flatten()
    .collect();

    a
}

fn main() {
    let map = include_bytes!("../input.txt");
    let start = Instant::now();

    let a: usize = map
        .split(|&x| x == b'\n')
        .enumerate()
        .map(|(line, x)| {
            x.iter()
                .enumerate()
                .filter(|(_, x)| **x == b'0')
                .map(move |(column, _)| Pos { line, column })
        })
        .flatten()
        .par_bridge()
        .map(|coord| calc(map, &coord).len())
        .sum();
    
    println!("{:?}", start.elapsed());

    println!("{}", a)
}
