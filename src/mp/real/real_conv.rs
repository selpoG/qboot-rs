use super::mpfr;

use super::real::{Long, Real, ULong, _GLOBAL_RND};

impl From<f32> for Real {
    fn from(from: f32) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_set_flt(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}
impl From<f64> for Real {
    fn from(from: f64) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_set_d(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}

impl From<&Real> for f64 {
    fn from(from: &Real) -> f64 {
        unsafe { mpfr::mpfr_get_d(&from.data, _GLOBAL_RND) }
    }
}

impl From<Long> for Real {
    fn from(from: Long) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_set_si(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}
impl From<ULong> for Real {
    fn from(from: ULong) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_set_ui(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}
