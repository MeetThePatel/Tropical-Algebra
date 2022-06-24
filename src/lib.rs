use std::ops::{Add, Mul};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct MaxTropical(pub f64);

impl MaxTropical {
    pub fn ninf() -> MaxTropical {
        MaxTropical(f64::NEG_INFINITY)
    }
}

impl std::fmt::Display for MaxTropical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for MaxTropical {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        MaxTropical(self.0.max(rhs.0))
    }
}

impl Mul for MaxTropical {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        MaxTropical(self.0 + rhs.0)
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

#[derive(PartialEq, PartialOrd, Debug)]
pub struct MinTropical(f64);

impl MinTropical {
    pub fn inf() -> MinTropical {
        MinTropical(f64::INFINITY)
    }
}

impl std::fmt::Display for MinTropical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for MinTropical {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        MinTropical(self.0.min(rhs.0))
    }
}

impl Mul for MinTropical {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        MinTropical(self.0 + rhs.0)
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
    fn max_multiplication() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(x * y, MaxTropical(6.0));
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
    fn min_multiplication() {
        let x = MinTropical(5.0);
        let y = MinTropical(1.0);
        assert_eq!(x * y, MinTropical(6.0));
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