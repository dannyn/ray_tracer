use super::canvas::Colour;
use super::scene_object::Material;
use super::transformation::Vector;

pub struct Light {
    pub intensity: f64,
    pub pos: Vector,
}

pub fn lighting(m: Material, p: Vector, l: Light, eye: Vector, normal: Vector) -> Colour {
    let effective = m.colour * l.intensity; 
    let lightv = (l.pos -p).normalize();
    let ambient = effective * m.ambient;
    let light_dot_normal = lightv.dot(&normal);

    let mut diffuse = Colour::new(0.0, 0.0, 0.0);
    let mut specular = Colour::new(0.0, 0.0, 0.0);
    if light_dot_normal >= 0.0 {
        diffuse = effective * m.diffuse * light_dot_normal;
        let reflect = (-lightv).reflect(&normal); //  helpers::reflect(-lightv, normal);
        let reflect_dot_eye = reflect.dot(&eye).powi(m.shininess as i32);
        if reflect_dot_eye > 0.0 {
            let s = l.intensity * m.specular * reflect_dot_eye;
            specular  = Colour::new(s, s, s);
        }
    }
    ambient + diffuse + specular
}


#[cfg(test)]
mod lighting_tests {
    use super::*;

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::new();
        let p = Vector::new(0.0, 0.0, 0.0);
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let l = Light{intensity: 1.0,
                      pos: Vector::new(0.0, 0.0, -10.0)};
        let result = lighting(m, p, l, eye, normal);
        assert_eq!(result, Colour::new(1.9, 1.9, 1.9)); 
    }
}

