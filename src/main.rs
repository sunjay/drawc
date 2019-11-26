mod parser;
mod math;
mod codegen;

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::error::Error;

use parser::read_points;
use codegen::generate_points_code;

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

    let mut output_file = File::create("test.rs")?;
    writeln!(output_file, "use turtle::Turtle;")?;
    writeln!(output_file, "")?;
    writeln!(output_file, "fn main() {{")?;
    writeln!(output_file, "    let mut turtle = Turtle::new();")?;

    generate_points_code(&mut output_file, &points)?;

    writeln!(output_file, "}}")?;

    Ok(())
}
