mod parser;
mod math;

use std::fmt;
use std::error::Error;

use parser::read_points;

struct DisplayError {
    err: Box<dyn Error>,
}

impl<T: Into<Box<dyn Error>>> From<T> for DisplayError {
    fn from(err: T) -> Self {
        Self {err: err.into()}
    }
}

impl fmt::Debug for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {err} = self;
        write!(f, "{}", err)
    }
}

fn main() -> Result<(), DisplayError> {
    let points = read_points()?;

    println!("{:?}", points);

    Ok(())
}
