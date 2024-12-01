use std::time::Instant;

const REQUIRED_SYMBOLS: usize = 1;

fn is_symbol(character: &char) -> bool {
    /// Checks if the character is considered a
    if *character == '.' {
        return false;
    }

    if character.is_numeric() {
        return false;
    }

    true
}

fn check_position(data: &Vec<Vec<char>>, n_lookup: usize, m_lookup: usize) -> bool {
    if let Some(a) = data.get(n_lookup) {
        if let Some(b) = a.get(m_lookup) {
            if is_symbol(b) {
                return true;
            }
        }
    }
    false
}

fn check_around(data: &Vec<Vec<char>>, n: usize, m_start: usize, m_end: usize) -> bool {
    let mut total_symbols: usize = 0;

    for n_lookup in n.saturating_sub(1)..=n.saturating_add(1) {
        for m_lookup in m_start.saturating_sub(1)..=m_end.saturating_add(1) {
            if check_position(data, n_lookup, m_lookup) {
                total_symbols += 1
            }
            if total_symbols >= REQUIRED_SYMBOLS {
                return true;
            }
        }
    }
    false
}

fn main() {
    let start = Instant::now();
    let mut a = 0;

    // Turn the input into a matrix for manipulation.
    let data: Vec<Vec<char>> = include_str!("../input2.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    for (n, i) in data.iter().enumerate() {
        let mut iterator = i.iter().enumerate();
        while let Some((m, j)) = iterator.next() {
            if j.is_numeric() {
                let mut number_end = m.saturating_add(1);

                while data
                    .get(n)
                    .unwrap()
                    .get(number_end)
                    .unwrap_or(&'a')
                    .is_numeric()
                {
                    number_end += 1;
                    iterator.next();
                }

                if check_around(&data, n, m, number_end - 1) {
                    let number = data[n][m..number_end]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    a += number;
                } else {
                    let number = data[n][m..number_end]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();

                    // println!("{}", number);
                }
            }
        }
    }
    let m = start.elapsed();
    println!("{:?}", m);
    println!("{}", a);
}
