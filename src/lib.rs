use std::ops::{Add, AddAssign, Mul, MulAssign};

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

impl Add for MaxTropical {
    type Output = Self;

    /// Addition operation in the max tropical semiring.
    fn add(self, rhs: Self) -> Self {
        MaxTropical(self.0.max(rhs.0))
    }
}

impl AddAssign for MaxTropical {
    /// Addition assignment operation in the max tropical semiring.
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul for MaxTropical {
    type Output = Self;

    /// Multiplication operation in the max tropical semiring.
    fn mul(self, rhs: Self) -> Self::Output {
        MaxTropical(self.0 + rhs.0)
    }
}

impl MulAssign for MaxTropical {
    /// Multiplication assignment operation in the max tropical semiring.
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

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

/// An element of the *min tropical semiring* (or **min-plus semiring** or **min-plus algebra**). A cursory exposition to the min tropical semiring can be found on [Wikipedia](https://en.wikipedia.org/wiki/Tropical_semiring#:~:text=min%20tropical%20semiring).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MinTropical(f64);

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

impl Add for MinTropical {
    type Output = Self;

    /// Addition operation in the min tropical semiring.
    fn add(self, rhs: Self) -> Self {
        MinTropical(self.0.min(rhs.0))
    }
}

impl AddAssign for MinTropical {
    /// Addition assignment operation in the min tropical semiring.
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul for MinTropical {
    type Output = Self;

    /// Multiplication operation in the min tropical semiring.
    fn mul(self, rhs: Self) -> Self::Output {
        MinTropical(self.0 + rhs.0)
    }
}

impl MulAssign for MinTropical {
    /// Multiplication assignment operation in the min tropical algebra.
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{MaxTropical, MinTropical};

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
}