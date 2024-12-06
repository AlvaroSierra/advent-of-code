
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
            Self::Left => (x - 1, y),
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1)
        }
    }
}


fn main() {
    let input = include_bytes!("../input.txt");
    let mut map: Vec<Vec<u8>> = input.split(|&x| x == b'\n').map(|x| x.into_iter().map(|x| *x).collect()).collect();
    
    let a: (usize, usize) = map.iter().enumerate().filter_map(|(inx, string)| string.iter().position(|&x| x == b'^').and_then(|x| Some((inx, x)))).next().unwrap();
    let (mut y, mut x) = a;
    let mut direction = Direction::Up;

    loop {
        let temp_pointer: &mut u8 = map.get_mut(y).unwrap().get_mut(x).unwrap();
        *temp_pointer = b'X';
        let (x_in_front, y_in_front) = direction.next_pos(x, y);
        println!("Looking at {x_in_front}, {y_in_front}");
        let character = map.get(y_in_front).and_then(|row| row.get(x_in_front));
        println!("Character: {}", String::from_utf8(vec![*character.unwrap_or(&13)]).unwrap());
        match character {
            Some(b'#') => {
                direction.next(); 
                println!("{:?}", direction);
            },
            Some(b'X') | Some(b'.') => { 
                println!("Moving forwards");
                x = x_in_front;
                y = y_in_front;
            },
            None => break,
            _ => panic!("Missing character {:?} at {x_in_front}, {y_in_front}", String::from_utf8(vec![*character.unwrap()])),
        }
    }
    let total = map.iter().map(|x| x.iter()).flatten().filter(|&&x| x == b'X').count();

    println!("{}", total);
}
