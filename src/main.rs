extern crate clap;
extern crate approx;
extern crate nalgebra as na;

use clap::{Arg, App};

mod canvas;
mod ray;
mod sphere;
mod scene_object;
mod light;
mod transformation;

use scene_object::SceneObject;
use transformation::Vector;

#[derive(Clone, Copy)]
struct Wall {
    z: f64, 
    size: f64, 
    pixel_size: f64, 
}

fn raytrace(x: f64, y: f64, source: Vector, wall: Wall, sphere: sphere::Sphere) -> canvas::Colour {
    let position = Vector::new(x, y, wall.z);
    let r = ray::Ray::new(source, (position-source).normalize() );
    
    match ray::get_hit(sphere.intersect(r)) {
        Some(hit) => {
            let point = r.position(hit.t); 
            let light = light::Light {intensity: 1.0, pos: Vector::new(-10.0, 10.0, -10.0)};
            let normal = sphere.normal(point);
            let c = light::lighting(sphere.material, point, light, r.dir, normal);
            return c;
        },
        None => { }
    }
    canvas::Colour::new(0.0, 0.0, 0.0)
}

fn run(filename: &str, canvas_size: usize) {

    let mut cvs = canvas::Canvas::new(canvas_size, canvas_size);
    let source = Vector::new(0.0, 0.0, -5.0);
    let sphere = sphere::Sphere::new();

    let wall = Wall {z: 10.0, 
                     size: 7.0, 
                     pixel_size: 7.0 / (canvas_size as f64) };
    let half = wall.size / 2.0; 

    for y in 0..(canvas_size - 1) {
        let world_y =  half - wall.pixel_size * (y as f64);
        for x in 0..(canvas_size - 1) {
            let world_x =  half - wall.pixel_size * (x as f64);
            let c =  raytrace(world_x, world_y, source, wall, sphere);
            cvs.write(x, y, c);
        }
    }
    cvs.save_ppm(filename.to_string());
}

fn main() {
    let matches = App::new("Ray Tracer")
                           .arg(Arg::with_name("filename")
                                .short("f")
                                .long("filename")
                                .value_name("FILE")
                                .required(true)
                                .takes_value(true))
                           .get_matches();
    let filename = matches.value_of("filename").unwrap();
    run(filename, 1000);    
}
