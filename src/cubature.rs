extern crate libc;

use std::simd::f64x2;
use std::slice;
use libc::{c_int,c_uint,c_void,c_double,size_t};

pub struct IntegrationParameters{
    max_evals : u64,
    abs_err : f64,
    rel_err : f64,
}
impl IntegrationParameters{
    pub fn new() -> IntegrationParameters {
        IntegrationParameters{
            max_evals : 1,
            abs_err : 0.01,
            rel_err : 0.01,
        }
    }
}

extern "C" {
    fn pcubature(fdim : c_uint,
            f : extern fn(ndim : c_uint, x : *mut c_double, data : *const c_void, fdim : c_uint, fval : *mut c_double) -> c_int,
            fdata : *mut c_void,
            dim : c_uint, xmin : *const c_double,  xmax : *const c_double,
            maxEval : size_t, reqAbsError : c_double, reqRelError : c_double,
            norm : u8,
            val : *mut c_double, err : *mut  c_double) -> i32;
}

pub fn integrate_2d<F : Fn(&f64x2) -> f64>(f: F, xmin : &[f64], xmax : &[f64], p : &IntegrationParameters)
    -> (f64,f64) {
        integrate_fn_over_box(|input| f(&f64x2(input[0], input[1])), xmin,xmax,p) //slow?
}

pub fn integrate_fn_over_box<F: Fn(&[f64]) -> f64>(f: F, xmin : &[f64], xmax : &[f64], p : &IntegrationParameters)
    -> (f64,f64) {
  let user_data = &f as *const _ as *mut c_void;
  let dim = xmin.len();
  let mut val = 0.0;
  let mut err = 0.0;
  let max_evals = p.max_evals;
  let abs_err = p.abs_err;
  let rel_err = p.rel_err;

  unsafe {
      pcubature(1,function_wrapper::<F>, user_data, dim as u32, xmin.as_ptr(),
        xmax.as_ptr(), max_evals, abs_err, rel_err,1, &mut val, &mut err);
  };

  extern "C" fn function_wrapper<F: Fn(&[f64]) -> f64>(ndim : u32, x : *mut f64, closure : *const c_void, _ : u32, fval : *mut c_double) -> i32{
    let closure = closure as *mut F;
    unsafe {
        let input = slice::from_raw_parts(x, ndim as usize);
        *fval = (*closure)(input);
        return 0;
    }
  }
  (val,err)
}
