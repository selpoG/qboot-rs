use super::super::gmp;

use super::super::integer::Integer;
use super::rational::{Long, Rational, ULong};

impl From<ULong> for Rational {
    fn from(from: ULong) -> Rational {
        let mut x: gmp::__mpq_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpq_init(&mut x);
            gmp::__gmpq_set_ui(&mut x, from, 1);
        }
        Rational { data: x }
    }
}
impl From<Long> for Rational {
    fn from(from: Long) -> Rational {
        let mut x: gmp::__mpq_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpq_init(&mut x);
            gmp::__gmpq_set_si(&mut x, from, 1);
        }
        Rational { data: x }
    }
}
impl From<f64> for Rational {
    fn from(from: f64) -> Rational {
        let mut x: gmp::__mpq_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpq_init(&mut x);
            gmp::__gmpq_set_d(&mut x, from);
        }
        Rational { data: x }
    }
}

impl From<&Rational> for Integer {
    fn from(from: &Rational) -> Integer {
        let mut n = Integer::new();
        unsafe { gmp::__gmpz_cdiv_q(&mut n.data, &from.data._mp_num, &from.data._mp_den) };
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
        unsafe { gmp::__gmpq_get_d(&from.data) }
    }
}
