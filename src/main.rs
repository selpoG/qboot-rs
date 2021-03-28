// -*- coding:utf-8-unix -*-
extern crate qboot;

use std::io::stdin;
use std::io::{stdout, Write};

use qboot::algebra::matrix::Vector;
use qboot::mp::{Integer, Rational, Real, ULong};

macro_rules! prompt {
    ($($arg:tt)*) => (print!($($arg)*); stdout().flush().unwrap());
}

fn cin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse::<T>().ok().unwrap()
}

fn main() {
    let mut n = Integer::from(1 as ULong);
    for i in 1..=100 {
        n *= i as ULong;
    }
    println!("100! = {}", n);

    let mut t = Rational::from(1.0);
    let mut s_d = Rational::from(1.0);
    for i in 1..=10 {
        t *= i as ULong;
        s_d += 1 as ULong / &t;
    }
    println!("Sum is {}", s_d);

    let mut t = Real::from(1.0);
    let mut s_d = Real::from(1.0);
    for i in 1..=100 {
        t *= i as ULong;
        s_d += t.recip();
    }
    println!("Sum is {}", s_d);

    let v: Vector<Real> = Vector::new(3);
    println!("{} + {} = {}", v, v, &v + &v);

    prompt!("a = ");
    let a: Real = cin();
    prompt!("b = ");
    let b: Real = cin();
    println!("{} + {} = {}", a, b, &a + &b);

    let mut v: Vector<Vector<Real>> = Vector::new(3);
    for i in 0..3 {
        v.arr[i] = Vector::new(2);
        for j in 0..2 {
            v.arr[i].arr[j] = Real::from((i + j) as ULong);
        }
    }
    println!("2 * {} = {}", v, &v + &v);
}
