use super::bindings::gmp;

use std::ops::Add;

pub struct Integer {
	pub data: gmp::mpz_t,
}

impl Integer {
	pub fn new() -> Integer {
		let mut x: gmp::mpz_t = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
		unsafe {
			gmp::__gmpz_init(&mut x[0]);
		}
		Integer { data: x }
	}
	pub fn reset(&mut self) {
		if !self.data[0]._mp_d.is_null() {
			unsafe { gmp::__gmpz_clear(&mut self.data[0]) }
		}
		self.data[0]._mp_d = std::ptr::null_mut();
	}
}

impl Drop for Integer {
	fn drop(&mut self) {
		if !self.data[0]._mp_d.is_null() {
			unsafe { gmp::__gmpz_clear(&mut self.data[0]) }
		}
	}
}

impl Add for Integer {
	type Output = Integer;

	fn add(self, rhs: Integer) -> Integer {
		let mut x = Integer::new();
		unsafe {
			gmp::__gmpz_add(&mut x.data[0], &self.data[0], &rhs.data[0]);
		}
		x
	}
}
