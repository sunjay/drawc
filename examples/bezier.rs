//! Generates and prints points along a bezier curve
//!
//! https://en.wikipedia.org/wiki/B%C3%A9zier_curve

type Vec2 = vek::Vec2<f64>;

struct CubicBezier {
    point0: Vec2,
    point1: Vec2,
    point2: Vec2,
    point3: Vec2,
}

impl CubicBezier {
    /// Returns the value of this curve at the given point
    pub fn at(&self, t: f64) -> Vec2 {
        let &Self {point0, point1, point2, point3} = self;
        // Copying the formula from here verbatim:
        // https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Cubic_B%C3%A9zier_curves
        (1.0 - t).powi(3) * point0
            + 3.0*(1.0 - t)*(1.0 - t)*t*point1
            + 3.0*(1.0 - t)*t*t*point2
            + t.powi(3)*point3
    }
}

fn main() {
    let curve = CubicBezier {
        point0: Vec2 {x: -200.0, y: -100.0},
        point1: Vec2 {x: -100.0, y: 400.0},
        point2: Vec2 {x: 100.0, y: -500.0},
        point3: Vec2 {x: 300.0, y: 200.0},
    };

    let samples = 100;
    for i in 0..samples {
        let t = i as f64 / samples as f64;
        let point = curve.at(t);
        println!("{},{}", point.x, point.y);
    }
}
