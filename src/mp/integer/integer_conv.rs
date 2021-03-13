use super::super::gmp;

use super::integer::{Integer, Long, ULong};

impl From<ULong> for Integer {
    fn from(from: ULong) -> Integer {
        let mut x: gmp::__mpz_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpz_init_set_ui(&mut x, from);
        }
        Integer { data: x }
    }
}
impl From<Long> for Integer {
    fn from(from: Long) -> Integer {
        let mut x: gmp::__mpz_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpz_init_set_si(&mut x, from);
        }
        Integer { data: x }
    }
}
impl From<f64> for Integer {
    fn from(from: f64) -> Integer {
        let mut x: gmp::__mpz_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpz_init_set_d(&mut x, from);
        }
        Integer { data: x }
    }
}

impl From<&Integer> for ULong {
    fn from(from: &Integer) -> ULong {
        unsafe { gmp::__gmpz_get_ui(&from.data) }
    }
}
impl From<&Integer> for Long {
    fn from(from: &Integer) -> Long {
        unsafe { gmp::__gmpz_get_si(&from.data) }
    }
}
impl From<&Integer> for f64 {
    fn from(from: &Integer) -> f64 {
        unsafe { gmp::__gmpz_get_d(&from.data) }
    }
}
