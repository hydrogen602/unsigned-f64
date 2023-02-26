mod positive;
#[allow(unused_imports)]
pub use positive::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r = positive::PositiveF64::new(2.0);
        assert!(r.is_some());
        assert!(f64::from(r.unwrap()) == 2.0);

        let n = positive::PositiveF64::new(-2.0);
        assert_eq!(n, None);
    }

    #[test]
    fn try_from() {
        let r = positive::PositiveF64::try_from(2.0);
        assert!(r.is_ok());
        assert!(f64::from(r.unwrap()) == 2.0);

        let n = positive::PositiveF64::try_from(-2.0);
        assert!(n.is_err());
    }

    #[test]
    fn add() {
        let r = positive::PositiveF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = positive::PositiveF64::new(3.0);
        assert!(n.is_some());
        let n = n.unwrap();

        let s = r + n;
        assert!(f64::from(s) == 5.0);

        let s = r + 3.0;
        assert!(s == 5.0);
    }

    #[test]
    fn sub() {
        let r = positive::PositiveF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = positive::PositiveF64::new(3.0);
        assert!(n.is_some());
        let n = n.unwrap();

        let s = r - n;
        assert!(s == -1.0);

        let s = r - 3.0;
        assert!(s == -1.0);
    }

    #[test]
    fn negate() {
        let r = positive::PositiveF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = -r;
        assert!(f64::from(n) == -2.0);
    }

    #[test]
    fn divide() {
        let r = positive::PositiveF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = positive::PositiveF64::new(3.0);
        assert!(n.is_some());
        let n = n.unwrap();

        let s = r / n;
        assert!(f64::from(s) == 2.0 / 3.0);

        let s = r / 3.0;
        assert!(s == 2.0 / 3.0);
    }

    #[test]
    fn test_square() {
        let r = -2.;
        let n = positive::PositiveF64::square(r);
        assert!(f64::from(n) == 4.0);
    }
}
