use super::super::mp;

use super::super::Integer;
use super::super::{Long, ULong};

pub struct Rational {
    pub data: mp::__mpq_struct,
}

impl Rational {
    pub fn new() -> Rational {
        let mut x: mp::__mpq_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            mp::__gmpq_init(&mut x);
        }
        Rational { data: x }
    }
    pub fn from_pair(num: &Integer, den: &Integer) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpz_set(&mut r.data._mp_num, &num.data);
            mp::__gmpz_set(&mut r.data._mp_den, &den.data);
            mp::__gmpq_canonicalize(&mut r.data);
        }
        r
    }
    pub fn from_pair_l(num: Long, den: ULong) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpq_set_si(&mut r.data, num, den);
            mp::__gmpq_canonicalize(&mut r.data);
        }
        r
    }
    pub fn from_pair_ul(num: ULong, den: ULong) -> Rational {
        let mut r = Rational::new();
        unsafe {
            mp::__gmpq_set_ui(&mut r.data, num, den);
            mp::__gmpq_canonicalize(&mut r.data);
        }
        r
    }
    pub fn reset(&mut self) {
        if !self.data._mp_num._mp_d.is_null() {
            unsafe { mp::__gmpq_clear(&mut self.data) }
        }
        self.data._mp_num._mp_d = std::ptr::null_mut();
        self.data._mp_den._mp_d = std::ptr::null_mut();
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        if !self.data._mp_num._mp_d.is_null() {
            unsafe { mp::__gmpq_clear(&mut self.data) }
        }
    }
}
