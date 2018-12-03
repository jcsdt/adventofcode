#[derive(Debug)]
pub enum ProgramError {
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
