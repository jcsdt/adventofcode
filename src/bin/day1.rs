use std::collections::HashSet;
use std::io::BufRead;

use aoc::runner::{run, Result};

fn main() -> Result<()> {
    run(|reader| {
        let parsed: Result<_> = reader
            .lines()
            .map(|line| line?.parse::<i32>().map_err(|e| e.into()))
            .collect();

        let vector: Vec<i32> = parsed?;
        let mut input = vector.iter().cycle();

        let mut acc = 0;
        let mut set = HashSet::new();
        loop {
            if !set.insert(acc) {
                break;
            }

            let v = input.next().expect("a cycle should always have a value");
            acc += v;
        }

        Ok(acc)
    })
}
