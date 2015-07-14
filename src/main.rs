extern crate libc;
extern crate num;
extern crate image;


use std::fs::File;
use std::path::Path;
use num::complex::Complex;

mod cubature;

struct CurveletTransform {
    parabolic_scaling : f64,
    rotation_angle : f64,
    translation_x : f64,
    translation_y : f64,
    cos_theta : f64,
    sin_theta : f64,
    sqrt_a : f64,
}

impl CurveletTransform {
    pub fn new(s : f64,
    t : f64,
    x : f64,
    y : f64) -> CurveletTransform {
        CurveletTransform{
            parabolic_scaling : s,
            rotation_angle : t,
            translation_x : x,
            translation_y : y,
            cos_theta : t.cos(),
            sin_theta : t.sin(),
            sqrt_a : s.sqrt(),
        }
    }
}

trait CoordinateTransform {
    fn apply(&self,  p : &[f64]) -> [f64;2];
}

impl CoordinateTransform for CurveletTransform{
    fn apply(&self, p : &[f64]) -> [f64;2]{
        let x = p[0];
        let y = p[1];
        //translate
        let x_t = x - self.translation_x;
        let y_t = y - self.translation_y;
        //rotate
        let x_r = self.cos_theta*x_t - self.sin_theta*y_t;
        let y_r = self.cos_theta*y_t + self.sin_theta*x_t;
        //scale
        [x_r*self.sqrt_a,y_r*self.parabolic_scaling]
    }
}

fn integrand(x : &[f64]) -> f64 {
    let r = x[0]*x[0] + x[1]*x[1];
    10000.0*256.0*((-r).exp())
}

fn main() {
    let t = CurveletTransform::new(25.0,0.5,0.25,0.25);
    println!("{:#?}", t.apply(&[0.0,0.0]));
    let imgx = 100;
    let imgy = 100;

    let scalex = 1.0 / imgx as f64;
    let scaley = 1.0 / imgy as f64;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut min = [0.0;2];
    let mut max = [1.0;2];
    for (px, py, pixel) in imgbuf.enumerate_pixels_mut() {
        min[0] = (px as f64) * scalex;
        min[1] = (py as f64) * scaley;
        max[0] = ((px + 1) as f64) * scalex;
        max[1] = ((py + 1) as f64) * scaley;

        let (r,err) = cubature::integrate_fn_over_box(|p| integrand(&(t.apply(p))), &min, &max, 100,
               1.0, 1.0);
        *pixel = image::Luma([r as u8]);

    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    image::ImageLuma8(imgbuf).save(fout, image::PNG);

}
