use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use crate::error::ProgramError;

pub type Result<T> = std::result::Result<T, ProgramError>;

pub fn run<F, D: Display>(computation: F) -> Result<()>
where
    F: FnOnce(BufReader<File>) -> Result<D>,
{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;

    let r = computation(BufReader::new(f))?;

    println!("{}", r);

    Ok(())
}
