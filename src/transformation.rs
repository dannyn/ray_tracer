use na::{Vector3, Vector4, Matrix4};
use std::ops::{Add, Sub, Mul, Neg};
use std::f64;
use approx::{abs_diff_eq, relative_eq};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    v: Vector4<f64>,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            v: Vector4::new(x, y, z, 1.0),
        }
    }

    pub fn x(&self) -> f64 {
        self.v.x
    }

    pub fn y(&self) -> f64 {
        self.v.y
    }

    pub fn z(&self) -> f64 {
        self.v.z
    }

    pub fn norm(&self) -> f64 {
        let v = Vector3::new(self.v.x, self.v.y, self.v.z);
        v.norm()
    }
    
    pub fn dot(&self, other: &Vector) -> f64 {
        let v = Vector3::new(self.v.x, self.v.y, self.v.z);
        let u = Vector3::new(other.v.x, other.v.y, other.v.z);
        Vector3::dot(&v, &u)
    }

    pub fn normalize(&self) -> Vector {
        let v = Vector3::new(self.v.x, self.v.y, self.v.z).normalize();
        Vector::new(v.x, v.y, v.z)
    }

    pub fn reflect(self, normal: &Vector) -> Vector {
        self - *normal * 2.0 * self.dot(normal)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        let v = self.v + other.v;
        Vector::new(v.x, v.y, v.z)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        let v = self.v - other.v;
        Vector::new(v.x, v.y, v.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, other: f64) -> Vector {
        Vector::new(self.v.x * other, self.v.y * other, self.v.z * other)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector::new(self * other.v.x, self * other.v.y, self * other.v.z)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(-self.v.x, -self.v.y, -self.v.z)
    }
}

impl approx::AbsDiffEq for Vector {
    type Epsilon = f64;
    fn default_epsilon() -> f64 {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Vector, epsilon: f64) -> bool {
        abs_diff_eq!(self.v, other.v)
    }
}

impl approx::RelativeEq for Vector {
    fn default_max_relative() -> f64 {
        f64::EPSILON
    }

    fn relative_eq(&self, other: &Vector, epsilon: f64, max_relative: f64) -> bool {
        relative_eq!(self.v, other.v)
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn operations() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let u = Vector::new(3.0, 2.0, 1.0);
        assert!(relative_eq!(v + u, Vector::new(4.0, 4.0, 4.0)));
        assert!(relative_eq!(v - u, Vector::new(-2.0, 0.0, 2.0)));
        assert!(relative_eq!(v * 2.0, Vector::new(2.0, 4.0, 6.0)));
        assert!(relative_eq!(2.0 * v, Vector::new(2.0, 4.0, 6.0)));
        assert!(relative_eq!(-v, Vector::new(-1.0, -2.0, -3.0)));
    }

    #[test]
    fn reflection() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new((2.0 as f64).sqrt()/2.0, (2.0 as f64).sqrt()/2.0, 0.0);
        
        let reflected = v.reflect(&n);
        assert!(relative_eq!(reflected, Vector::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn norm() {
        assert!(relative_eq!(Vector::new(1.0, 0.0, 0.0).norm(), 1.0));
        assert!(relative_eq!(Vector::new(0.0, 1.0, 0.0).norm(), 1.0));
        assert!(relative_eq!(Vector::new(0.0, 0.0, 1.0).norm(), 1.0));
        assert!(relative_eq!(Vector::new(1.0, 2.0, 3.0).norm(), 14.0f64.sqrt()));
        assert!(relative_eq!(Vector::new(-1.0, -2.0, -3.0).norm(), 14.0f64.sqrt()));
    }

    #[test]
    fn normalize() {
        assert!(relative_eq!(Vector::new(4.0, 0.0, 0.0).normalize(), Vector::new(1.0, 0.0, 0.0)));
    }

    fn dot() {
        let u = Vector::new(1.0, 2.0, 3.0);
        let v = Vector::new(2.0, 3.0, 4.0);
        assert!(relative_eq!(u.dot(&v), 20.0));
    }
}

#[derive(Debug, Clone, Copy)] 
pub struct Transformation {
    transm: Matrix4<f64>,
    invm: Matrix4<f64>,
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation { 
            transm: Matrix4::identity(), 
            invm: Matrix4::identity(), }
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Transformation {
        let t = Vector3::new(x, y, z);
        let m = Matrix4::new_nonuniform_scaling(&t);
        Transformation { 
            transm: self.transm * m,
            invm: (self.transm * m).try_inverse().unwrap(),
        }
    }

    pub fn shear(&self, x: f64, y: f64, z: f64) {
    }

    pub fn rotate(&self, x: f64, y: f64, z: f64) {
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) -> Transformation {
        let t = Vector3::new(x, y, z);
        let m = Matrix4::new_translation(&t);
        Transformation { 
            transm: self.transm * m,
            invm: (self.transm * m).try_inverse().unwrap(),
        }
    }
}

impl Mul for Transformation {
    type Output = Transformation;
    fn mul(self, other: Transformation) -> Transformation {
        Transformation {transm: self.transm * other.transm,
                        invm: self.invm * other.invm}
    }
}

impl Mul<Vector4<f64>> for Transformation {
    type Output = Vector4<f64>;
    fn mul(self, other: Vector4<f64>) -> Vector4<f64> {
        self.transm * other
    }
}

impl Mul<Vector3<f64>> for Transformation {
    type Output = Vector3<f64>;
    fn mul(self, other: Vector3<f64>) -> Vector3<f64> {
        let v = Vector4::new(other.x, other.y, other.z, 1.0);
        let prod = self.transm * v;
        Vector3::new(prod.x, prod.y, prod.z)
    }
}

impl Mul<Vector> for Transformation {
    type Output = Transformation;
    fn mul(self, other: Vector) -> Transformation {
        Transformation::new()
    }
}

impl Mul<Transformation> for Vector {
    type Output = Transformation;
    fn mul(self, other: Transformation) -> Transformation {
        Transformation::new()
    }
}


#[cfg(test)]
mod transformation_tests {
    use super::*;

    #[test]
    fn construction() {
        let m = Transformation::new();
        assert!(relative_eq!(m.transm, Matrix4::identity()));
        assert!(relative_eq!(m.invm, Matrix4::identity()));
    }

    #[test]
    fn scale() {
        let m = Transformation::new().scale(1.0, 2.0, 3.0);
        let n = Matrix4::new(1.0f64, 0.0, 0.0, 0.0, 
                             0.0,    2.0, 0.0, 0.0, 
                             0.0,    0.0, 3.0, 0.0,
                             0.0,    0.0, 0.0, 1.0);
        assert!(relative_eq!(m.transm, n));
    }

    #[test]
    fn multiplication() {
        let m = Transformation::new();
        let n = Transformation::new();
        assert!(relative_eq!( (m*n).transm, Matrix4::identity()));
    }

}
