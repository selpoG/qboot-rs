use super::super::gmp;

use super::super::integer::Integer;

pub type ULong = ::std::os::raw::c_ulong;
pub type Long = ::std::os::raw::c_long;

pub struct Rational {
    pub data: gmp::__mpq_struct,
}

impl Rational {
    pub fn new() -> Rational {
        let mut x: gmp::__mpq_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpq_init(&mut x);
        }
        Rational { data: x }
    }
    pub fn from_pair(num: &Integer, den: &Integer) -> Rational {
        let mut r = Rational::new();
        unsafe {
            gmp::__gmpz_set(&mut r.data._mp_num, &num.data);
            gmp::__gmpz_set(&mut r.data._mp_den, &den.data);
            gmp::__gmpq_canonicalize(&mut r.data);
        }
        r
    }
    pub fn from_pair_l(num: Long, den: ULong) -> Rational {
        let mut r = Rational::new();
        unsafe {
            gmp::__gmpq_set_si(&mut r.data, num, den);
            gmp::__gmpq_canonicalize(&mut r.data);
        }
        r
    }
    pub fn from_pair_ul(num: ULong, den: ULong) -> Rational {
        let mut r = Rational::new();
        unsafe {
            gmp::__gmpq_set_ui(&mut r.data, num, den);
            gmp::__gmpq_canonicalize(&mut r.data);
        }
        r
    }
    pub fn reset(&mut self) {
        if !self.data._mp_num._mp_d.is_null() {
            unsafe { gmp::__gmpq_clear(&mut self.data) }
        }
        self.data._mp_num._mp_d = std::ptr::null_mut();
        self.data._mp_den._mp_d = std::ptr::null_mut();
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        if !self.data._mp_num._mp_d.is_null() {
            unsafe { gmp::__gmpq_clear(&mut self.data) }
        }
    }
}
