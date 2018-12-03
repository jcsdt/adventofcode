use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

use crate::error::ProgramError;

pub type Result<T> = std::result::Result<T, ProgramError>;

pub fn run<F, D: Display>(computation: F) -> Result<()>
where
    F: FnOnce(&str) -> Result<D>,
{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let r = computation(content.trim())?;

    println!("{}", r);

    Ok(())
}
