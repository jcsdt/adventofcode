use aoc::runner::{run, Result};

fn diff(a: &str, b: &str) -> i32 {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() as i32
}

fn main() -> Result<()> {
    run(|content| {
        let input: Vec<&str> = content.split("\n").collect();
        let mut i = 1;
        let (a, b) = loop {
            let mut r = input
                .iter()
                .zip(input.iter().skip(i))
                .filter(|(a, b)| diff(a, b) <= 1);

            match r.next() {
                Some(t) => break t,
                None => (),
            };

            i += 1;
        };

        Ok(a.chars()
            .zip(b.chars())
            .filter(|(x, y)| x == y)
            .map(|(x, _)| x)
            .collect::<String>())
    })
}
