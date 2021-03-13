use super::gmp;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use super::integer::{Integer, Long, ULong};

fn _add(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: gmp::mpz_srcptr) {
    unsafe {
        gmp::__gmpz_add(target, op1, op2);
    }
}
fn _add_ui(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: ULong) {
    unsafe {
        gmp::__gmpz_add_ui(target, op1, op2);
    }
}
fn _add_si(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: Long) {
    unsafe {
        if op2 >= 0 {
            gmp::__gmpz_add_ui(target, op1, op2 as ULong);
        } else {
            gmp::__gmpz_sub_ui(target, op1, -op2 as ULong);
        }
    }
}

fn _mul(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: gmp::mpz_srcptr) {
    unsafe {
        gmp::__gmpz_mul(target, op1, op2);
    }
}
fn _mul_ui(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: ULong) {
    unsafe {
        gmp::__gmpz_mul_ui(target, op1, op2);
    }
}
fn _mul_si(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: Long) {
    unsafe {
        gmp::__gmpz_mul_si(target, op1, op2);
    }
}

fn _sub(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: gmp::mpz_srcptr) {
    unsafe {
        gmp::__gmpz_sub(target, op1, op2);
    }
}
fn _sub_ui(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: ULong) {
    unsafe {
        gmp::__gmpz_sub_ui(target, op1, op2);
    }
}
fn _sub_si(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: Long) {
    unsafe {
        if op2 >= 0 {
            gmp::__gmpz_sub_ui(target, op1, op2 as ULong);
        } else {
            gmp::__gmpz_add_ui(target, op1, -op2 as ULong);
        }
    }
}
fn _ui_sub(target: gmp::mpz_ptr, op1: ULong, op2: gmp::mpz_srcptr) {
    unsafe {
        gmp::__gmpz_ui_sub(target, op1, op2);
    }
}
fn _si_sub(target: gmp::mpz_ptr, op1: Long, op2: gmp::mpz_srcptr) {
    unsafe {
        _sub_si(target, op2, op1);
        gmp::__gmpz_neg(target, target);
    }
}

fn _div(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: gmp::mpz_srcptr) {
    unsafe {
        gmp::__gmpz_fdiv_q(target, op1, op2);
    }
}
fn _div_ui(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: ULong) {
    unsafe {
        gmp::__gmpz_fdiv_q_ui(target, op1, op2);
    }
}
fn _ui_div(target: gmp::mpz_ptr, op1: ULong, op2: gmp::mpz_srcptr) {
    unsafe {
        if gmp::__gmpz_cmp_ui(op2, 0) < 0 {
            let mut x: gmp::__mpz_struct = std::mem::MaybeUninit::uninit().assume_init();
            gmp::__gmpz_init_set(&mut x, op2);
            gmp::__gmpz_neg(&mut x, &x);
            _ui_div(target, op1, &x);
            gmp::__gmpz_clear(&mut x);
        } else if gmp::__gmpz_cmp_ui(op2, op1) > 0 {
            gmp::__gmpz_set_ui(target, 0);
        } else {
            let op2 = gmp::__gmpz_get_ui(op2);
            gmp::__gmpz_set_ui(target, op1 / op2);
        }
    }
}

fn _rem(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: gmp::mpz_srcptr) {
    unsafe {
        gmp::__gmpz_fdiv_r(target, op1, op2);
    }
}
fn _rem_ui(target: gmp::mpz_ptr, op1: gmp::mpz_srcptr, op2: ULong) {
    unsafe {
        gmp::__gmpz_fdiv_r_ui(target, op1, op2);
    }
}
fn _ui_rem(target: gmp::mpz_ptr, op1: ULong, op2: gmp::mpz_srcptr) {
    unsafe {
        if gmp::__gmpz_cmp_ui(op2, op1) > 0 {
            gmp::__gmpz_set_ui(target, op1);
        } else {
            let op2 = if gmp::__gmpz_cmp_ui(op2, 0) >= 0 {
                gmp::__gmpz_get_ui(op2)
            } else {
                let mut x: gmp::__mpz_struct = std::mem::MaybeUninit::uninit().assume_init();
                gmp::__gmpz_init_set(&mut x, op2);
                gmp::__gmpz_neg(&mut x, &x);
                let val = gmp::__gmpz_get_ui(&x);
                gmp::__gmpz_clear(&mut x);
                val
            };
            gmp::__gmpz_set_ui(target, op1 % op2);
        }
    }
}

macro_rules! define_flipped {
    ($f: ident, $f2: ident, $T: ty) => {
        fn $f2(target: gmp::mpz_ptr, op1: $T, op2: gmp::mpz_srcptr) {
            $f(target, op2, op1);
        }
    };
}
// Op: Add, Sub, Mul, Div
// op_name: add, sub, mul, div
// f: _add_ui
// f2: _ui_add (if Op is Add or Mul, f2 must be defined by define_flipped! macro)
// T: ULong
macro_rules! define_binops {
    ($Op: ident, $op_name: ident, $f: ident, $f2:ident, $T: ty) => {
        impl $Op<$T> for &Integer {
            type Output = Integer;
            fn $op_name(self, rhs: $T) -> Integer {
                let mut x = Integer::new();
                $f(&mut x.data, &self.data, rhs);
                x
            }
        }
        impl $Op<$T> for Integer {
            type Output = Integer;
            fn $op_name(mut self, rhs: $T) -> Integer {
                $f(&mut self.data, &self.data, rhs);
                self
            }
        }
        impl $Op<&Integer> for $T {
            type Output = Integer;
            fn $op_name(self, rhs: &Integer) -> Integer {
                let mut x = Integer::new();
                $f2(&mut x.data, self, &rhs.data);
                x
            }
        }
        impl $Op<Integer> for $T {
            type Output = Integer;
            fn $op_name(self, mut rhs: Integer) -> Integer {
                $f2(&mut rhs.data, self, &rhs.data);
                rhs
            }
        }
    };
}
// Op: AddAssign, SubAssign, MulAssign, DivAssign
// op_name: add_assign, sub_assign, mul_assign, div_assign
// f: _add_ui
// T: ULong
macro_rules! define_assign {
    ($Op: ident, $op_name: ident, $f: ident, $T: ty) => {
        impl $Op<$T> for Integer {
            fn $op_name(&mut self, rhs: $T) {
                $f(&mut self.data, &self.data, rhs);
            }
        }
    };
}

