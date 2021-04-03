use super::super::mp;

use crate::algebra::matrix::Ring;

pub struct Integer {
    pub data: mp::__mpz_struct,
}

impl Integer {
    pub fn new() -> Integer {
        let mut x: mp::__mpz_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            mp::__gmpz_init(&mut x);
        }
        Integer { data: x }
    }
    pub fn reset(&mut self) {
        if !self.data._mp_d.is_null() {
            unsafe { mp::__gmpz_clear(&mut self.data) }
        }
        self.data._mp_d = std::ptr::null_mut();
    }
    pub fn sgn(&self) -> i32 {
        if self.data._mp_size < 0 {
            -1
        } else if self.data._mp_size > 0 {
            1
        } else {
            0
        }
    }
}

impl Drop for Integer {
    fn drop(&mut self) {
        if !self.data._mp_d.is_null() {
            unsafe { mp::__gmpz_clear(&mut self.data) }
        }
    }
}

impl Clone for Integer {
    fn clone(&self) -> Integer {
        let mut x: mp::__mpz_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            mp::__gmpz_init_set(&mut x, &self.data);
        }
        Integer { data: x }
    }
}
impl Ring for Integer {
    fn zero() -> Self {
        Integer::new()
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
}
