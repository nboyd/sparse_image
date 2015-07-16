use std::simd::f64x2;
type Point = f64x2;

struct CurveletTransform {
    parabolic_scaling : f64,
    rotation_angle : f64,
    translation : Point,
    cos_theta : f64,
    sin_theta : f64,
    scaling : Point,
}

impl CurveletTransform {
    pub fn new(s : f64,
    t : f64,
    translation : Point) -> CurveletTransform {
        CurveletTransform{
            parabolic_scaling : s,
            rotation_angle : t,
            translation : translation
            cos_theta : t.cos(),
            sin_theta : t.sin(),
            scaling : Point(a, a.sqrt()),
        }
    }
}

trait CoordinateTransform {
    fn apply(&self,  p : Point) -> Point;
}

impl CoordinateTransform for CurveletTransform{
    fn apply(&self, p : Point) -> Point{
        //translate
        let Point(x_t,y_t) = p + self.translation;
        //rotate
        let x_r = self.cos_theta*x_t - self.sin_theta*y_t;
        let y_r = self.cos_theta*y_t + self.sin_theta*x_t;
        //scale
        self.scaling*Point(x_r,y_r_)
    }
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

}
