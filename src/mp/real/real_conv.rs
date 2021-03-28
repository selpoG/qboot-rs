use super::super::mp;

use super::super::integer::Integer;
use super::super::rational::Rational;
use super::real::{Long, Real, ULong, _GLOBAL_RND};

impl From<&Integer> for Real {
    fn from(from: &Integer) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mp::mpfr_set_z(&mut x.data, &from.data, _GLOBAL_RND);
        }
        x
    }
}
impl From<&Rational> for Real {
    fn from(from: &Rational) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mp::mpfr_set_q(&mut x.data, &from.data, _GLOBAL_RND);
        }
        x
    }
}
impl From<f32> for Real {
    fn from(from: f32) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mp::mpfr_set_flt(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}
impl From<f64> for Real {
    fn from(from: f64) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mp::mpfr_set_d(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}
impl From<Long> for Real {
    fn from(from: Long) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mp::mpfr_set_si(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}
impl From<ULong> for Real {
    fn from(from: ULong) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mp::mpfr_set_ui(&mut x.data, from, _GLOBAL_RND);
        }
        x
    }
}

impl From<&Real> for Integer {
    fn from(from: &Real) -> Integer {
        let mut x = Integer::new();
        unsafe {
            mp::mpfr_get_z(&mut x.data, &from.data, _GLOBAL_RND);
        }
        x
    }
}
impl From<&Real> for Rational {
    fn from(from: &Real) -> Rational {
        let mut x = Rational::new();
        unsafe {
            mp::mpfr_get_q(&mut x.data, &from.data);
        }
        x
    }
}
impl From<&Real> for f64 {
    fn from(from: &Real) -> f64 {
        unsafe { mp::mpfr_get_d(&from.data, _GLOBAL_RND) }
    }
}
impl From<&Real> for f32 {
    fn from(from: &Real) -> f32 {
        unsafe { mp::mpfr_get_flt(&from.data, _GLOBAL_RND) }
    }
}
impl From<&Real> for Long {
    fn from(from: &Real) -> Long {
        unsafe { mp::mpfr_get_si(&from.data, _GLOBAL_RND) }
    }
}
impl From<&Real> for ULong {
    fn from(from: &Real) -> ULong {
        unsafe { mp::mpfr_get_ui(&from.data, _GLOBAL_RND) }
    }
}
