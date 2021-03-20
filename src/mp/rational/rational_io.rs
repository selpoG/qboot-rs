use super::super::gmp;

use std::ffi::{CStr, CString};
use std::fmt;

use super::rational::Rational;

impl std::str::FromStr for Rational {
    // 面倒なので省略
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c_str = CString::new(s).expect("CString::new failed");
        unsafe {
            let mut x = Rational::new();
            match gmp::__gmpq_set_str(&mut x.data, c_str.as_ptr(), 10) {
                -1 => Err(()),
                _ => Ok(x),
            }
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base: i32 = 10;
        unsafe {
            let capacity = gmp::__gmpz_sizeinbase(&self.data._mp_num, base)
                + gmp::__gmpz_sizeinbase(&self.data._mp_den, base)
                + 3;
            let buf = gmp::malloc(capacity) as *mut ::std::os::raw::c_char;
            if buf == ::std::ptr::null_mut() {
                Err(std::fmt::Error {})
            } else {
                gmp::__gmpq_get_str(buf, base, &self.data);
                let s = CStr::from_ptr(buf).to_str();
                let ret = match s {
                    Ok(s) => {
                        write!(f, "Rational({})", s)
                    }
                    Err(_err) => Err(std::fmt::Error {}),
                };
                gmp::free(buf as *mut ::std::os::raw::c_void);
                ret
            }
        }
    }
}
