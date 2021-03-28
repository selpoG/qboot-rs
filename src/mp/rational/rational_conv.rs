use super::super::mp;

use super::super::Integer;
use super::super::{Long, ULong};
use super::rational::Rational;

impl From<&Integer> for Rational {
    fn from(from: &Integer) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpq_set_z(&mut r.data, &from.data);
        }
        r
    }
}
impl From<ULong> for Rational {
    fn from(from: ULong) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpq_set_ui(&mut r.data, from, 1);
        }
        r
    }
}
impl From<Long> for Rational {
    fn from(from: Long) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpq_set_si(&mut r.data, from, 1);
        }
        r
    }
}
impl From<f64> for Rational {
    fn from(from: f64) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpq_set_d(&mut r.data, from);
        }
        r
    }
}

impl From<&Rational> for Integer {
    fn from(from: &Rational) -> Integer {
        let mut n = Integer::new();
        unsafe { mp::__gmpz_cdiv_q(&mut n.data, &from.data._mp_num, &from.data._mp_den) }
        n
    }
}
impl From<&Rational> for ULong {
    fn from(from: &Rational) -> ULong {
        ULong::from(&Integer::from(from))
    }
}
impl From<&Rational> for Long {
    fn from(from: &Rational) -> Long {
        Long::from(&Integer::from(from))
    }
}
impl From<&Rational> for f64 {
    fn from(from: &Rational) -> f64 {
        unsafe { mp::__gmpq_get_d(&from.data) }
    }
}
