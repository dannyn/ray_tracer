use std::f64;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug)]
struct Vec4 {
    x: f64, 
    y: f64,
    z: f64,
    w: f64,
}

impl  Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 {x: x, y: y, z: z, w: w}
    }

    pub fn vector(x: f64, y: f64, z:f64) -> Vec4 {
        Vec4 {x: x, y: y, z: z, w: 0.0}
    }
    
    pub fn point(x: f64, y: f64, z:f64) -> Vec4 {
        Vec4 {x: x, y: y, z: z, w: 1.0}
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vec4 {
        let m = self.mag();
        Vec4 { x: self.x / m, 
               y: self.y / m, 
               z: self.z / m,
               w: self.w / m}
    }

    pub fn dot(&self, other: &Vec4) -> f64 {
        self.x * other.x +
        self.y * other.y + 
        self.z * other.z + 
        self.w * other.w
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        Vec4::vector(self.y * other.z - self.z * other.y,
                     self.z * other.x - self.x * other.z,
                     self.x * other.y - self.y * other.x)
    }
}

impl  PartialEq for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        ((self.x - other.x).abs() < f64::EPSILON) &&
        ((self.y - other.y).abs() < f64::EPSILON) &&
        ((self.z - other.z).abs() < f64::EPSILON) &&
        ((self.w - other.w).abs() < f64::EPSILON)
    }
}
impl Eq for Vec4 {}

impl Add for Vec4
{
    type Output = Vec4;
    fn  add(self, other: Vec4) -> Vec4 {
        Vec4 { x: self.x + other.x, 
               y: self.y + other.y, 
               z: self.z + other.z, 
               w: self.w + other.w, }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 { x: self.x - other.x, 
               y: self.y - other.y, 
               z: self.z - other.z, 
               w: self.w - other.w, }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, other: f64) -> Vec4 {
        Vec4 { x: self.x * other, 
               y: self.y * other, 
               z: self.z * other, 
               w: self.w * other, }
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 { x: self * other.x, 
               y: self * other.y, 
               z: self * other.z, 
               w: self * other.w, }
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, other: f64) -> Vec4 {
        Vec4 { x: self.x / other, 
               y: self.y / other, 
               z: self.z / other, 
               w: self.w / other, }
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Vec4 {
        Vec4 { x: -self.x, 
               y: -self.y, 
               z: -self.z, 
               w: -self.w, }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction() {
        let v = Vec4::new(1.0, 2.2, 3.0, 0.0);
        let w = Vec4::new(2.0, 0.0, 3.2, 1.0);
        let z = Vec4::new(1.0, 2.2, 3.0, 0.0);
        assert_eq!(v,z);
        assert_ne!(v,w);

        assert_eq!(Vec4::point(4.0, -4.0, 3.0), Vec4::new(4.0, -4.0, 3.0, 1.0));
        assert_eq!(Vec4::vector(4.0, -4.0, 3.0), Vec4::new(4.0, -4.0, 3.0, 0.0));
        assert_ne!(Vec4::vector(4.0, -4.0, 3.0), Vec4::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn operators() {
        assert_eq!(Vec4::new(3.0, -2.0, 5.0, 1.0) + Vec4::new(-2.0, 3.0, 1.0, 0.0), 
                   Vec4::new(1.0,  1.0, 6.0, 1.0));
        assert_eq!(Vec4::new(3.0, -2.0, 5.0, 1.0) - Vec4::new(-2.0, 3.0, 1.0, 0.0), 
                   Vec4::new(5.0, -5.0, 4.0, 1.0));
        assert_eq!(3.0 * Vec4::new(3.0, -2.0, 5.0, 1.0), Vec4::new(9.0, -6.0, 15.0, 3.0));
        assert_eq!(Vec4::new(3.0, -2.0, 5.0, 1.0) * 3.0, Vec4::new(9.0, -6.0, 15.0, 3.0));
        assert_eq!(Vec4::new(4.0, -2.0, 6.0, 1.0) / 2.0, Vec4::new(2.0, -1.0, 3.0, 0.5));
        assert_eq!(-Vec4::new(3.0, -2.0, 5.0, 1.0), Vec4::new(-3.0, 2.0, -5.0, -1.0));
   }
   
   #[test]
   fn fuctions() {
        assert_eq!(Vec4::vector(1.0, 0.0, 0.0).mag(), 1.0);
        assert_eq!(Vec4::vector(0.0, 1.0, 0.0).mag(), 1.0);
        assert_eq!(Vec4::vector(0.0, 0.0, 1.0).mag(), 1.0);
        assert_eq!(Vec4::vector(1.0, 2.0, 3.0).mag(), (14.0f64).sqrt());
        assert_eq!(Vec4::vector(-1.0, -2.0, -3.0).mag(), (14.0f64).sqrt());

        assert_eq!(Vec4::vector(4.0, 0.0, 0.0).normalize(), Vec4::vector(1.0, 0.0, 0.0));
        assert_eq!(Vec4::vector(1.0, 2.0, 3.0).normalize(), Vec4::vector(1.0/(14f64).sqrt(), 2.0/(14f64).sqrt(), 3.0/(14f64).sqrt()));
        assert_eq!(Vec4::vector(1.0, 2.0, 3.0).normalize().mag(), 1.0);

        assert_eq!(Vec4::vector(1.0, 2.0, 3.0).dot(&Vec4::vector(2.0, 3.0, 4.0)), 20.0);
        assert_eq!(Vec4::vector(1.0, 2.0, 3.0).cross(&Vec4::vector(2.0, 3.0, 4.0)), Vec4::vector(-1.0, 2.0, -1.0));
        assert_eq!(Vec4::vector(2.0, 3.0, 4.0).cross(&Vec4::vector(1.0, 2.0, 3.0)), Vec4::vector(1.0, -2.0, 1.0));
    }
}
