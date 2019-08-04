// Chapter 2, write a vec3 class

// TODO: Implement: make_unit_vector(), *,  -, -=, *=, scalar * vec, scalar *= vec, /, /=, salar /= vec, dot, cross    


// f32 is the trait which provides `sqrt()`
use std::f32;

// operator traits (operator overloading)
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug, PartialEq)]
pub struct Vec3
{
  pub data:Vec<f32>
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 { data:vec![0f32; 3] } // initialize with 0, three items
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn length(&self) -> f32 {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }
}

// Operator overrides

// +
impl Add for Vec3 
{
    type Output = Self;

     fn add(self, other:Self) -> Self {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] + other.data[0];
        return_vec3.data[1] = self.data[1] + other.data[1];
        return_vec3.data[2] = self.data[2] + other.data[2];

        return_vec3
    }
}

// +=
impl AddAssign for Vec3 
{
     fn add_assign(&mut self, other: Self) {
        (*self).data[0] += other.data[0];
        (*self).data[1] += other.data[1];
        (*self).data[2] += other.data[2];
    }     
}
