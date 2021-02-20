pub mod algebra;
pub mod mp;

#[cfg(test)]
mod tests {
    use super::algebra::matrix::Vector;
    use super::mp::real::Real;
    #[test]
    fn sample() {
        let _vec: Vector<Real> = Vector::new(10);
        let _vec: Vector<Vector<Real>> = Vector::new(10);
        let _vec: Vector<Vector<Vector<Real>>> = Vector::new(10);
        println!("{}", _vec.dim());
    }
}
