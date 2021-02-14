use super::bindings::mpfr;

use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::{Add, AddAssign, MulAssign};

use once_cell::sync::OnceCell;

use super::super::algebra::matrix::Ring;

pub type Precision = mpfr::mpfr_prec_t;
pub type RoundMode = mpfr::mpfr_rnd_t;
pub type ULong = mpfr::mpfr_ulong;
pub type Long = mpfr::mpfr_long;

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
	fn _nan() -> Real {
		let mut x: mpfr::__mpfr_struct = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
		unsafe {
			mpfr::mpfr_init2(&mut x, _GLOBAL_PREC);
		}
		Real { data: x }
	}
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

impl std::str::FromStr for Real {
	// 面倒なので省略
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let c_str = CString::new(s).expect("CString::new failed");
		let mut x = Real::_nan();
		unsafe {
			match mpfr::mpfr_set_str(&mut x.data, c_str.as_ptr(), 10, _GLOBAL_RND) {
				-1 => Err(()),
				_ => Ok(x),
			}
		}
	}
}

impl From<f32> for Real {
	fn from(from: f32) -> Real {
		let mut x = Real::_nan();
		unsafe {
			mpfr::mpfr_set_flt(&mut x.data, from, _GLOBAL_RND);
		}
		x
	}
}
impl From<f64> for Real {
	fn from(from: f64) -> Real {
		let mut x = Real::_nan();
		unsafe {
			mpfr::mpfr_set_d(&mut x.data, from, _GLOBAL_RND);
		}
		x
	}
}

impl From<Long> for Real {
	fn from(from: Long) -> Real {
		let mut x = Real::_nan();
		unsafe {
			mpfr::mpfr_set_si(&mut x.data, from, _GLOBAL_RND);
		}
		x
	}
}
impl From<ULong> for Real {
	fn from(from: ULong) -> Real {
		let mut x = Real::_nan();
		unsafe {
			mpfr::mpfr_set_ui(&mut x.data, from, _GLOBAL_RND);
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

impl Add for Real {
	type Output = Real;

	fn add(self, rhs: Real) -> Real {
		let mut x = Real::new();
		unsafe {
			mpfr::mpfr_add(&mut x.data, &self.data, &rhs.data, _GLOBAL_RND);
		}
		x
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
		let mut x = Real::new();
		unsafe {
			mpfr::mpfr_add(&mut x.data, &self.data, &rhs.data, _GLOBAL_RND);
		}
		x
	}
}

impl Add for &Real {
	type Output = Real;

	fn add(self, rhs: &Real) -> Real {
		self._add(&rhs)
	}
}

impl AddAssign<&Real> for Real {
	fn add_assign(&mut self, rhs: &Real) {
		unsafe {
			mpfr::mpfr_add(&mut self.data, &self.data, &rhs.data, _GLOBAL_RND);
		}
	}
}

impl MulAssign<&Real> for Real {
	fn mul_assign(&mut self, rhs: &Real) {
		unsafe {
			mpfr::mpfr_mul(&mut self.data, &self.data, &rhs.data, _GLOBAL_RND);
		}
	}
}

impl MulAssign<ULong> for Real {
	fn mul_assign(&mut self, rhs: ULong) {
		unsafe {
			mpfr::mpfr_mul_ui(&mut self.data, &self.data, rhs, _GLOBAL_RND);
		}
	}
}

impl fmt::Display for Real {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.isnan() {
			write!(f, "Real(@NaN@)")
		} else if self.isinf() {
			if self.sgn() >= 0 {
				write!(f, "Real(@Inf@)")
			} else {
				write!(f, "Real(-@Inf@)")
			}
		} else if self.iszero() {
			if self.sgn() >= 0 {
				write!(f, "Real(0)")
			} else {
				write!(f, "Real(-0)")
			}
		} else {
			let mut x = self.clone();
			let mut exp: mpfr::mpfr_exp_t = 0;
			let exp_ptr: *mut mpfr::mpfr_exp_t = &mut exp;
			let prec: mpfr::size_t = 6 + 1;
			let base: i32 = 10;
			unsafe {
				let sgn = x.sgn();
				let neg = sgn < 0;
				if neg {
					mpfr::mpfr_neg(&mut x.data, &x.data, _GLOBAL_RND);
				}
				let ch = mpfr::mpfr_get_str(
					std::ptr::null_mut(),
					exp_ptr,
					base,
					prec,
					&x.data,
					_GLOBAL_RND,
				);
				if ch == std::ptr::null_mut() {
					Err(std::fmt::Error {})
				} else {
					struct Guard {
						ch: *mut i8,
					}
					impl Drop for Guard {
						fn drop(&mut self) {
							unsafe { mpfr::mpfr_free_str(self.ch) }
						}
					}
					let mut _guard = Guard { ch: ch };
					match CStr::from_ptr(ch).to_str() {
						Ok(s) => {
							let mut v = String::from(s);
							if v.len() > 1 {
								v.insert(1, '.')
							}
							exp -= 1;
							v.push('e');
							if exp >= 0 {
								v.push('+')
							} else {
								v.push('-');
								exp = -exp
							}
							if -9 <= exp && exp <= 9 {
								v.push('0')
							}
							v.push_str(&exp.to_string());
							if neg {
								v.insert(0, '-')
							}
							let res = write!(f, "Real({})", v);
							res
						}
						Err(_err) => Err(std::fmt::Error {}),
					}
				}
			}
		}
	}
}
