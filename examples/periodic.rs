//! Generates and prints points along a periodic function

/// Represenxs a periodic function: y = a*sin(b*(x - c)) + d
pub struct Periodic {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Periodic {
    /// Returns the value of the periodic function at this value of x
    pub fn at(&self, x: f64) -> f64 {
        let Self {a, b, c, d} = self;
        a*(b*(x - c)).sin() + d
    }
}

fn main() {
    let curve = Periodic {
        a: 100.0,
        b: 1.0/15.0,
        c: 50.0,
        d: -25.0,
    };

    // The minimum and maximum values along the x axis
    let min_x = -300.0;
    let max_x = 300.0;

    // The number of points to generate
    let samples = 150;
    // The amount to scale up the image so it is visible in the window
    let scale_x = 1.0;
    let scale_y = 1.0;

    for i in 0..samples {
        let x = (i as f64 / samples as f64) * (max_x - min_x) + min_x;
        let y = curve.at(x);

        println!("{},{}", x * scale_x, y * scale_y);
    }
}
