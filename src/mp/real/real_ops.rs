use super::super::mp;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::super::{Integer, Long, Rational, ULong};
use super::real::{Real, _GLOBAL_RND};

fn _add(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_add(target, op1, op2, _GLOBAL_RND);
    }
}
fn _add_ui(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: ULong) {
    unsafe {
        mp::mpfr_add_ui(target, op1, op2, _GLOBAL_RND);
    }
}
fn _add_si(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: Long) {
    unsafe {
        mp::mpfr_add_si(target, op1, op2, _GLOBAL_RND);
    }
}
fn _add_d(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: f64) {
    unsafe {
        mp::mpfr_add_d(target, op1, op2, _GLOBAL_RND);
    }
}
fn _add_z(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Integer) {
    unsafe {
        mp::mpfr_add_z(target, op1, &op2.data, _GLOBAL_RND);
    }
}
fn _add_q(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Rational) {
    unsafe {
        mp::mpfr_add_q(target, op1, &op2.data, _GLOBAL_RND);
    }
}

fn _mul(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_mul(target, op1, op2, _GLOBAL_RND);
    }
}
fn _mul_ui(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: ULong) {
    unsafe {
        mp::mpfr_mul_ui(target, op1, op2, _GLOBAL_RND);
    }
}
fn _mul_si(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: Long) {
    unsafe {
        mp::mpfr_mul_si(target, op1, op2, _GLOBAL_RND);
    }
}
fn _mul_d(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: f64) {
    unsafe {
        mp::mpfr_mul_d(target, op1, op2, _GLOBAL_RND);
    }
}
fn _mul_z(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Integer) {
    unsafe {
        mp::mpfr_mul_z(target, op1, &op2.data, _GLOBAL_RND);
    }
}
fn _mul_q(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Rational) {
    unsafe {
        mp::mpfr_mul_q(target, op1, &op2.data, _GLOBAL_RND);
    }
}

fn _sub(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_sub(target, op1, op2, _GLOBAL_RND);
    }
}
fn _sub_ui(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: ULong) {
    unsafe {
        mp::mpfr_sub_ui(target, op1, op2, _GLOBAL_RND);
    }
}
fn _sub_si(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: Long) {
    unsafe {
        mp::mpfr_sub_si(target, op1, op2, _GLOBAL_RND);
    }
}
fn _sub_d(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: f64) {
    unsafe {
        mp::mpfr_sub_d(target, op1, op2, _GLOBAL_RND);
    }
}
fn _sub_z(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Integer) {
    unsafe {
        mp::mpfr_sub_z(target, op1, &op2.data, _GLOBAL_RND);
    }
}
fn _sub_q(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Rational) {
    unsafe {
        mp::mpfr_sub_q(target, op1, &op2.data, _GLOBAL_RND);
    }
}
fn _ui_sub(target: mp::mpfr_ptr, op1: ULong, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_ui_sub(target, op1, op2, _GLOBAL_RND);
    }
}
fn _si_sub(target: mp::mpfr_ptr, op1: Long, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_si_sub(target, op1, op2, _GLOBAL_RND);
    }
}
fn _d_sub(target: mp::mpfr_ptr, op1: f64, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_d_sub(target, op1, op2, _GLOBAL_RND);
    }
}
fn _z_sub(target: mp::mpfr_ptr, op1: &Integer, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_z_sub(target, &op1.data, op2, _GLOBAL_RND);
    }
}
fn _q_sub(target: mp::mpfr_ptr, op1: &Rational, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_sub_q(target, op2, &op1.data, _GLOBAL_RND);
        mp::mpfr_neg(target, target, _GLOBAL_RND);
    }
}

