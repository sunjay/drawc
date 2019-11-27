//! Generates and prints points along a piecewise linear function

/// Represents a linear function: y = mx + b
pub struct Linear {
    m: f64,
    b: f64,
}

impl Linear {
    /// Returns the value of the linear function at this value of x
    pub fn at(&self, x: f64) -> f64 {
        let Self {m, b} = self;
        m*x + b
    }
}

/// Represenxs a piecewise linear function
pub struct Piecewise {
    /// Provides values for all x before the first interval covered by `pieces`
    start: Linear,
    /// The functions for all x greater than the provided value xi.
    /// That is, for each tuple (xi, f), we only evaluate f if it was the last value for which
    /// x >= xi.
    ///
    /// The vector is required to be ordered in increasing value of xi.
    pieces: Vec<(f64, Linear)>,
}

impl Piecewise {
    /// Returns the value of the piecewise function at this value of x
    pub fn at(&self, x: f64) -> f64 {
        let Self {start, pieces} = self;
        let mut selected = start;
        for (value, piece) in pieces {
            if x < *value {
                break;
            }
            selected = piece;
        }

        selected.at(x)
    }
}

fn main() {
    let curve = Piecewise {
        start: Linear {m: -1.0, b: -3.0},
        pieces: vec![
            (-3.0, Linear {m: 1.0, b: 3.0}),
            (0.0, Linear {m: -2.0, b: 3.0}),
            (3.0, Linear {m: 0.5, b: -4.5}),
            (6.0, Linear {m: 0.0, b: -1.5})
        ],
    };

    // The minimum and maximum values along the x axis
    let min_x = -5.0;
    let max_x = 10.0;

    // The number of points to generate
    let samples = 150;
    // The amount to scale up the image so it is visible in the window
    let scale_x = 40.0;
    let scale_y = 40.0;

    let offset_x = -100.0;

    for i in 0..samples {
        let x = (i as f64 / samples as f64) * (max_x - min_x) + min_x;
        let y = curve.at(x);

        println!("{},{}", x * scale_x + offset_x, y * scale_y);
    }
}
