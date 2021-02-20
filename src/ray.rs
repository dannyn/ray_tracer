use super::scene_object::SceneObject;
use super::transformation::Vector;

use approx::relative_eq;


pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn SceneObject>,
}


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub dir: Vector,
}


pub fn get_hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    for i in intersections {
        if i.t >= 0.0 {
            return Some(i);
        }
    }
    None
}

impl Ray {
    pub fn new(origin: Vector, dir: Vector) -> Ray {
        Ray { origin: origin, dir: dir }
    }

    pub fn position(&self, t: f64) -> Vector { 
        self.origin + self.dir * t
    }
}

#[cfg(test)]
mod ray_tests {
    use super::*;

    #[test]
    fn construction() {
        let o = Vector::new(1.0, 2.0, 3.0);
        let d = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);
        assert!(relative_eq!(r.origin, o));
        assert!(relative_eq!(r.dir, d));
    }

    #[test]
    fn position() {
        let r = Ray::new(Vector::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert!(relative_eq!(r.position(0.0), Vector::new(2.0, 3.0, 4.0)));
        assert!(relative_eq!(r.position(1.0), Vector::new(3.0, 3.0, 4.0)));
        assert!(relative_eq!(r.position(-1.0), Vector::new(1.0, 3.0, 4.0)));
        assert!(relative_eq!(r.position(2.5), Vector::new(4.5, 3.0, 4.0))); 
    }
}
