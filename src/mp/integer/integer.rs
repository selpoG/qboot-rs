use super::gmp;

pub type ULong = ::std::os::raw::c_ulong;
pub type Long = ::std::os::raw::c_long;

pub struct Integer {
    pub data: gmp::__mpz_struct,
}

impl Integer {
    pub fn new() -> Integer {
        let mut x: gmp::__mpz_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        unsafe {
            gmp::__gmpz_init(&mut x);
        }
        Integer { data: x }
    }
    pub fn reset(&mut self) {
        if !self.data._mp_d.is_null() {
            unsafe { gmp::__gmpz_clear(&mut self.data) }
        }
        self.data._mp_d = std::ptr::null_mut();
    }
}

impl Drop for Integer {
    fn drop(&mut self) {
        if !self.data._mp_d.is_null() {
            unsafe { gmp::__gmpz_clear(&mut self.data) }
        }
    }
}
