//! This library is an implementation of the tropical algebras in Rust. A cursory exposition to the tropical algebras can be found on [Wikipedia](https://en.wikipedia.org/wiki/Tropical_semiring).

use auto_ops::impl_op_ex;
use ndarray::Array2;

/// An element of the *max tropical semiring* (or **max-plus semiring** or **max-plus algebra**). A cursory exposition to the max tropical semiring can be found on [Wikipedia](https://en.wikipedia.org/wiki/Tropical_semiring#:~:text=max%20tropical%20semiring).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MaxTropical(
    pub f64
);

impl MaxTropical {
    /// Addition unit element of the max tropical semiring.
    pub fn ninf() -> MaxTropical {
        MaxTropical(f64::NEG_INFINITY)
    }

    /// Multiplication unit element of the max tropical semiring.
    pub fn zero() -> MaxTropical {
        MaxTropical(0.0)
    }
}

impl std::fmt::Display for MaxTropical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_op_ex!(+ |a: MaxTropical, b: MaxTropical| -> MaxTropical { MaxTropical(a.0.max(b.0)) });
impl_op_ex!(+= |a: &mut MaxTropical, b: MaxTropical| { a.0 = a.0.max(b.0) });
impl_op_ex!(* |a: MaxTropical, b: MaxTropical| -> MaxTropical { MaxTropical(a.0 + b.0) });
impl_op_ex!(*= |a: &mut MaxTropical, b: MaxTropical| { a.0 = a.0 + b.0 });

impl num_traits::identities::Zero for MaxTropical {
    fn zero() -> Self {
        MaxTropical::ninf()
    }

    fn set_zero(&mut self) {
        self.0 = MaxTropical::ninf().0;
    }

    fn is_zero(&self) -> bool {
        self.0 == MaxTropical::ninf().0
    }
}

/// Matrix addition in the max tropical semiring.
pub fn max_tropical_mat_add (lhs: &Array2<MaxTropical>, rhs: &Array2<MaxTropical>) -> Array2<MaxTropical> {
    if lhs.shape() != rhs.shape() {
        panic!("LHS has dimensions: {} x {}. RHS has dimensions: {} x {}. Cannot multiply matrices.", lhs.nrows(), lhs.ncols(), rhs.nrows(), rhs.ncols());
    }
    let mut ret = Array2::zeros((lhs.nrows(), lhs.ncols()));
    for i in 0..ret.nrows() {
        for j in 0..ret.ncols() {
            ret[[i, j]] = lhs[[i, j]] + rhs[[i, j]];
        }
    }
    ret
}

/// Matrix multiplication in the max tropical semiring.
pub fn max_tropical_mat_mul (lhs: &Array2<MaxTropical>, rhs: &Array2<MaxTropical>) -> Array2<MaxTropical> {
    if lhs.ncols() != rhs.nrows() {
        panic!("LHS has dimensions: {} x {}. RHS has dimensions: {} x {}. Cannot multiply matrices.", lhs.nrows(), lhs.ncols(), rhs.nrows(), rhs.ncols());
    }
    let mut ret = Array2::zeros((lhs.nrows(), rhs.ncols()));
    for i in 0..ret.nrows() {
        for j in 0..ret.ncols() {
            ret[[i, j]] = MaxTropical::ninf();
            for k in 0..ret.ncols() {
                ret[[i, j]] += lhs[[i, k]] * rhs[[k, j]];
            }
        }
    }
    ret
}

/// An element of the *min tropical semiring* (or **min-plus semiring** or **min-plus algebra**). A cursory exposition to the min tropical semiring can be found on [Wikipedia](https://en.wikipedia.org/wiki/Tropical_semiring#:~:text=min%20tropical%20semiring).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MinTropical(pub f64);

impl MinTropical {
    /// Addition unit element of the min tropical semiring.
    pub fn inf() -> MinTropical {
        MinTropical(f64::INFINITY)
    }

    /// Multiplication unit element of the min tropical semiring.
    pub fn zero() -> MinTropical {
        MinTropical(0.0)
    }
}

impl std::fmt::Display for MinTropical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_op_ex!(+ |a: MinTropical, b: MinTropical| -> MinTropical { MinTropical(a.0.min(b.0)) });
impl_op_ex!(+= |a: &mut MinTropical, b: MinTropical| { a.0 = a.0.min(b.0) });
impl_op_ex!(* |a: MinTropical, b: MinTropical| -> MinTropical { MinTropical(a.0 + b.0) });
impl_op_ex!(*= |a: &mut MinTropical, b: MinTropical| { a.0 = a.0 + b.0 });

impl num_traits::identities::Zero for MinTropical {
    fn zero() -> Self {
        MinTropical::inf()
    }

    fn set_zero(&mut self) {
        self.0 = MinTropical::inf().0;
    }

    fn is_zero(&self) -> bool {
        self.0 == MinTropical::inf().0
    }
}

/// Matrix addition in the min tropical semiring.
pub fn min_tropical_mat_add (lhs: &Array2<MinTropical>, rhs: &Array2<MinTropical>) -> Array2<MinTropical> {
    if lhs.shape() != rhs.shape() {
        panic!("LHS has dimensions: {} x {}. RHS has dimensions: {} x {}. Cannot multiply matrices.", lhs.nrows(), lhs.ncols(), rhs.nrows(), rhs.ncols());
    }
    let mut ret = Array2::zeros((lhs.nrows(), lhs.ncols()));
    for i in 0..ret.nrows() {
        for j in 0..ret.ncols() {
            ret[[i, j]] = lhs[[i, j]] + rhs[[i, j]];
        }
    }
    ret
}

