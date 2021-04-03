#[allow(dead_code)]
#[allow(improper_ctypes)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod mp;

pub type ULong = ::std::os::raw::c_ulong;
pub type Long = ::std::os::raw::c_long;

mod base;
pub mod integer;
pub mod rational;
pub mod real;

pub use integer::Integer;
pub use rational::Rational;
pub use real::Real;
