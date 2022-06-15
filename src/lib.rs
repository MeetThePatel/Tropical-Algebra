#[derive(PartialEq, PartialOrd, Debug)]
struct MaxTropical(f64);

impl MaxTropical {
    fn add(x: MaxTropical, y: MaxTropical) -> MaxTropical {
       MaxTropical(x.x.max(y.x))
    }

    fn multiply(x: MaxTropical, y: MaxTropical) -> MaxTropical {
       MaxTropical(x.x + y.x)
    }

    fn ninf() -> MaxTropical {
        MaxTropical(f64::NEG_INFINITY)
    }
}

#[cfg(test)]
mod tests {
    use crate::MaxTropical;

    #[test]
    fn addition() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(MaxTropical::add(x, y), MaxTropical(5.0));
    }

    #[test]
    fn multiplication() {
        let x = MaxTropical(5.0);
        let y = MaxTropical(1.0);
        assert_eq!(MaxTropical::multiply(x, y), MaxTropical(6.0));
    }

    #[test]
    fn identity_addition() {
        let ninf = MaxTropical::ninf();
        let x = MaxTropical(5.0);
        assert_eq!(MaxTropical::add(x, ninf), MaxTropical(5.0));
    }

    #[test]
    fn identity_multiplication() {
        let zero = MaxTropical(0.0);
        let x = MaxTropical(5.0);
        assert_eq!(MaxTropical::multiply(x, zero), MaxTropical(5.0));
    }

}
