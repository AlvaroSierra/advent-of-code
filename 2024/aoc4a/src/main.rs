use std::time::Instant;

const substring: &'static str = "XMAS";
const backwards_substring: &'static str = "SAMX";

fn count_in_line(line: &str) -> usize {
    let forwards = line
        .as_bytes()
        .windows(substring.len())
        .filter(|&w| w == substring.as_bytes())
        .count();

    let backwards = line
        .as_bytes()
        .windows(backwards_substring.len())
        .filter(|&w| w == backwards_substring.as_bytes())
        .count();

    forwards + backwards
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn horizontal_matches(lines: Vec<&str>) -> usize {
    lines.into_iter().map(|x| count_in_line(x)).sum()
}

fn vertical_matches(window: Vec<Vec<char>>) -> usize {
    let columns: Vec<String> = transpose(window)
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect();

    horizontal_matches(columns.iter().map(std::ops::Deref::deref).collect())
}

fn shift(mut line: Vec<char>, offset: isize, direction: isize, to_length: usize) -> Vec<char> {
    if direction > 0 {
        for _ in 0..(offset) {
            line.remove(0);
        }

        while to_length < line.len() {
            line.pop();
        }
    }

    if direction < 0 {
        for _ in 0..(offset) {
            line.pop();
        }

        while to_length < line.len() {
            line.remove(0);
        }
    }

    return line;
}

fn diagonal(window: Vec<Vec<char>>, offset: isize) -> usize {
    let mut total = 0;
    let col_length = window[0].len() - 3;

    for inx in 2..window.len() - 1 {
        let mut local_window: Vec<Vec<char>> = vec![];

        for row_index in (1..=2).rev() {
            let a = window[inx - row_index].clone();
            let shift_by = 2 - row_index;
            let shifted = shift(a, shift_by as isize, offset, col_length);
            local_window.push(shifted);
        }

        local_window.push(shift(window[inx].clone(), 2, offset, col_length));

        let a = window[inx + 1].clone();
        local_window.push(shift(a, 0, offset * -1, col_length));
        let vertical_matches = vertical_matches(local_window.clone());

        total += vertical_matches;
    }

    total
}

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");

    let horizontal_matches: usize = horizontal_matches(input.lines().collect());
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let vertical_matches = vertical_matches(matrix.clone());
    let diagonal_matches = diagonal(matrix.clone(), 1) + diagonal(matrix, -1);
    
    println!("{:?}", start.elapsed());
    println!("{horizontal_matches}, {vertical_matches}, {diagonal_matches}");
}

