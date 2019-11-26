//! Generates and prints points along a cubic polynomial a + bx + cx^2 + dx^3

/// Represenxs a cubic polynomial a + bx + cx^2 + dx^3
pub struct Cubic {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Cubic {
    /// Returns the value of the cubic polynomial at this value of x
    pub fn at(&self, x: f64) -> f64 {
        let Self {a, b, c, d} = self;
        a + b*x + c*x*x + d*x*x*x
    }
}

fn main() {
    let curve = Cubic {
        a: 6.0,
        b: 25.0,
        c: 3.0,
        d: -4.0,
    };

    // The minimum and maximum values along the x axis
    let min_x = -3.0;
    let max_x = 4.0;

    // The number of points to generate
    let samples = 100;
    // The amount to scale up the image so it is visible in the window
    let scale_x = 80.0;
    let scale_y = 4.0;

    for i in 0..samples {
        let x = (i as f64 / samples as f64) * (max_x - min_x) + min_x;
        let y = curve.at(x);

        println!("{},{}", x * scale_x, y * scale_y);
    }
}
