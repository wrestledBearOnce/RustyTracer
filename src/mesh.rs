use crate::Ray;

pub trait Mesh
{
    fn intersects(&self, ray:&Ray) -> f32;
}