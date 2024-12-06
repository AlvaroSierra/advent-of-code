use rayon::prelude::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn next_pos(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::Right => (x + 1, y),
            Self::Left => (x.overflowing_sub(1).0, y),
            Self::Up => (x, y.overflowing_sub(1).0),
            Self::Down => (x, y + 1),
        }
    }
}

#[derive(Clone)]
struct Map {
    data: Vec<Vec<u8>>,
    modified_val: Option<(usize, usize)>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if let Some(vals) = self.modified_val {
            if y == vals.0 && x == vals.1 {
                return Some(b'#');
            }
        }
        self.data.get(y).and_then(|row| row.get(x)).copied()
    }

    fn mod_point(&mut self, x: usize, y: usize) {
        self.modified_val = Some((x, y));
    }

    fn start_xy(&self) -> (usize, usize) {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(inx, string)| {
                string
                    .iter()
                    .position(|&x| x == b'^')
                    .and_then(|x| Some((inx, x)))
            })
            .next()
            .unwrap()
    }

    fn set_val(&mut self, x: usize, y: usize, to: u8) {
        let temp_pointer: &mut u8 = self.data.get_mut(y).unwrap().get_mut(x).unwrap();
        *temp_pointer = to;
    }

    fn count_total(&self) -> usize {
        self.data
        .iter()
        .map(|x| x.iter())
        .flatten()
        .filter(|&&x| x == b'X')
        .count()
    }

    fn walk(&mut self) -> Option<usize>{
        let (mut y, mut x) = self.start_xy();
        let mut direction = Direction::Up;
        let mut finished = false;

        for _ in 0..135200 {
            self.set_val(x, y, b'X');
            let (x_in_front, y_in_front) = direction.next_pos(x, y);
            let character = self.get(x_in_front, y_in_front);
            match character {
                Some(b'#') => {
                    direction.next();
                }
                Some(b'X') | Some(b'.') => {
                    x = x_in_front;
                    y = y_in_front;
                }
                None => {finished = true; break},
                _ => panic!(
                    "Missing character {:?} at {x_in_front}, {y_in_front}",
                    String::from_utf8(vec![character.unwrap()])
                ),
            }
        }
        
        match finished {
            true => Some(self.count_total()),
            false => None
        }
    }
}

fn main() {
    let input = include_bytes!("../input.txt");
    let map = Map {
        data: input
            .split(|&x| x == b'\n')
            .map(|x| x.into_iter().map(|x| *x).collect())
            .collect(),
        modified_val: None,
    };
    
    let total: usize = (0..130).into_par_iter().map(|y| {
        (0..130).into_iter().map(|x|
            {
                let mut temp_map = map.clone();
                temp_map.mod_point(x, y);
                temp_map.walk()
            }
        ).filter(|x| x.is_none()).count()
    }).sum();

    println!("{:?}", total);
}
