extern crate nalgebra as na;
use crate::mesh::Mesh;
use crate::Ray;
use na::Vector3;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Mesh for Sphere {
    // check for intersection between sphere and ray
    // https://iquilezles.org/www/articles/intersectors/intersectors.htm
    fn intersects(&self, ray: &Ray) -> f32 {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-half_b - (discriminant).sqrt()) / a;
        }
    }
}
