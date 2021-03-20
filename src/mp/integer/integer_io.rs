use super::super::mp;

use std::ffi::{CStr, CString};
use std::fmt;

use super::integer::Integer;

impl std::str::FromStr for Integer {
    // 面倒なので省略
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c_str = CString::new(s).expect("CString::new failed");
        unsafe {
            let mut x: mp::__mpz_struct = std::mem::MaybeUninit::uninit().assume_init();
            match mp::__gmpz_init_set_str(&mut x, c_str.as_ptr(), 10) {
                -1 => {
                    mp::__gmpz_clear(&mut x);
                    Err(())
                }
                _ => Ok(Integer { data: x }),
            }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base: i32 = 10;
        unsafe {
            let capacity = mp::__gmpz_sizeinbase(&self.data, base) + 2;
            let buf = mp::malloc(capacity) as *mut ::std::os::raw::c_char;
            if buf == ::std::ptr::null_mut() {
                Err(std::fmt::Error {})
            } else {
                mp::__gmpz_get_str(buf, base, &self.data);
                let s = CStr::from_ptr(buf).to_str();
                let ret = match s {
                    Ok(s) => {
                        write!(f, "Integer({})", s)
                    }
                    Err(_err) => Err(std::fmt::Error {}),
                };
                mp::free(buf as *mut ::std::os::raw::c_void);
                ret
            }
        }
    }
}