/// Matrix multiplication in the min tropical semiring.
pub fn min_tropical_mat_mul (lhs: &Array2<MinTropical>, rhs: &Array2<MinTropical>) -> Array2<MinTropical> {
    if lhs.ncols() != rhs.nrows() {
        panic!("LHS has dimensions: {} x {}. RHS has dimensions: {} x {}. Cannot multiply matrices.", lhs.nrows(), lhs.ncols(), rhs.nrows(), rhs.ncols());
    }
    let mut ret = Array2::zeros((lhs.nrows(), rhs.ncols()));
    for i in 0..ret.nrows() {
        for j in 0..ret.ncols() {
            ret[[i, j]] = MinTropical::inf();
            for k in 0..ret.ncols() {
                ret[[i, j]] += lhs[[i, k]] * rhs[[k, j]];
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use ndarray::arr2;
    use crate::*;

    #[test]
    fn max_addition() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(x + y, MaxTropical(5.0));
    }

    #[test]
    fn max_addition_assign() {
        let mut x = MaxTropical(5.0);
        x += MaxTropical(1.0);
        assert_eq!(x, MaxTropical(5.0));
    }

    #[test]
    fn max_multiplication() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(x * y, MaxTropical(6.0));
    }

    #[test]
    fn max_multiplication_assign() {
        let mut x = MaxTropical(5.0);
        x *= MaxTropical(1.0);
        assert_eq!(x, MaxTropical(6.0));
    }

    #[test]
    fn max_identity_addition() {
        let ninf = MaxTropical::ninf();
        let x = MaxTropical(5.0);
        assert_eq!(x + ninf, MaxTropical(5.0));
    }

    #[test]
    fn max_identity_multiplication() {
        let zero = MaxTropical(0.0);
        let x = MaxTropical(5.0);
        assert_eq!(x * zero, MaxTropical(5.0));
    }

    #[test]
    fn max_matrix_addition() {
        let x = arr2(&[[MaxTropical(2.0), MaxTropical(4.0)], [MaxTropical(1.0), MaxTropical(0.0)]]);
        let y = arr2(&[[MaxTropical(5.0), MaxTropical::ninf()], [MaxTropical(6.0), MaxTropical(-3.0)]]);
        let z = arr2(&[[MaxTropical(5.0), MaxTropical(4.0)], [MaxTropical(6.0), MaxTropical(0.0)]]);
        assert_eq!(max_tropical_mat_add(&x, &y), z);
    }

    #[test]
    fn max_matrix_multiplication() {
        let x = arr2(&[[MaxTropical(2.0), MaxTropical(4.0)], [MaxTropical(1.0), MaxTropical(0.0)]]);
        let y = arr2(&[[MaxTropical(5.0), MaxTropical::ninf()], [MaxTropical(6.0), MaxTropical(-3.0)]]);
        let z = arr2(&[[MaxTropical(10.0), MaxTropical(1.0)], [MaxTropical(6.0), MaxTropical(-3.0)]]);
        assert_eq!(max_tropical_mat_mul(&x, &y), z);
    }

    #[test]
    fn min_addition() {
        let x = MinTropical(5.0);
        let y = MinTropical(1.0);
        assert_eq!(x + y, MinTropical(1.0));
    }

    #[test]
    fn min_addition_assign() {
        let mut x = MinTropical(5.0);
        x += MinTropical(1.0);
        assert_eq!(x, MinTropical(1.0));
    }

    #[test]
    fn min_multiplication() {
        let x = MinTropical(5.0);
        let y = MinTropical(1.0);
        assert_eq!(x * y, MinTropical(6.0));
    }

    #[test]
    fn min_multiplication_assign() {
        let mut x = MinTropical(5.0);
        x *= MinTropical(1.0);
        assert_eq!(x, MinTropical(6.0));
    }

    #[test]
    fn min_identity_addition() {
        let ninf = MinTropical::inf();
        let x = MinTropical(5.0);
        assert_eq!(x + ninf, MinTropical(5.0));
    }

    #[test]
    fn min_identity_multiplication() {
        let zero = MinTropical(0.0);
        let x = MinTropical(5.0);
        assert_eq!(x * zero, MinTropical(5.0));
    }

    #[test]
    fn min_matrix_addition() {
        let x = arr2(&[[MinTropical(2.0), MinTropical(4.0)], [MinTropical(1.0), MinTropical(0.0)]]);
        let y = arr2(&[[MinTropical(5.0), MinTropical::inf()], [MinTropical(6.0), MinTropical(-3.0)]]);
        let z = arr2(&[[MinTropical(2.0), MinTropical(4.0)], [MinTropical(1.0), MinTropical(-3.0)]]);
        assert_eq!(min_tropical_mat_add(&x, &y), z);
    }

    #[test]
    fn min_matrix_multiplication() {
        let x = arr2(&[[MinTropical(2.0), MinTropical(4.0)], [MinTropical(1.0), MinTropical(0.0)]]);
        let y = arr2(&[[MinTropical(5.0), MinTropical::inf()], [MinTropical(6.0), MinTropical(-3.0)]]);
        let z = arr2(&[[MinTropical(7.0), MinTropical(1.0)], [MinTropical(6.0), MinTropical(-3.0)]]);
        assert_eq!(min_tropical_mat_mul(&x, &y), z);
    }
}
