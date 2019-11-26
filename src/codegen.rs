use std::fs::File;
use std::io::{self, Write};

use crate::math::Vec2;

pub fn generate_points_code(output_file: &mut File, points: &[Vec2]) -> io::Result<()> {
    if points.is_empty() {
        return Ok(());
    }

    // Move to the start point
    writeln!(output_file, "    turtle.pen_up();")?;
    let first = &points[0];
    writeln!(output_file, "    turtle.go_to(({:.1}, {:.1}));", first.x, first.y)?;
    writeln!(output_file, "    turtle.pen_down();")?;

    // Draw the rest of the points
    for point in &points[1..] {
        writeln!(output_file, "    turtle.go_to(({:.1}, {:.1}));", point.x, point.y)?;
    }

    Ok(())
}
