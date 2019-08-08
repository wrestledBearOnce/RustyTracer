// Chapter 2, write a vec3 class

// TODO: Implement: scalar * vec, scalar *= vec, salar /= vec, cross    


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

    pub fn ctor(x:f32, y:f32, z:f32) -> Self {
        Vec3 { data:vec![x, y, z] }
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

    pub fn make_unit_vector(&self) -> Self
    {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] / self.length();
        return_vec3.data[1] = self.data[1] / self.length();
        return_vec3.data[2] = self.data[2] / self.length();
        return_vec3
    }

    pub fn dot(&self, other:Vec3) -> f32
    {
        self.data[0] * other.data[0] + self.data[1] * other.data[1] + self.data[2] * other.data[2]     
    }

    pub fn cross(&self, other:Vec3) -> Self
    {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        return_vec3.data[1] = -self.data[0] * other.data[2] + self.data[2] * other.data[0];
        return_vec3.data[2] = self.data[0] * other.data[1] - self.data[1] * other.data[0];
        return_vec3      
    }

    pub fn add(&self, other:f32) -> Self
    {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] + other;
        return_vec3.data[1] = self.data[1] + other;
        return_vec3.data[2] = self.data[2] + other;
        return_vec3
    }

    pub fn mul(&self, other:f32) -> Self
    {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] * other;
        return_vec3.data[1] = self.data[1] * other;
        return_vec3.data[2] = self.data[2] * other;
        return_vec3
    }

    pub fn copy(&self) -> Self
    {
        let mut return_vec3 = Vec3::new();
        return_vec3.data = self.data.clone();
     
        return_vec3
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


// +
impl Sub for Vec3 
{
    type Output = Self;

     fn sub(self, other:Self) -> Self {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] - other.data[0];
        return_vec3.data[1] = self.data[1] - other.data[1];
        return_vec3.data[2] = self.data[2] - other.data[2];

        return_vec3
    }
}

// +=
impl SubAssign for Vec3 
{
     fn sub_assign(&mut self, other: Self) {
        (*self).data[0] -= other.data[0];
        (*self).data[1] -= other.data[1];
        (*self).data[2] -= other.data[2];
    }     
}

// *
impl Mul for Vec3 
{
    type Output = Self;

     fn mul(self, other:Self) -> Self {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] * other.data[0];
        return_vec3.data[1] = self.data[1] * other.data[1];
        return_vec3.data[2] = self.data[2] * other.data[2];

        return_vec3
    }
}

// *=
impl MulAssign for Vec3 
{
     fn mul_assign(&mut self, other: Self) {
        (*self).data[0] *= other.data[0];
        (*self).data[1] *= other.data[1];
        (*self).data[2] *= other.data[2];
    }     
}

// *
impl Div for Vec3 
{
    type Output = Self;

     fn div(self, other:Self) -> Self {
        let mut return_vec3 = Vec3::new(); 
        return_vec3.data[0] = self.data[0] / other.data[0];
        return_vec3.data[1] = self.data[1] / other.data[1];
        return_vec3.data[2] = self.data[2] / other.data[2];

        return_vec3
    }
}

// *=
impl DivAssign for Vec3 
{
     fn div_assign(&mut self, other: Self) {
        (*self).data[0] /= other.data[0];
        (*self).data[1] /= other.data[1];
        (*self).data[2] /= other.data[2];
    }     
}

