//use na::{Vector3, dot};

use super::scene_object::SceneObject;
use super::scene_object::Material;
use super::ray;
use super::transformation::{Transformation, Vector};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub material: Material,
    pub trans: Transformation,
}

impl SceneObject for Sphere {
    fn intersect(&self, r: ray::Ray) -> Vec<ray::Intersection> {

        let sphere_to_ray = r.origin - Vector::new(0.0, 0.0, 0.0);
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * r.dir.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant =  (b * b) - (4.0 * a * c);

        let mut hits = vec!();

        if discriminant >= 0.0 { 
            let t1 = ray::Intersection {
                t: (-b - discriminant.sqrt()) / (2.0 * a),
                object: Box::new(*self),
            };
            let t2 = ray::Intersection { 
                t: (-b + discriminant.sqrt()) / (2.0 * a),
                object: Box::new(*self),
            };
            if t1.t < t2.t { 
                hits.push(t1);
                hits.push(t2);
            } else {
                hits.push(t2);
                hits.push(t1);
            }
        }
        hits
    }

    fn normal(&self, p: Vector) -> Vector {
        let object_normal = (p - Vector::new(0.0, 0.0, 0.0)).normalize();
        //self.transm.try_inverse().unwrap().transpose() * object_normal
        object_normal
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        let m = Material::new();
        Sphere {material: m,
                trans: Transformation::new()}
    }
}

mod sphere_tests {
    use super::*;
    #[test]
    fn intersection_tangent() {
        let r = ray::Ray::new(Vector::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
 
        let hit = ray::get_hit(intersections).unwrap();
        assert_eq!(hit.t, 5.0);
    }

    #[test]
    fn intersection_two_points() {
        let r = ray::Ray::new(Vector::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);

        let hit = ray::get_hit(intersections).unwrap();
        assert_eq!(hit.t, 4.0);
    }

    #[test]
    fn intersection_misses() {
        let r = ray::Ray::new(Vector::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 0);

        let hit = ray::get_hit(intersections);
        assert!(hit.is_none());
    }

    #[test]
    fn intersection_inside() {
        let r = ray::Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);

        let hit = ray::get_hit(intersections).unwrap();
        assert_eq!(hit.t, 1.0);
    }

    #[test]
    fn intersection_behind() {
        let r = ray::Ray::new(Vector::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);

        let hit = ray::get_hit(intersections);
        assert!(hit.is_none());
    }

    #[test]
    fn normals() {
        let s = Sphere::new();
        let s3o3 = (3.0 as f64).sqrt() / 3.0;
        assert_eq!(s.normal(Vector::new(1.0, 0.0, 0.0)), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(s.normal(Vector::new(0.0, 1.0, 0.0)), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(s.normal(Vector::new(0.0, 0.0, 1.0)), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(s.normal(Vector::new(s3o3, s3o3, s3o3)), Vector::new(s3o3, s3o3, s3o3));

        assert_eq!(s.normal(Vector::new(s3o3, s3o3, s3o3)), Vector::new(s3o3, s3o3, s3o3).normalize());
    }
}

