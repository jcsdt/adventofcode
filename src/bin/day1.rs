use std::collections::HashSet;

use aoc::runner::{run, Result};

fn main() -> Result<()> {
    run(|content| {
        let parsed: Result<_> = content
            .split("\n")
            .map(|line| line.parse::<i32>().map_err(|e| e.into()))
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