macro_rules! define_add {
    ($f: ident, $f2: ident, $T: ty) => {
        define_flipped!($f, $f2, $T);
        define_binops!(Add, add, $f, $f2, $T);
    };
}
macro_rules! define_add_assign {
    ($f: ident, $T: ty) => {
        define_assign!(AddAssign, add_assign, $f, $T);
    };
}

macro_rules! define_mul {
    ($f: ident, $f2: ident, $T: ty) => {
        define_flipped!($f, $f2, $T);
        define_binops!(Mul, mul, $f, $f2, $T);
    };
}
macro_rules! define_mul_assign {
    ($f: ident, $T: ty) => {
        define_assign!(MulAssign, mul_assign, $f, $T);
    };
}

macro_rules! define_sub {
    ($f: ident, $f2: ident, $T: ty) => {
        define_binops!(Sub, sub, $f, $f2, $T);
    };
}
macro_rules! define_sub_assign {
    ($f: ident, $T: ty) => {
        define_assign!(SubAssign, sub_assign, $f, $T);
    };
}

macro_rules! define_div {
    ($f: ident, $f2: ident, $T: ty) => {
        define_binops!(Div, div, $f, $f2, $T);
    };
}
macro_rules! define_div_assign {
    ($f: ident, $T: ty) => {
        define_assign!(DivAssign, div_assign, $f, $T);
    };
}

macro_rules! define_addmul {
    ($f_add: ident, $f_add2: ident, $f_mul: ident, $f_mul2: ident, $T: ty) => {
        define_add!($f_add, $f_add2, $T);
        define_add_assign!($f_add, $T);
        define_mul!($f_mul, $f_mul2, $T);
        define_mul_assign!($f_mul, $T);
    };
}

macro_rules! define_subdiv {
    ($f_sub: ident, $f_sub2: ident, $f_div: ident, $f_div2: ident, $T: ty) => {
        define_sub!($f_sub, $f_sub2, $T);
        define_sub_assign!($f_sub, $T);
        define_div!($f_div, $f_div2, $T);
        define_div_assign!($f_div, $T);
    };
}

define_addmul!(_add_ui, _ui_add, _mul_ui, _ui_mul, ULong);
define_addmul!(_add_si, _si_add, _mul_si, _si_mul, Long);

define_subdiv!(_sub_ui, _ui_sub, _div_ui, _ui_div, ULong);
define_binops!(Rem, rem, _rem_ui, _ui_rem, ULong);
define_assign!(RemAssign, rem_assign, _rem_ui, ULong);
define_sub!(_sub_si, _si_sub, Long);
define_sub_assign!(_sub_si, Long);

// Op: Add, Sub, Mul, Div
// op_name: add, sub, mul, div
// f: _add
macro_rules! define_realops {
    ($Op: ident, $op_name: ident, $f: ident) => {
        impl $Op for &Integer {
            type Output = Integer;
            fn $op_name(self, rhs: &Integer) -> Integer {
                let mut x = Integer::new();
                $f(&mut x.data, &self.data, &rhs.data);
                x
            }
        }
        impl $Op<Integer> for &Integer {
            type Output = Integer;
            fn $op_name(self, mut rhs: Integer) -> Integer {
                $f(&mut rhs.data, &self.data, &rhs.data);
                rhs
            }
        }
        impl $Op<&Integer> for Integer {
            type Output = Integer;
            fn $op_name(mut self, rhs: &Integer) -> Integer {
                $f(&mut self.data, &self.data, &rhs.data);
                self
            }
        }
        impl $Op for Integer {
            type Output = Integer;
            fn $op_name(mut self, rhs: Integer) -> Integer {
                $f(&mut self.data, &self.data, &rhs.data);
                self
            }
        }
    };
}

// Op: AddAssign, SubAssign, MulAssign, DivAssign
// op_name: add_assign, sub_assign, mul_assign, div_assign
// f: _add
macro_rules! define_realassign {
    ($Op: ident, $op_name: ident, $f: ident) => {
        impl $Op<&Integer> for Integer {
            fn $op_name(&mut self, rhs: &Integer) {
                $f(&mut self.data, &self.data, &rhs.data);
            }
        }
        impl $Op for Integer {
            fn $op_name(&mut self, rhs: Integer) {
                $f(&mut self.data, &self.data, &rhs.data);
            }
        }
    };
}

define_realops!(Add, add, _add);
define_realops!(Mul, mul, _mul);
define_realops!(Sub, sub, _sub);
define_realops!(Div, div, _div);
define_realops!(Rem, rem, _rem);

define_realassign!(AddAssign, add_assign, _add);
define_realassign!(MulAssign, mul_assign, _mul);
define_realassign!(SubAssign, sub_assign, _sub);
define_realassign!(DivAssign, div_assign, _div);
define_realassign!(RemAssign, rem_assign, _rem);
