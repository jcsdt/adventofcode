use std::collections::{HashMap, HashSet};

use aoc::runner::{run, Result};

fn count_letters(string: &str) -> HashSet<i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for l in string.chars() {
        let counter = counts.entry(l).or_insert(0);
        *counter += 1;
    }

    counts.drain().map(|(_, v)| v).collect::<HashSet<i32>>()
}

fn select_two_and_three(counts: HashSet<i32>) -> (i32, i32) {
    let two = if counts.contains(&2) { 1 } else { 0 };
    let three = if counts.contains(&3) { 1 } else { 0 };

    (two, three)
}

fn main() -> Result<()> {
    run(|content| {
        let (x, y) = content
            .split("\n")
            .map(|s| count_letters(&s))
            .map(|letters| select_two_and_three(letters))
            .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        Ok(x * y)
    })
}
