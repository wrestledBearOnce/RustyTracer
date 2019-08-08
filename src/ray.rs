
use crate::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray
{
    pub A:Vec3,
    pub B:Vec3
}

impl Ray {
    pub fn new() -> Self {
        Ray { A : Vec3::new(), B : Vec3::new() }
    }

    pub fn ctor(a:Vec3, b:Vec3) -> Self {
        Ray {A : a, B : b}     
    }

    pub fn origin(&self) -> &Vec3 {
        &self.A
    }

    pub fn direction(&self) -> &Vec3 {
        &self.B
    }

    pub fn point_at_parameter(&self, t:f32) -> Vec3 {          
        let return_var = self.A.copy() + self.B.mul(t);
        return_var
        
    }
}