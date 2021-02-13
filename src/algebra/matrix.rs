use std::fmt;
use std::ops::Add;

pub struct Vector<T>
where
	T: Ring,
{
	pub arr: Box<[T]>,
}

pub trait Ring: Clone {
	fn zero() -> Self;
	fn iszero(&self) -> bool;
	fn _add(&self, rhs: &Self) -> Self;
}
impl Ring for f64 {
	fn zero() -> f64 {
		0.0
	}
	fn iszero(&self) -> bool {
		*self == 0.0
	}
	fn _add(&self, rhs: &Self) -> Self {
		*self + *rhs
	}
}
impl Ring for f32 {
	fn zero() -> f32 {
		0.0
	}
	fn iszero(&self) -> bool {
		*self == 0.0
	}
	fn _add(&self, rhs: &Self) -> Self {
		*self + *rhs
	}
}
impl<T> Ring for Vector<T>
where
	T: Ring,
{
	fn zero() -> Vector<T> {
		Vector { arr: Box::new([]) }
	}
	fn iszero(&self) -> bool {
		for i in 0..self.dim() {
			if !self.arr[i].iszero() {
				return false;
			}
		}
		return true;
	}
	fn _add(&self, rhs: &Vector<T>) -> Vector<T> {
		assert_eq!(self.dim(), rhs.dim());
		let dim = self.dim();
		let mut arr: Vec<T> = Vec::with_capacity(dim);
		for i in 0..dim {
			arr.push(self.arr[i]._add(&rhs.arr[i]));
		}
		Vector {
			arr: arr.into_boxed_slice(),
		}
	}
}
impl<T> Clone for Vector<T>
where
	T: Ring,
{
	fn clone(&self) -> Vector<T> {
		let mut arr = Vec::with_capacity(self.dim());
		for i in 0..self.arr.len() {
			arr.push(self.arr[i].clone());
		}
		Vector {
			arr: arr.into_boxed_slice(),
		}
	}
}
impl<T> Vector<T>
where
	T: Ring,
{
	pub fn dim(&self) -> usize {
		self.arr.len()
	}
	pub fn new(len: usize) -> Vector<T> {
		let mut arr = Vec::with_capacity(len);
		for _ in 0..len {
			arr.push(T::zero());
		}
		Vector {
			arr: arr.into_boxed_slice(),
		}
	}
}

impl<T> Add for &Vector<T>
where
	T: Ring,
{
	type Output = Vector<T>;

	fn add(self, rhs: &Vector<T>) -> Vector<T> {
		self._add(&rhs)
	}
}

impl<T> fmt::Display for Vector<T>
where
	T: Ring + fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Vec[")?;
		if self.dim() > 0 {
			write!(f, "{}", self.arr[0])?;
			for i in 1..self.dim() {
				write!(f, ", {}", self.arr[i])?;
			}
		}
		write!(f, "]")
	}
}
