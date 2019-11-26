use std::fs::File;
use std::io::{self, Write};

use crate::math::Vec2;

pub fn generate_points_code(output_file: &mut File, points: &[Vec2]) -> io::Result<()> {
    if points.is_empty() {
        return Ok(());
    }

    // Turtle starts at the origin facing towards north
    let init_pos = Vec2 {x: 0.0, y: 0.0};
    let init_dir = Vec2 {x: 0.0, y: 1.0};

    let mut current_pos = points[0];
    // Move to the start position without drawing a line
    writeln!(output_file, "    turtle.pen_up();")?;
    let mut current_dir = generate_move(output_file, init_pos, init_dir, current_pos)?;
    writeln!(output_file, "    turtle.pen_down();")?;

    // Draw the rest of the points
    for &target_pos in &points[1..] {
        let next_dir = generate_move(output_file, current_pos, current_dir, target_pos)?;

        current_pos = target_pos;
        current_dir = next_dir;
    }

    Ok(())
}

/// Generates code to move the turtle to the given target position.
///
/// Uses the current position and direction to calculate how to do that.
///
/// Returns the new direction of the turtle (not normalized)
fn generate_move(
    output_file: &mut File,
    current_pos: Vec2,
    current_dir: Vec2,
    target_pos: Vec2,
) -> io::Result<Vec2> {
    let target_dir = target_pos - current_pos;
    let angle = current_dir.angle_between(target_dir).to_degrees();

    writeln!(output_file, "    turtle.left({:.5});", angle)?;
    writeln!(output_file, "    turtle.forward({:.5});", target_dir.magnitude())?;

    Ok(target_dir)
}
