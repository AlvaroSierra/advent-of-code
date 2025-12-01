use std::{collections::HashMap, i64};
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

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
    
    let (old_seeds, maps) = deserialize(raw_input);

    let seeds = old_seeds.chunks(2).map(|x| (x[0]..(x[0] + x[1])).collect::<Vec<i64>>()).flatten();


    let built_maps: Vec<Vec<Map>> = maps
        .into_iter()
        .map(|x| x.into_iter().map(|m| Map::new(m)).collect())
        .collect();


    let min_value: Arc<RwLock<i64>> = Arc::new(RwLock::new(i64::MAX));

    println!("Starting computations");
    dbg!(&seeds);
    
    seeds.par_bridge().for_each(|i| {
        let mut val = i;
        for layer in built_maps.iter(){
            val = match layer.iter().find(|x| x.check(val)){
                Some(map) => map.destination(val),
                None => val
            }
        }
        if val < *min_value.read().unwrap() {
            let mut w = min_value.write().unwrap();
            *w = val;
        }
    });

    println!("{:?}", min_value);
}
