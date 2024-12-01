use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    start: i64,
    end: i64,
    delta: i64,
}

impl Map {
    fn new(line: &str) -> Map {
        let mut iterator = line.split(" ");
        let destination_start: i64 = iterator.next().unwrap().parse().unwrap();
        let source_start: i64 = iterator.next().unwrap().parse().unwrap();
        let amount: i64 = iterator.next().unwrap().parse().unwrap();

        Self {
            start: source_start,
            end: source_start + amount,
            delta: source_start - destination_start,
        }
    }

    fn destination(&self, source: i64) -> i64 {
        source - self.delta
    }

    fn check(&self, source: i64) -> bool {
        source >= self.start && source < self.end
    }
}

fn deserialize(input: &str) -> (Vec<i64>, Vec<Vec<&str>>) {
    let mut a = input.split("\r\n\r\n");
    let seeds: Vec<i64> = a.next().unwrap()[7..].split(" ").map(|x| x.parse().unwrap()).collect();

    let b: Vec<Vec<&str>> = a
        .map(|x| {
            let mut iterators = x.split("\r\n");
            iterators.next();
            iterators.filter(|x| *x != "").collect()
        })
        .collect();

    (seeds, b)
}

fn main() {
    let raw_input = include_str!("../input2.txt");
    
    let (seeds, maps) = deserialize(raw_input);

    let built_maps: Vec<Vec<Map>> = maps
        .into_iter()
        .map(|x| x.into_iter().map(|m| Map::new(m)).collect())
        .collect();

    let mut results: Vec<i64> = Vec::new();

    for i in seeds.into_iter(){
        let mut val = i;
        for layer in built_maps.iter(){
            val = match layer.iter().find(|x| x.check(val)){
                Some(map) => map.destination(val),
                None => val
            }
        }
        results.push(val); 
    }

    println!("{:?}", results.into_iter().min());
}
