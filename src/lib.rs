mod unsigned;
#[allow(unused_imports)]
pub use unsigned::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r = unsigned::UnsignedF64::new(2.0);
        assert!(r.is_some());
        assert!(f64::from(r.unwrap()) == 2.0);

        let n = unsigned::UnsignedF64::new(-2.0);
        assert_eq!(n, None);
    }

    #[test]
    fn try_from() {
        let r = unsigned::UnsignedF64::try_from(2.0);
        assert!(r.is_ok());
        assert!(f64::from(r.unwrap()) == 2.0);

        let n = unsigned::UnsignedF64::try_from(-2.0);
        assert!(n.is_err());
    }

    #[test]
    fn add() {
        let r = unsigned::UnsignedF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = unsigned::UnsignedF64::new(3.0);
        assert!(n.is_some());
        let n = n.unwrap();

        let s = r + n;
        assert!(f64::from(s) == 5.0);

        let s = r + 3.0;
        assert!(s == 5.0);
    }

    #[test]
    fn sub() {
        let r = unsigned::UnsignedF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = unsigned::UnsignedF64::new(3.0);
        assert!(n.is_some());
        let n = n.unwrap();

        let s = r - n;
        assert!(s == -1.0);

        let s = r - 3.0;
        assert!(s == -1.0);
    }

    #[test]
    fn negate() {
        let r = unsigned::UnsignedF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = -r;
        assert!(f64::from(n) == -2.0);
    }

    #[test]
    fn divide() {
        let r = unsigned::UnsignedF64::new(2.0);
        assert!(r.is_some());
        let r = r.unwrap();

        let n = unsigned::UnsignedF64::new(3.0);
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
        let n = unsigned::UnsignedF64::square(r);
        assert!(f64::from(n) == 4.0);
    }

    #[test]
    fn test_neg_zero_issue() {
        let one: crate::UnsignedF64 = crate::UnsignedF64::new(1.0).unwrap();
        let neg_zero = crate::UnsignedF64::new(-0.0).unwrap();

        let neg_inf: crate::UnsignedF64 = one / neg_zero;
        let neg_pi_by_two = neg_inf.atan(); // Return PI/2 for -Inf
        let neg_one = neg_pi_by_two.abs() / neg_pi_by_two;

        println!("neg_one={neg_one}");

        assert!(neg_one >= 0.0);
    }
}
