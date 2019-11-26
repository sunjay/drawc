use std::fmt;
use std::io::{self, BufRead};
use std::num::ParseFloatError;
use std::error::Error;

use crate::math::Vec2;

#[derive(Debug)]
pub enum ParseError {
    IOError(io::Error),
    ParseFloatError(ParseFloatError),
    MissingComma {lineno: usize},
    ShouldBePair {lineno: usize},
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IOError(err)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> Self {
        ParseError::ParseFloatError(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        match self {
            IOError(err) => write!(f, "{}", err),
            ParseFloatError(err) => write!(f, "{}", err),
            MissingComma {lineno} => write!(f, "missing comma on line {} of input", lineno),
            ShouldBePair {lineno} => write!(f, "line {} of input was not a pair of numbers", lineno),
        }
    }
}

impl Error for ParseError {}

pub fn read_points() -> Result<Vec<Vec2>, ParseError> {
    let stdin = io::stdin();
    stdin.lock().lines().enumerate().map(|(i, line)| {
        let lineno = i + 1;
        let line = line?;
        let mut parts = line.split(',');

        let x: f64 = parts.next()
            .ok_or(ParseError::MissingComma {lineno})?
            .parse()?;

        let y: f64 = parts.next()
            .ok_or(ParseError::ShouldBePair {lineno})?
            .parse()?;

        if parts.next().is_some() {
            Err(ParseError::ShouldBePair {lineno})?
        }

        Ok(Vec2 {x, y})
    }).collect()
}
