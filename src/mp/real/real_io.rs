use super::super::mp;

use std::ffi::{CStr, CString};
use std::fmt;

use super::real::{Real, _GLOBAL_RND};
use crate::algebra::matrix::Ring;

impl std::str::FromStr for Real {
    // 面倒なので省略
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c_str = CString::new(s).expect("CString::new failed");
        let mut x = Real::_nan();
        unsafe {
            match mp::mpfr_set_str(&mut x.data, c_str.as_ptr(), 10, _GLOBAL_RND) {
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
            let mut exp: mp::mpfr_exp_t = 0;
            let exp_ptr: *mut mp::mpfr_exp_t = &mut exp;
            let prec: mp::size_t = 6 + 1;
            let base: i32 = 10;
            unsafe {
                let sgn = x.sgn();
                let neg = sgn < 0;
                if neg {
                    mp::mpfr_neg(&mut x.data, &x.data, _GLOBAL_RND);
                }
                let ch = mp::mpfr_get_str(
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
                    let ret = match CStr::from_ptr(ch).to_str() {
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
                            write!(f, "Real({})", v)
                        }
                        Err(_err) => Err(std::fmt::Error {}),
                    };
                    mp::mpfr_free_str(ch);
                    ret
                }
            }
        }
    }
}
