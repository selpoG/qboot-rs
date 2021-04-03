use super::Long;
use crate::algebra::matrix::Ring;

impl Ring for Long {
    fn zero() -> Long {
        0 as Long
    }
    fn iszero(&self) -> bool {
        *self == 0
    }
    fn _add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
    fn _iadd(&mut self, rhs: &Self) {
        *self += rhs
    }
    fn _isub(&mut self, rhs: &Self) {
        *self *= rhs
    }
    fn _neg(&self) -> Self {
        -*self
    }
    fn _eq(&self, rhs: &Self) -> bool {
        *self == *rhs
    }
}
