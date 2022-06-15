#[derive(PartialEq, PartialOrd, Debug)]
struct MaxTropical(f64);

impl MaxTropical {
    fn add(x: MaxTropical, y: MaxTropical) -> MaxTropical {
        MaxTropical(x.0.max(y.0))
    }

    fn multiply(x: MaxTropical, y: MaxTropical) -> MaxTropical {
        MaxTropical(x.0 + y.0)
    }

    fn ninf() -> MaxTropical {
        MaxTropical(f64::NEG_INFINITY)
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
struct MinTropical(f64);

impl MinTropical {
    fn add(x: MinTropical, y: MinTropical) -> MinTropical {
        MinTropical(x.0.min(y.0))
    }

    fn multiply(x: MinTropical, y: MinTropical) -> MinTropical {
        MinTropical(x.0 + y.0)
    }

    fn inf() -> MinTropical {
        MinTropical(f64::INFINITY)
    }
}

#[cfg(test)]
mod tests {
    use crate::{MaxTropical, MinTropical};

    #[test]
    fn max_addition() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(MaxTropical::add(x, y), MaxTropical(5.0));
    }

    #[test]
    fn max_multiplication() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(MaxTropical::multiply(x, y), MaxTropical(6.0));
    }

    #[test]
    fn max_identity_addition() {
        let ninf = MaxTropical::ninf();
        let x = MaxTropical(5.0);
        assert_eq!(MaxTropical::add(x, ninf), MaxTropical(5.0));
    }

    #[test]
    fn max_identity_multiplication() {
        let zero = MaxTropical(0.0);
        let x = MaxTropical(5.0);
        assert_eq!(MaxTropical::multiply(x, zero), MaxTropical(5.0));
    }

    #[test]
    fn min_addition() {
        let x = MinTropical(5.0);
        let y = MinTropical(1.0);
        assert_eq!(MinTropical::add(x, y), MinTropical(1.0));
    }

    #[test]
    fn min_multiplication() {
        let x = MinTropical(5.0);
        let y = MinTropical(1.0);
        assert_eq!(MinTropical::multiply(x, y), MinTropical(6.0));
    }

    #[test]
    fn min_identity_addition() {
        let ninf = MinTropical::inf();
        let x = MinTropical(5.0);
        assert_eq!(MinTropical::add(x, ninf), MinTropical(5.0));
    }

    #[test]
    fn min_identity_multiplication() {
        let zero = MinTropical(0.0);
        let x = MinTropical(5.0);
        assert_eq!(MinTropical::multiply(x, zero), MinTropical(5.0));
    }
}