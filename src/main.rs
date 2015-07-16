extern crate libc;
extern crate image;


use std::fs::File;
use std::path::Path;

mod curvelet;
mod cubature;
mod matrix;
use matrix::*;

fn integrand(x : &[f64]) -> f64 {
    let r = x[0]*x[0] + x[1]*x[1];
    256.0*((-r).exp())
}

fn test_borrow(l : &[CurveletTransform]){
    let z = l[1].parabolic_scaling;
    println!("{}",z);
}

fn main() {
    let t1 = CurveletTransform::new(25.0,0.5,0.25,0.25);
    let t2 = CurveletTransform::new(24.0,0.5,0.25,0.25);

    let L = vec![t1,t2];

    test_borrow(&L);

    println!("{}", t2);


    // let mut M = Matrix::new(vec![0.0;6],2,3);
    //
    // println!("{}", M);
    // M[(1,1)] = 17.0;
    // println!("{}", M);
    // println!("{}", M[(1,1)]);
    // let Z = M.clone();
    // println!("{}",M.add(Zres));

    let imgx = 1000;
    let imgy = 1000;

    let scalex = 1.0 / imgx as f64;
    let scaley = 1.0 / imgy as f64;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut min = [0.0;2];
    let mut max = [1.0;2];

    print!("Processing... ");
    let mut t = CurveletTransform::new(25.0,0.5,0.25,0.25);
    for (px, py, pixel) in imgbuf.enumerate_pixels_mut() {
        min[0] = (px as f64) * scalex;
        min[1] = (py as f64) * scaley;
        max[0] = ((px + 1) as f64) * scalex;
        max[1] = ((py + 1) as f64) * scaley;

        let (r,err) = cubature::integrate_fn_over_box(|p| integrand(&(t.apply(p))), &min, &max, 100,
               1.0, 1.0);
        //correct for pixel pitch (?)
        *pixel = image::Luma([(imgx as f64 * imgy as f64 * r) as u8]);
    }
    println!("done.");


    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    image::ImageLuma8(imgbuf).save(fout, image::PNG);

}
