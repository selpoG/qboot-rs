use super::super::mp;

use super::real::{Real, _GLOBAL_PREC};

impl Real {
    pub fn _nan() -> Real {
        let mut x: mp::__mpfr_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            mp::mpfr_init2(&mut x, _GLOBAL_PREC);
        }
        Real { data: x }
    }
}
