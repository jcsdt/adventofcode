use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum ProgramError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl From<std::io::Error> for ProgramError {
    fn from(err: std::io::Error) -> ProgramError {
        ProgramError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for ProgramError {
    fn from(err: std::num::ParseIntError) -> ProgramError {
        ProgramError::ParseError(err)
    }
}

type Result<T> = std::result::Result<T, ProgramError>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;

    let parsing: Result<Vec<i32>> = BufReader::new(f).lines()
        .map(|line| line?.parse::<i32>().map_err(|e| e.into()))
        .collect();

    let result: i32 = parsing?.iter().sum();

    println!("{}", result);

    Ok(())
}
