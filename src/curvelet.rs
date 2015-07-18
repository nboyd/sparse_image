use std::simd::f64x2;

use cubature::*;
use common::*;
//hardcoded for now. shoudl go into Curvelet...
fn base_wavelet(f64x2(x,y) : f64x2) -> f64 {
    256.0*((-(x*x + y*y)).exp())
}

#[derive(Clone,Debug)]
pub struct Curvelet {
    pub parabolic_scaling : f64,
    rotation_angle : f64,
    translation : f64x2,
    cos_theta : f64,
    sin_theta : f64,
    scaling : f64x2,
}

impl Curvelet {
    pub fn new(s : f64,
    t : f64,
    translation : f64x2) -> Curvelet {
        Curvelet{
            parabolic_scaling : s,
            rotation_angle : t,
            translation : translation,
            cos_theta : t.cos(),
            sin_theta : t.sin(),
            scaling : f64x2(s, s.sqrt()),
        }
    }

    fn apply(&self, p : f64x2) -> f64x2{
        //translate
        let f64x2(x_t,y_t) = p - self.translation;
        //rotate
        let x_r = self.cos_theta*x_t - self.sin_theta*y_t;
        let y_r = self.cos_theta*y_t + self.sin_theta*x_t;
        //scale
        self.scaling*f64x2(x_r,y_r)
    }
}

pub fn render(curvelets : &[(f64, &Curvelet)], p : &ImageParameters,  result : &mut [f64], ip : &IntegrationParameters) {
    assert!(p.len() == result.len());
    let summed  = |p : &f64x2| {
        let mut sum = 0.0;
        for &(w,t) in curvelets.iter(){
            let new_coord = t.apply(p.clone());
            //println!("{:?}", w);
            sum += w*base_wavelet(new_coord);
        }
        sum
    };
    let w = p.width;
    let h = p.height;
    let scalex = 1.0 / w as f64;
    let scaley = 1.0 / h as f64;

    let mut min = [0.0;2];
    let mut max = [1.0;2];

    for px in 0..w  {
        for py in 0..h {
            min[0] = (px as f64) * scalex;
            min[1] = (py as f64) * scaley;
            max[0] = ((px + 1) as f64) * scalex;
            max[1] = ((py + 1) as f64) * scaley;
            let (r,_) = integrate_2d(&summed, &min, &max, ip);
            let linear_index = py*w + px;
            result[linear_index] = r;
        }
    }
}
