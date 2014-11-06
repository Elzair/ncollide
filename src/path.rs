extern crate "nalgebra" as na;
extern crate ncollide;

use na::Pnt3;
use ncollide::procedural::path::{PolylinePath, PolylinePattern, StrokePattern, ArrowheadCap};
use ncollide::procedural;

fn main() {
    let control_points = [
        Pnt3::new(0.0f32, 1.0, 0.0),
        Pnt3::new(2.0f32, 4.0, 2.0),
        Pnt3::new(2.0f32, 1.0, 4.0),
        Pnt3::new(4.0f32, 4.0, 6.0),
        Pnt3::new(2.0f32, 1.0, 8.0),
        Pnt3::new(2.0f32, 4.0, 10.0),
        Pnt3::new(0.0f32, 1.0, 12.0),
        Pnt3::new(-2.0f32, 4.0, 10.0),
        Pnt3::new(-2.0f32, 1.0, 8.0),
        Pnt3::new(-4.0f32, 4.0, 6.0),
        Pnt3::new(-2.0f32, 1.0, 4.0),
        Pnt3::new(-2.0f32, 4.0, 2.0),
    ];

    // Setup the path.
    let bezier   = procedural::bezier_curve(control_points, 100);
    let mut path = PolylinePath::new(&bezier);

    // Setup the pattern.
    let start_cap   = ArrowheadCap::new(1.5f32, 2.0, 0.0);
    let end_cap     = ArrowheadCap::new(2.0f32, 2.0, 0.5);
    let pattern     = procedural::unit_circle(100);
    let mut pattern = PolylinePattern::new(&pattern, true, start_cap, end_cap);

    // Stroke!
    let _ = pattern.stroke(&mut path);
}
