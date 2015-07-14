extern crate libc;
extern crate num;
extern crate image;


// use std::fs::File;
// use std::path::Path;
// use num::complex::Complex;

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
    fn apply(&self, x : f64, y : f64) -> (f64,f64);
}

impl CoordinateTransform for CurveletTransform{
    fn apply(&self, x : f64, y : f64) -> (f64,f64){
        //translate
        let x_t = x + self.translation_x;
        let y_t = y + self.translation_y;
        //rotate
        let x_r = self.cos_theta*x_t - self.sin_theta*y_t;
        let y_r = self.cos_theta*y_t + self.sin_theta*x_t;
        //scale
        (x_r*self.sqrt_a,y_r*self.parabolic_scaling)
    }
}

fn square(x : &[f64]) -> f64 {
    x[0]*(x[0]+1E-3).ln()
}

fn main() {
    let t = CurveletTransform::new(2.0,0.5,1.0,0.0);
    println!("{:#?}", t.apply(1.0,1.0));
    // let max_iterations = 256u16;
    //
    // let imgx = 5000;
    // let imgy = 5000;
    //
    // let scalex = 4.0 / imgx as f32;
    // let scaley = 4.0 / imgy as f32;
    //
    // // Create a new ImgBuf with width: imgx and height: imgy
    // let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    //
    // // Iterate over the coordiantes and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let cy = y as f32 * scaley - 2.0;
    //     let cx = x as f32 * scalex - 2.0;
    //
    //     let mut z = Complex::new(cx, cy);
    //     let c = Complex::new(-0.4, 0.6);
    //
    //     let mut i = 0;
    //
    //     for t in (0..max_iterations) {
    //         if z.norm() > 2.0 {
    //             break
    //         }
    //         z = z * z + c;
    //         i = t;
    //     }
    //
    //     // Create an 8bit pixel of type Luma and value i
    //     // and assign in to the pixel at position (x, y)
    //     *pixel = image::Luma([i as u8]);
    //
    // }
    //
    //
    // // Save the image as “fractal.png”
    // let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    //
    // // We must indicate the image’s color type and what format to save as
    // image::ImageLuma8(imgbuf).save(fout, image::PNG);

    // fn hcubature(fdim : c_uint,
    //         f : extern fn(ndim : c_uint, x : *mut c_double, data : *mut c_void, fval : *mut c_double) -> c_int,
    //         fdata : *mut c_void,
    //         dim : c_uint, xmin : *mut c_double,  xmax : *mut c_double,
    //         maxEval : size_t, reqAbsError : c_double, reqRelError : c_double,
    //         norm : libc::uint8_t,
    //         val : *mut c_double, err : c_double) -> i32;
     let xmin = [0.0;1];
     let xmax = [1.0;1];
     let (r,err) = cubature::integrate_fn_over_box(square, &xmin, &xmax, 100,
            0.01, 0.01);
    //   let r = unsafe{ cubature::pcubature(1, testFn, &mut fdata[0] as *mut _ as *mut c_void,1,&mut xmin[0],&mut xmax[0], 10, 1.0,1.0, 0, &mut fval, &mut err);};
     println!("integral is {:?} to err: {:?}", r, err);
}
