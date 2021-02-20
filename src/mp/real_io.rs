use super::bindings::mpfr;

use std::ffi::{CStr, CString};
use std::fmt;

use super::super::algebra::matrix::Ring;
use super::real::{Real, _GLOBAL_RND};

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
