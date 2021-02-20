use super::bindings::mpfr;

use super::real::{Real, _GLOBAL_PREC};

impl Real {
    pub fn _nan() -> Real {
        let mut x: mpfr::__mpfr_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            mpfr::mpfr_init2(&mut x, _GLOBAL_PREC);
        }
        Real { data: x }
    }
}