fn _div(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_div(target, op1, op2, _GLOBAL_RND);
    }
}
fn _div_ui(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: ULong) {
    unsafe {
        mp::mpfr_div_ui(target, op1, op2, _GLOBAL_RND);
    }
}
fn _div_si(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: Long) {
    unsafe {
        mp::mpfr_div_si(target, op1, op2, _GLOBAL_RND);
    }
}
fn _div_d(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: f64) {
    unsafe {
        mp::mpfr_div_d(target, op1, op2, _GLOBAL_RND);
    }
}
fn _div_z(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Integer) {
    unsafe {
        mp::mpfr_div_z(target, op1, &op2.data, _GLOBAL_RND);
    }
}
fn _div_q(target: mp::mpfr_ptr, op1: mp::mpfr_srcptr, op2: &Rational) {
    unsafe {
        mp::mpfr_div_q(target, op1, &op2.data, _GLOBAL_RND);
    }
}
fn _ui_div(target: mp::mpfr_ptr, op1: ULong, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_ui_div(target, op1, op2, _GLOBAL_RND);
    }
}
fn _si_div(target: mp::mpfr_ptr, op1: Long, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_si_div(target, op1, op2, _GLOBAL_RND);
    }
}
fn _d_div(target: mp::mpfr_ptr, op1: f64, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_d_div(target, op1, op2, _GLOBAL_RND);
    }
}
fn _z_div(target: mp::mpfr_ptr, op1: &Integer, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_ui_div(target, 1 as ULong, op2, _GLOBAL_RND);
    }
    _mul_z(target, target, op1)
}
fn _q_div(target: mp::mpfr_ptr, op1: &Rational, op2: mp::mpfr_srcptr) {
    unsafe {
        mp::mpfr_ui_div(target, 1 as ULong, op2, _GLOBAL_RND);
    }
    _mul_q(target, target, op1)
}

macro_rules! define_flipped {
    ($f: ident, $f2: ident, $T: ty) => {
        fn $f2(target: mp::mpfr_ptr, op1: $T, op2: mp::mpfr_srcptr) {
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
        impl $Op<$T> for &Real {
            type Output = Real;
            fn $op_name(self, rhs: $T) -> Real {
                let mut x = Real::_nan();
                $f(&mut x.data, &self.data, rhs);
                x
            }
        }
        impl $Op<$T> for Real {
            type Output = Real;
            fn $op_name(mut self, rhs: $T) -> Real {
                $f(&mut self.data, &self.data, rhs);
                self
            }
        }
        impl $Op<&Real> for $T {
            type Output = Real;
            fn $op_name(self, rhs: &Real) -> Real {
                let mut x = Real::_nan();
                $f2(&mut x.data, self, &rhs.data);
                x
            }
        }
        impl $Op<Real> for $T {
            type Output = Real;
            fn $op_name(self, mut rhs: Real) -> Real {
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
        impl $Op<$T> for Real {
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
define_addmul!(_add_d, _d_add, _mul_d, _d_mul, f64);
define_addmul!(_add_z, _z_add, _mul_z, _z_mul, &Integer);
define_addmul!(_add_q, _q_add, _mul_q, _q_mul, &Rational);

define_subdiv!(_sub_ui, _ui_sub, _div_ui, _ui_div, ULong);
define_subdiv!(_sub_si, _si_sub, _div_si, _si_div, Long);
define_subdiv!(_sub_d, _d_sub, _div_d, _d_div, f64);
define_subdiv!(_sub_z, _z_sub, _div_z, _z_div, &Integer);
define_subdiv!(_sub_q, _q_sub, _div_q, _q_div, &Rational);

// Op: Add, Sub, Mul, Div
// op_name: add, sub, mul, div
// f: _add
macro_rules! define_realops {
    ($Op: ident, $op_name: ident, $f: ident) => {
        impl $Op for &Real {
            type Output = Real;
            fn $op_name(self, rhs: &Real) -> Real {
                let mut x = Real::_nan();
                $f(&mut x.data, &self.data, &rhs.data);
                x
            }
        }
        impl $Op<Real> for &Real {
            type Output = Real;
            fn $op_name(self, mut rhs: Real) -> Real {
                $f(&mut rhs.data, &self.data, &rhs.data);
                rhs
            }
        }
        impl $Op<&Real> for Real {
            type Output = Real;
            fn $op_name(mut self, rhs: &Real) -> Real {
                $f(&mut self.data, &self.data, &rhs.data);
                self
            }
        }
        impl $Op for Real {
            type Output = Real;
            fn $op_name(mut self, rhs: Real) -> Real {
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
        impl $Op<&Real> for Real {
            fn $op_name(&mut self, rhs: &Real) {
                $f(&mut self.data, &self.data, &rhs.data);
            }
        }
        impl $Op for Real {
            fn $op_name(&mut self, rhs: Real) {
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
