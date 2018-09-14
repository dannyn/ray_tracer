use std::fs;
use std::f64;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
pub struct Colour {
    pub r: f64, 
    pub g: f64, 
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour {r: r, g: g, b: b}
    }

    pub fn rgb_string(&self) -> String {
        format!("{} {} {}", (self.r * 255.0) as i32, 
                            (self.g * 255.0) as i32, 
                            (self.b * 255.0) as i32)
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Colour) -> bool {
        ((self.r - other.r).abs() < f64::EPSILON) &&
        ((self.g - other.g).abs() < f64::EPSILON) &&
        ((self.b - other.b).abs() < f64::EPSILON)
    }
}
impl Eq for Colour {}

impl Add for Colour
{
    type Output = Colour;
    fn  add(self, other: Colour) -> Colour {
        Colour { r: self.r + other.r, 
                 g: self.g + other.g, 
                 b: self.b + other.b}
    }
}

impl Sub for Colour {
    type Output = Colour;
    fn sub(self, other: Colour) -> Colour{
        Colour { r: self.r - other.r, 
                 g: self.g - other.g, 
                 b: self.b - other.b}
      }
}

impl Mul for Colour {
    type Output = Colour;
    fn mul(self, other: Colour) -> Colour{
        Colour { r: self.r * other.r, 
                 g: self.g * other.g, 
                 b: self.b * other.b}
      }
}

impl Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, other: f64) -> Colour {
        Colour { r: self.r * other, 
                 g: self.g * other, 
                 b: self.b * other}
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;
    fn mul(self, other: Colour ) -> Colour {
        Colour { r: self * other.r, 
                 g: self * other.g, 
                 b: self * other.b}
    }
}

#[cfg(test)]
mod colour_tests {
    use super::*;

    #[test]
    fn operators() {
        assert_eq!(Colour::new(0.9, 0.6, 0.75) + Colour::new(0.7, 0.1, 0.25), Colour::new(1.6, 0.7, 1.0));
        assert_eq!(Colour::new(0.9, 0.6, 0.75) - Colour::new(0.7, 0.1, 0.25), Colour::new(0.2, 0.5, 0.5));
        assert_eq!(Colour::new(0.2, 0.3, 0.4) * 2.0, Colour::new(0.4, 0.6, 0.8));
        assert_eq!(2.0 * Colour::new(0.2, 0.3, 0.4), Colour::new(0.4, 0.6, 0.8));
        assert_eq!(Colour::new(1.0, 0.2, 0.4) * Colour::new(0.9, 1.0, 0.1), Colour::new(0.9, 0.2, 0.04));
        assert_eq!(Colour::new(1.0, 0.0, 0.5).rgb_string(), "255 0 127");
    }
}


#[derive(Debug)]
pub struct Canvas {
    height: usize, 
    width: usize,
    pub data: Vec<Vec<Colour>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            height: height, 
            width: width,
            data: vec![vec![Colour::new(0.0, 0.0, 0.0); height]; width]
        } 
    }
    
    pub fn write(&mut self, x: usize, y: usize, c: Colour) {
        self.data[x][y] = c.clone();
    }

    pub fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n256\n", self.width, self.height)
    }

    pub fn ppm_data(&self) -> String {
        let mut ppm_str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                ppm_str.push_str(&self.data[x][y].rgb_string());
                ppm_str.push_str("\n");
            }
        }
        ppm_str
    }

    pub fn save_ppm(&self, filename: String) {
        let data = format!("{}{}", self.ppm_header(), self.ppm_data());
        fs::write(filename, data).expect("Unable to write file");
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn construction() {
        let m = vec![vec![Colour::new(0.0, 0.0, 0.0); 5]; 5];
        let mut canvas = Canvas::new(5,5); 
        assert_eq!(m, canvas.data);

        canvas.write(2,2, Colour::new(1.0, 0.0, 0.0));
        assert_eq!(canvas.data[2][2], Colour::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn output_ppm() {
        let mut canvas = Canvas::new(5,3); 
        let ppm_str = "0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n255 0 0\n0 0 0\n0 0 0\n";

        canvas.write(2,2, Colour::new(1.0, 0.0, 0.0));

        let header = canvas.ppm_header();
        let data = canvas.ppm_data();

        assert_eq!(header, "P3\n5 3\n256\n");
        assert_eq!(data, ppm_str);

    }
}
