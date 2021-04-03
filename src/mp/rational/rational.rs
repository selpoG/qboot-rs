use super::super::mp;

use super::super::Integer;
use super::super::{Long, ULong};
use crate::algebra::matrix::Ring;

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
    pub fn sgn(&self) -> i32 {
        if self.data._mp_num._mp_size < 0 {
            -1
        } else if self.data._mp_num._mp_size > 0 {
            1
        } else {
            0
        }
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        if !self.data._mp_num._mp_d.is_null() {
            unsafe { mp::__gmpq_clear(&mut self.data) }
        }
    }
}

impl Clone for Rational {
    fn clone(&self) -> Rational {
        let mut x = Rational::new();
        unsafe {
            mp::__gmpq_set(&mut x.data, &self.data);
        }
        x
    }
}
impl Ring for Rational {
    fn zero() -> Self {
        Rational::new()
    }
    fn iszero(&self) -> bool {
        self.sgn() == 0
    }
    fn _add(&self, rhs: &Self) -> Self {
        self + rhs
    }
    fn _iadd(&mut self, rhs: &Self) {
        *self += rhs
    }
    fn _isub(&mut self, rhs: &Self) {
        *self *= rhs
    }
    fn _neg(&self) -> Self {
        -self
    }
    fn _eq(&self, rhs: &Self) -> bool {
        return self == rhs;
    }
}
