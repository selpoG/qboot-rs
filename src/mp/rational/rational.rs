use super::super::gmp;

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
