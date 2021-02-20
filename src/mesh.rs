use cgmath::InnerSpace;

use super::scene_object::SceneObject;
use super::scene_object::Material;
use super::ray;

pub struct Triangle {
    vertices: [cgmath::Vector3<f64>; 3],
    normal: cgmath::Vector3<f64>,
    e1: cgmath::Vector3<f64>,
    e2: cgmath::Vector3<f64>,
}

impl Triangle {
    pub fn new(vertices: [cgmath::Vector3<f64>; 3]) -> Triangle {
        Triangle { vertices: vertices, 
                normal: 0, 
                e1: vertices[2] - vertices[0],
                e2: vertices[3] - vertices[0]
    }
}

/*#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub material: Material,
}

impl SceneObject for Sphere {
    fn intersect(&self, r: ray::Ray) -> Vec<ray::Intersection> {

        let sphere_to_ray = r.origin - cgmath::vec3(0.0, 0.0, 0.0);
        let a = cgmath::dot(r.dir, r.dir);
        let b = 2.0 * cgmath::dot(r.dir, sphere_to_ray);
        let c = cgmath::dot(sphere_to_ray, sphere_to_ray) - 1.0;

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

    fn normal(&self, p: cgmath::Vector3<f64>) -> cgmath::Vector3<f64> {
        (p - cgmath::vec3(0.0, 0.0, 0.0)).normalize()
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        let m = Material::new();
        Sphere {material: m}
    }
}

mod sphere_tests {
    use super::*;
    #[test]
    fn intersection_tangent() {
        let r = ray::Ray::new(cgmath::vec3(0.0, 1.0, -5.0), cgmath::vec3(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
 
        let hit = ray::get_hit(intersections).unwrap();
        assert_eq!(hit.t, 5.0);
    }

    #[test]
    fn intersection_two_points() {
        let r = ray::Ray::new(cgmath::vec3(0.0, 0.0, -5.0), cgmath::vec3(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 6.0);

        let hit = ray::get_hit(intersections).unwrap();
        assert_eq!(hit.t, 4.0);
    }

    #[test]
    fn intersection_misses() {
        let r = ray::Ray::new(cgmath::vec3(0.0, 2.0, -5.0), cgmath::vec3(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 0);

        let hit = ray::get_hit(intersections);
        assert!(hit.is_none());
    }

    #[test]
    fn intersection_inside() {
        let r = ray::Ray::new(cgmath::vec3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 1.0)); 
        let s = Sphere::new();
        let intersections = s.intersect(r);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);

        let hit = ray::get_hit(intersections).unwrap();
        assert_eq!(hit.t, 1.0);
    }

    #[test]
    fn intersection_behind() {
        let r = ray::Ray::new(cgmath::vec3(0.0, 0.0, 5.0), cgmath::vec3(0.0, 0.0, 1.0)); 
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
        assert_eq!(s.normal(cgmath::vec3(1.0, 0.0, 0.0)), cgmath::vec3(1.0, 0.0, 0.0));
        assert_eq!(s.normal(cgmath::vec3(0.0, 1.0, 0.0)), cgmath::vec3(0.0, 1.0, 0.0));
        assert_eq!(s.normal(cgmath::vec3(0.0, 0.0, 1.0)), cgmath::vec3(0.0, 0.0, 1.0));
        assert_eq!(s.normal(cgmath::vec3(s3o3, s3o3, s3o3)), cgmath::vec3(s3o3, s3o3, s3o3));

        assert_eq!(s.normal(cgmath::vec3(s3o3, s3o3, s3o3)), cgmath::vec3(s3o3, s3o3, s3o3).normalize());
    }
}
*/
