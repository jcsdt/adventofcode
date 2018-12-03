use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

use aoc::error::ProgramError;

type Result<T> = std::result::Result<T, ProgramError>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;

    let parsed: Result<_> = BufReader::new(f).lines()
        .map(|line| line?.parse::<i32>().map_err(|e| e.into()))
        .collect();
    
    let vector: Vec<i32> = parsed?;
    let mut input = vector.iter().cycle();

    let mut acc = 0;
    let mut set = HashSet::new();
    let result: &i32 = loop {
        if !set.insert(acc) {
            break &acc;
        }

        let v = input.next().expect("a cycle should always have a value");
        acc += v;
    };

    println!("{}", result);

    Ok(())
}
