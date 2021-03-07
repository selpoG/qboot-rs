#[allow(dead_code)]
#[allow(improper_ctypes)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod mpfr;

mod _real;
pub mod real;
mod real_conv;
mod real_io;
mod real_ops;

pub use real::*;
