use super::transformation::Vector;

use super::canvas::Colour;
use super::ray;

pub trait SceneObject {
    fn intersect(&self,r: ray::Ray) -> Vec<ray::Intersection>;
    fn normal(&self, p: Vector) -> Vector;
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
   pub colour: Colour,
   pub ambient: f64,
   pub diffuse: f64,
   pub specular: f64,
   pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material{colour: Colour::new(1.0, 1.0, 1.0), 
                 ambient: 0.1, 
                 diffuse: 0.9, 
                 specular: 0.9, 
                 shininess: 200.0}
    }
}
