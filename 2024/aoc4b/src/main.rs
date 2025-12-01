use std::time::Instant;

fn val_at_index(window: &Vec<&str>, row_inx: usize, col_inx: usize) -> char {
    if let Some(val) = window.get(row_inx).and_then(|x| x.chars().nth(col_inx)) {
        return val;
    }
    panic!("Should never reach this point")
}

fn main() {
    let start = Instant::now();

    let input: &str = include_str!("../input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let n_cols = lines[0].len();
    let mut total = 0;

    for row_inx in 1..lines.len() - 1 {
        for col_inx in 1..n_cols - 1 {
            if val_at_index(&lines, row_inx, col_inx) == 'A' {
                total += match (
                    val_at_index(&lines, row_inx - 1, col_inx - 1),
                    val_at_index(&lines, row_inx + 1, col_inx - 1),
                    val_at_index(&lines, row_inx - 1, col_inx + 1),
                    val_at_index(&lines, row_inx + 1, col_inx + 1),
                ) {
                    ('M', 'M', 'S', 'S') => 1,
                    ('S', 'S', 'M', 'M') => 1,
                    ('M', 'S', 'M', 'S') => 1,
                    ('S', 'M', 'S', 'M') => 1,
                    _ => 0,
                }
            }
        }
    }
    println!("{:?}", start.elapsed());
    println!("{}", total)
}
