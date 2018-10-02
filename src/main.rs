mod canvas;

fn main() {
    let mut cvs = canvas::Canvas::new(5,3); 

    cvs.write(0,0, canvas::Colour::new(1.0, 0.0, 0.0));
    cvs.write(0,1, canvas::Colour::new(0.0, 1.0, 0.0));
    cvs.write(0,2, canvas::Colour::new(0.0, 0.0, 1.0));

    let header = cvs.ppm_header();
    let data = cvs.ppm_data();

    println!("{}{}", header, data);

    cvs.save_ppm("here.ppm".to_string());
}
