extern crate nalgebra as na;
use na::Vector3;
use std::fmt;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    // constructs a new Ray
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        return Ray {
            origin: origin,
            direction: direction,
        };
    }

    // returns ray at a given position t determined by a linear interpolation
    pub fn at(&self, t: f32) -> Vector3<f32> {
        return self.origin + (self.direction * t);
    }

    // returns interpolated color (for the background)
    pub fn color(&self) -> Vector3<f32> {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        // linear color interpolation
        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}

// ray to_string() impl
impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position: {} Target: {}", self.origin, self.direction)
    }
}
