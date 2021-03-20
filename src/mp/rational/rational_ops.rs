use super::super::mp;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::rational::{Long, Rational, ULong};

fn nummut(p: mp::mpq_ptr) -> *mut mp::__mpz_struct {
    unsafe { &mut (*p)._mp_num }
}
fn denmut(p: mp::mpq_ptr) -> *mut mp::__mpz_struct {
    unsafe { &mut (*p)._mp_den }
}
fn numref(p: mp::mpq_srcptr) -> *const mp::__mpz_struct {
    unsafe { &(*p)._mp_num }
}
fn denref(p: mp::mpq_srcptr) -> *const mp::__mpz_struct {
    unsafe { &(*p)._mp_den }
}

fn _add(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: mp::mpq_srcptr) {
    unsafe {
        mp::__gmpq_add(target, op1, op2);
    }
}
fn _add_ui(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: ULong) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpq_set(target, op1);
        }
        mp::__gmpz_addmul_ui(nummut(target), denref(op1), op2);
    }
}
fn _add_si(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: Long) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpq_set(target, op1);
        }
        if op2 >= 0 {
            mp::__gmpz_addmul_ui(nummut(target), denref(op1), op2 as ULong);
        } else {
            mp::__gmpz_submul_ui(nummut(target), denref(op1), -op2 as ULong);
        }
    }
}

fn _mul(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: mp::mpq_srcptr) {
    unsafe {
        mp::__gmpq_mul(target, op1, op2);
    }
}
fn _mul_ui(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: ULong) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpz_set(denmut(target), denref(op1));
        }
        mp::__gmpz_mul_ui(nummut(target), numref(op1), op2);
        mp::__gmpq_canonicalize(target);
    }
}
fn _mul_si(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: Long) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpz_set(denmut(target), denref(op1));
        }
        mp::__gmpz_mul_si(nummut(target), numref(op1), op2);
        mp::__gmpq_canonicalize(target);
    }
}

fn _sub(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: mp::mpq_srcptr) {
    unsafe {
        mp::__gmpq_sub(target, op1, op2);
    }
}
fn _sub_ui(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: ULong) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpq_set(target, op1);
        }
        mp::__gmpz_submul_ui(nummut(target), denref(op1), op2);
    }
}
fn _sub_si(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: Long) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpq_set(target, op1);
        }
        if op2 >= 0 {
            mp::__gmpz_submul_ui(nummut(target), denref(op1), op2 as ULong);
        } else {
            mp::__gmpz_addmul_ui(nummut(target), denref(op1), -op2 as ULong);
        }
    }
}
fn _ui_sub(target: mp::mpq_ptr, op1: ULong, op2: mp::mpq_srcptr) {
    _sub_ui(target, op2, op1);
    unsafe {
        mp::__gmpq_neg(target, target);
    }
}
fn _si_sub(target: mp::mpq_ptr, op1: Long, op2: mp::mpq_srcptr) {
    _sub_si(target, op2, op1);
    unsafe {
        mp::__gmpq_neg(target, target);
    }
}

fn _div(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: mp::mpq_srcptr) {
    unsafe {
        mp::__gmpq_div(target, op1, op2);
    }
}
fn _div_ui(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: ULong) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpz_set(nummut(target), numref(op1));
        }
        mp::__gmpz_mul_ui(denmut(target), denref(op1), op2);
        mp::__gmpq_canonicalize(target);
    }
}
fn _div_si(target: mp::mpq_ptr, op1: mp::mpq_srcptr, op2: Long) {
    unsafe {
        if target as mp::mpq_srcptr != op1 {
            mp::__gmpz_set(nummut(target), numref(op1));
        }
        mp::__gmpz_mul_si(denmut(target), denref(op1), op2);
        mp::__gmpq_canonicalize(target);
    }
}
fn _ui_div(target: mp::mpq_ptr, op1: ULong, op2: mp::mpq_srcptr) {
    _div_ui(target, op2, op1);
    unsafe {
        mp::__gmpq_inv(target, target);
    }
}
fn _si_div(target: mp::mpq_ptr, op1: Long, op2: mp::mpq_srcptr) {
    _div_si(target, op2, op1);
    unsafe {
        mp::__gmpq_inv(target, target);
    }
}

macro_rules! define_flipped {
    ($f: ident, $f2: ident, $T: ty) => {
        fn $f2(target: mp::mpq_ptr, op1: $T, op2: mp::mpq_srcptr) {
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
        impl $Op<$T> for &Rational {
            type Output = Rational;
            fn $op_name(self, rhs: $T) -> Rational {
                let mut x = Rational::new();
                $f(&mut x.data, &self.data, rhs);
                x
            }
        }
        impl $Op<$T> for Rational {
            type Output = Rational;
            fn $op_name(mut self, rhs: $T) -> Rational {
                $f(&mut self.data, &self.data, rhs);
                self
            }
        }
        impl $Op<&Rational> for $T {
            type Output = Rational;
            fn $op_name(self, rhs: &Rational) -> Rational {
                let mut x = Rational::new();
                $f2(&mut x.data, self, &rhs.data);
                x
            }
        }
        impl $Op<Rational> for $T {
            type Output = Rational;
            fn $op_name(self, mut rhs: Rational) -> Rational {
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
        impl $Op<$T> for Rational {
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
define_subdiv!(_sub_si, _si_sub, _div_si, _si_div, Long);

// Op: Add, Sub, Mul, Div
// op_name: add, sub, mul, div
// f: _add
macro_rules! define_realops {
    ($Op: ident, $op_name: ident, $f: ident) => {
        impl $Op for &Rational {
            type Output = Rational;
            fn $op_name(self, rhs: &Rational) -> Rational {
                let mut x = Rational::new();
                $f(&mut x.data, &self.data, &rhs.data);
                x
            }
        }
        impl $Op<Rational> for &Rational {
            type Output = Rational;
            fn $op_name(self, mut rhs: Rational) -> Rational {
                $f(&mut rhs.data, &self.data, &rhs.data);
                rhs
            }
        }
        impl $Op<&Rational> for Rational {
            type Output = Rational;
            fn $op_name(mut self, rhs: &Rational) -> Rational {
                $f(&mut self.data, &self.data, &rhs.data);
                self
            }
        }
        impl $Op for Rational {
            type Output = Rational;
            fn $op_name(mut self, rhs: Rational) -> Rational {
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
        impl $Op<&Rational> for Rational {
            fn $op_name(&mut self, rhs: &Rational) {
                $f(&mut self.data, &self.data, &rhs.data);
            }
        }
        impl $Op for Rational {
            fn $op_name(&mut self, rhs: Rational) {
                $f(&mut self.data, &self.data, &rhs.data);
            }
        }
    };
}

define_realops!(Add, add, _add);
define_realops!(Mul, mul, _mul);
define_realops!(Sub, sub, _sub);
define_realops!(Div, div, _div);

define_realassign!(AddAssign, add_assign, _add);
define_realassign!(MulAssign, mul_assign, _mul);
define_realassign!(SubAssign, sub_assign, _sub);
define_realassign!(DivAssign, div_assign, _div);
