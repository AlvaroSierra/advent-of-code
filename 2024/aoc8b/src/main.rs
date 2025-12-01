use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Point {
    line: isize,
    column: isize,
}

impl<'a> std::ops::Sub<&'a Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &'a Point) -> Self::Output {
        Self::Output {
            line: self.line - rhs.line,
            column: self.column - rhs.column,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl<'a, 'b> std::ops::Sub<&'a Point> for &'b Point {
    type Output = Point;

    fn sub(self, rhs: &'a Point) -> Self::Output {
        Self::Output {
            line: self.line - rhs.line,
            column: self.column - rhs.column,
        }
    }
}

impl std::ops::Mul<isize> for &Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            line: rhs * self.line,
            column: rhs * self.column,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let n_lines = input.lines().count() as isize;
    let columns = input.lines().next().unwrap().chars().count() as isize;

    let mut array = vec![0; n_lines as usize * columns as usize];

    let antenas: HashMap<char, Vec<Point>> = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, character)| match character {
                    '.' => None,
                    antena_char => Some((antena_char, Point {
                        line: line_number as isize,
                        column: column as isize,
                    })),
                })
        })
        .flatten()
        .filter_map(|x| x)
        .fold(HashMap::new(), |mut map, x| {
            map.entry(x.0).or_insert_with(Vec::new).push(x.1);

            array[(x.1.line * columns + x.1.column) as usize] = 1;
            map
        });

    let a = antenas
        .iter()
        .map(|(_, points)| {
            points.iter().map(|x1| {
                points.iter().map(move |x2| {
                    (1..=100).map(move |i| {
                        if x1 == x2 {
                            return Point {
                                line: n_lines,
                                column: columns,
                            };
                        }

                        (x2 * (1 + i)) - &(x1 * i)
                    })
                })
            })
        })
        .flatten()
        .flatten()
        .flatten()
        .filter(|x| x.line >= 0 && x.line < n_lines && x.column >= 0 && x.column < columns)
        .for_each(|x| {
            array[(x.line * columns + x.column) as usize] = 1;
        });

    println!("{:?}", array);
    let a: usize = array.iter().sum();

    println!("{}", a)
}
