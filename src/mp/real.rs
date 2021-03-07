use super::bindings::mpfr;

use once_cell::sync::OnceCell;

use super::super::algebra::matrix::Ring;

pub type Precision = mpfr::mpfr_prec_t;
pub type RoundMode = mpfr::mpfr_rnd_t;
pub type ULong = mpfr::mpfr_ulong;
pub type Long = mpfr::mpfr_long;
pub type UIntMax = mpfr::uintmax_t;
pub type IntMax = mpfr::intmax_t;

// _GLOBAL_PREC, _GLOBAL_RND を複数回書き換えられるようにする
static _PREC_SET: OnceCell<bool> = OnceCell::new();
pub static mut _GLOBAL_PREC: Precision = 1000;
static _RND_SET: OnceCell<bool> = OnceCell::new();
pub static mut _GLOBAL_RND: RoundMode = mpfr::mpfr_rnd_t_MPFR_RNDN;

// 1 回だけ安全に初期化できることにする (その前に _GLOBAL_PREC, _GLOBAL_RND が参照されていないことは良識に任せる…)
pub fn set_prec(prec: Precision) -> Result<(), &'static str> {
    match _PREC_SET.set(true) {
        Ok(()) => {
            unsafe { _GLOBAL_PREC = prec }
            Ok(())
        }
        Err(_) => Err("prec is already set"),
    }
}
pub fn set_rnd(rnd: RoundMode) -> Result<(), &'static str> {
    match _RND_SET.set(true) {
        Ok(()) => {
            unsafe { _GLOBAL_RND = rnd }
            Ok(())
        }
        Err(_) => Err("rnd is already set"),
    }
}

pub struct Real {
    pub data: mpfr::__mpfr_struct,
}

impl Real {
    pub fn new() -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_set_zero(&mut x.data, 1);
        }
        x
    }
    pub fn reset(&mut self) {
        if !self.data._mpfr_d.is_null() {
            unsafe { mpfr::mpfr_clear(&mut self.data) }
        }
        self.data._mpfr_d = std::ptr::null_mut();
    }
    pub fn sgn(&self) -> mpfr::mpfr_int {
        unsafe { mpfr::mpfr_sgn(&self.data) }
    }
    pub fn isnan(&self) -> bool {
        unsafe { mpfr::mpfr_nan_p(&self.data) != 0 }
    }
    pub fn isinf(&self) -> bool {
        unsafe { mpfr::mpfr_inf_p(&self.data) != 0 }
    }
    pub fn recip(&self) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_ui_div(&mut x.data, 1 as ULong, &self.data, _GLOBAL_RND);
        }
        x
    }
}

impl Drop for Real {
    fn drop(&mut self) {
        if !self.data._mpfr_d.is_null() {
            unsafe { mpfr::mpfr_clear(&mut self.data) }
        }
    }
}

impl Clone for Real {
    fn clone(&self) -> Real {
        let mut x = Real::_nan();
        unsafe {
            mpfr::mpfr_set(&mut x.data, &self.data, _GLOBAL_RND);
        }
        x
    }
}
impl Ring for Real {
    fn zero() -> Self {
        Real::new()
    }
    fn iszero(&self) -> bool {
        unsafe { mpfr::mpfr_zero_p(&self.data) != 0 }
    }
    fn _add(&self, rhs: &Self) -> Self {
        self + rhs
    }
}
