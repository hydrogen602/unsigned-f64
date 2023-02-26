use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
// a float that is guaranteed to be positive or zero
pub struct PositiveF64(f64);

impl PositiveF64 {
    pub fn new(value: f64) -> Option<PositiveF64> {
        if value >= 0.0 {
            Some(PositiveF64(value))
        } else {
            None
        }
    }

    pub fn abs(f: f64) -> PositiveF64 {
        PositiveF64(f.abs())
    }

    pub fn square(f: f64) -> PositiveF64 {
        PositiveF64(f * f)
    }

    /// Performs no checking. Useful in scenarios where you know the value is positive and you want to avoid the overhead of checking.
    pub unsafe fn from_f64_unchecked(value: f64) -> PositiveF64 {
        PositiveF64(value)
    }

    pub const PI: PositiveF64 = PositiveF64(std::f64::consts::PI);
    pub const E: PositiveF64 = PositiveF64(std::f64::consts::E);
    pub const FRAC_1_PI: PositiveF64 = PositiveF64(std::f64::consts::FRAC_1_PI);
    pub const FRAC_1_SQRT_2: PositiveF64 = PositiveF64(std::f64::consts::FRAC_1_SQRT_2);
    pub const FRAC_2_PI: PositiveF64 = PositiveF64(std::f64::consts::FRAC_2_PI);
    pub const FRAC_2_SQRT_PI: PositiveF64 = PositiveF64(std::f64::consts::FRAC_2_SQRT_PI);
    pub const FRAC_PI_2: PositiveF64 = PositiveF64(std::f64::consts::FRAC_PI_2);
    pub const FRAC_PI_3: PositiveF64 = PositiveF64(std::f64::consts::FRAC_PI_3);
    pub const FRAC_PI_4: PositiveF64 = PositiveF64(std::f64::consts::FRAC_PI_4);
    pub const FRAC_PI_6: PositiveF64 = PositiveF64(std::f64::consts::FRAC_PI_6);
    pub const FRAC_PI_8: PositiveF64 = PositiveF64(std::f64::consts::FRAC_PI_8);
    pub const LN_10: PositiveF64 = PositiveF64(std::f64::consts::LN_10);
    pub const LN_2: PositiveF64 = PositiveF64(std::f64::consts::LN_2);
    pub const LOG10_E: PositiveF64 = PositiveF64(std::f64::consts::LOG10_E);
    pub const LOG2_E: PositiveF64 = PositiveF64(std::f64::consts::LOG2_E);
    pub const SQRT_2: PositiveF64 = PositiveF64(std::f64::consts::SQRT_2);
    pub const TAU: PositiveF64 = PositiveF64(std::f64::consts::TAU);
    pub const EPSILON: PositiveF64 = PositiveF64(f64::EPSILON);
    pub const INFINITY: PositiveF64 = PositiveF64(f64::INFINITY);

    pub const ZERO: PositiveF64 = PositiveF64(0.0);
    pub const ONE: PositiveF64 = PositiveF64(1.0);
}

impl PartialEq<f64> for PositiveF64 {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<f64> for PositiveF64 {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<PositiveF64> for f64 {
    fn from(value: PositiveF64) -> f64 {
        value.0
    }
}

impl TryFrom<f64> for PositiveF64 {
    type Error = (); // there's nothing to report, its just negative

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(PositiveF64(value))
        } else {
            Err(())
        }
    }
}

impl AsRef<f64> for PositiveF64 {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsMut<f64> for PositiveF64 {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl fmt::Display for PositiveF64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for PositiveF64 {
    type Output = PositiveF64;
    fn add(self, other: PositiveF64) -> PositiveF64 {
        PositiveF64(self.0 + other.0)
    }
}

impl Add<f64> for PositiveF64 {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        self.0 + rhs
    }
}

impl Sub for PositiveF64 {
    type Output = f64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<f64> for PositiveF64 {
    type Output = f64;
    fn sub(self, rhs: f64) -> Self::Output {
        self.0 - rhs
    }
}

impl Mul for PositiveF64 {
    type Output = PositiveF64;
    fn mul(self, rhs: Self) -> Self::Output {
        PositiveF64(self.0 * rhs.0)
    }
}

impl Mul<f64> for PositiveF64 {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Div for PositiveF64 {
    type Output = PositiveF64;
    fn div(self, rhs: Self) -> Self::Output {
        PositiveF64(self.0 / rhs.0)
    }
}

impl Div<f64> for PositiveF64 {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        self.0 / rhs
    }
}

impl Neg for PositiveF64 {
    type Output = f64;
    fn neg(self) -> Self::Output {
        -self.0
    }
}

impl AddAssign for PositiveF64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl MulAssign for PositiveF64 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for PositiveF64 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
