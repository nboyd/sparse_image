#![feature(core_simd)]
mod cubature;
extern crate libc;
extern crate image;

use std::path::Path;

mod common;
use common::*;
mod curvelet;
use curvelet::*;
use std::simd::f64x2;

fn main() {
    let t1 = Curvelet::new(25.0,0.5,f64x2(0.25,0.25));
    let t2 = Curvelet::new(24.0,1.2,f64x2(0.75,0.25));
    let t3 = Curvelet::new(1000.0,1.2,f64x2(0.75,0.75));
    let t4 = Curvelet::new(100.0,1.2,f64x2(0.75,0.1));
    let t5 = Curvelet::new(100.0,1.2,f64x2(0.10,0.2));
    let t6 = Curvelet::new(100.0,1.2,f64x2(0.3,0.1));

    let h = 1000;
    let w = 1000;
    let image_params = ImageParameters{height:h, width:w};
    let int_params = cubature::IntegrationParameters::new();
    let mut image_buf = vec![0.0; h*w];
    //t2.parabolic_scaling.powf(0.75)
    render(&[(1.0,&t1), (1.0,&t2), (1.0, &t3), (1.0,&t4), (1.0,&t5), (1.0, &t6)], &image_params,&mut image_buf,&int_params);
    //println!("{:#?}", image_buf);
    let mut max_intensity = std::f64::NEG_INFINITY;
    for i in image_buf.iter(){
        max_intensity = if max_intensity > *i { max_intensity } else {*i};
    }
    println!("{}",max_intensity);
    let scale_factor = 254.5/max_intensity;
    let final_image = image_buf.iter().map(|i| ( (*i)*scale_factor) as u8).collect::<Vec<u8>>();
    image::save_buffer(&Path::new("output.png"), &final_image, h as u32, w as u32, image::Gray(8));
}
