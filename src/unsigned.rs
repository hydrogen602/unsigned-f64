use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// a float that is guaranteed to be positive or zero
pub struct UnsignedF64(f64);

impl UnsignedF64 {
    pub fn new(value: f64) -> Option<UnsignedF64> {
        if value >= 0.0_f64 {
            Some(UnsignedF64(value))
        } else {
            None
        }
    }

    pub fn square(f: f64) -> UnsignedF64 {
        UnsignedF64(f * f)
    }

    // f64 methods

    pub fn floor(&self) -> UnsignedF64 {
        UnsignedF64(self.0.floor())
    }

    pub fn ceil(&self) -> UnsignedF64 {
        UnsignedF64(self.0.ceil())
    }

    pub fn round(&self) -> UnsignedF64 {
        UnsignedF64(self.0.round())
    }

    pub fn trunc(&self) -> UnsignedF64 {
        UnsignedF64(self.0.trunc())
    }

    pub fn fract(&self) -> UnsignedF64 {
        UnsignedF64(self.0.fract())
    }

    pub fn abs(&self) -> UnsignedF64 {
        UnsignedF64(self.0.abs())
    }

    /// This can return -1 in the case of -0.0.
    /// -0.0 is valid for UnsignedF64 since its zero, but signum considers it negative.
    pub fn signum(&self) -> f64 {
        self.0.signum()
    }

    /// This can return a negative if sign is -0.0.
    /// -0.0 is valid for UnsignedF64 since its zero, but copysign considers it negative.
    pub fn copysign(&self, sign: f64) -> f64 {
        self.0.copysign(sign)
    }

    pub fn mul_add(&self, a: UnsignedF64, b: UnsignedF64) -> UnsignedF64 {
        UnsignedF64(self.0.mul_add(a.0, b.0))
    }

    // // TODO: I'm not sure if div_euclid guarantees a non-negative result.
    // // change this to UnsignedF64 if it does.
    // pub fn div_euclid(&self, rhs: UnsignedF64) -> f64 {
    //     self.0.div_euclid(rhs.0)
    // }

    pub fn rem_euclid(&self, rhs: UnsignedF64) -> UnsignedF64 {
        UnsignedF64(self.0.rem_euclid(rhs.0))
    }

    pub fn powi(&self, n: i32) -> UnsignedF64 {
        UnsignedF64(self.0.powi(n))
    }

    pub fn powf(&self, n: f64) -> UnsignedF64 {
        UnsignedF64(self.0.powf(n))
    }

    /// This should not return NaN.
    pub fn sqrt(&self) -> UnsignedF64 {
        UnsignedF64(self.0.sqrt())
    }

    pub fn exp(&self) -> UnsignedF64 {
        UnsignedF64(self.0.exp())
    }

    pub fn exp2(&self) -> UnsignedF64 {
        UnsignedF64(self.0.exp2())
    }

    pub fn ln(&self) -> f64 {
        self.0.ln()
    }

    pub fn log(&self, base: f64) -> f64 {
        self.0.log(base)
    }

    pub fn log2(&self) -> f64 {
        self.0.log2()
    }

    pub fn log10(&self) -> f64 {
        self.0.log10()
    }

    pub fn cbrt(&self) -> UnsignedF64 {
        UnsignedF64(self.0.cbrt())
    }

    pub fn hypot(&self, other: f64) -> UnsignedF64 {
        UnsignedF64(self.0.hypot(other))
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn tan(&self) -> f64 {
        self.0.tan()
    }

    pub fn asin(&self) -> UnsignedF64 {
        UnsignedF64(self.0.asin())
    }

    pub fn acos(&self) -> UnsignedF64 {
        UnsignedF64(self.0.acos())
    }

    pub fn atan(&self) -> UnsignedF64 {
        UnsignedF64(self.0.atan())
    }

    // TODO: can this be UnsignedF64?
    // pub fn atan2(&self, other: f64) -> UnsignedF64 {
    //     UnsignedF64(self.0.atan2(other))
    // }

    pub fn sin_cos(&self) -> (f64, f64) {
        self.0.sin_cos()
    }

    pub fn exp_m1(&self) -> UnsignedF64 {
        UnsignedF64(self.0.exp_m1())
    }

    pub fn ln_1p(&self) -> UnsignedF64 {
        UnsignedF64(self.0.ln_1p())
    }

    pub fn sinh(&self) -> UnsignedF64 {
        UnsignedF64(self.0.sinh())
    }

    pub fn cosh(&self) -> UnsignedF64 {
        UnsignedF64(self.0.cosh())
    }

    pub fn tanh(&self) -> UnsignedF64 {
        UnsignedF64(self.0.tanh())
    }

    pub fn asinh(&self) -> UnsignedF64 {
        UnsignedF64(self.0.asinh())
    }

    pub fn acosh(&self) -> UnsignedF64 {
        UnsignedF64(self.0.acosh())
    }

    pub fn atanh(&self) -> UnsignedF64 {
        UnsignedF64(self.0.atanh())
    }

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    pub fn is_subnormal(&self) -> bool {
        self.0.is_subnormal()
    }

    pub fn is_normal(&self) -> bool {
        self.0.is_normal()
    }

    pub fn classify(&self) -> std::num::FpCategory {
        self.0.classify()
    }

    /// this may be false if the value is -0.0
    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    /// this may be true if the value is -0.0
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    pub fn recip(&self) -> UnsignedF64 {
        UnsignedF64(self.0.recip())
    }

    pub fn to_degrees(&self) -> UnsignedF64 {
        UnsignedF64(self.0.to_degrees())
    }

    pub fn to_radians(&self) -> UnsignedF64 {
        UnsignedF64(self.0.to_radians())
    }

    pub fn max(self, other: UnsignedF64) -> UnsignedF64 {
        UnsignedF64(self.0.max(other.0))
    }

    pub fn min(self, other: UnsignedF64) -> UnsignedF64 {
        UnsignedF64(self.0.min(other.0))
    }

    // for to/from bits, use f64's methods

    pub fn total_cmp(&self, other: &UnsignedF64) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }

    pub fn clamp(self, min: UnsignedF64, max: UnsignedF64) -> UnsignedF64 {
        UnsignedF64(self.0.clamp(min.0, max.0))
    }

    /// Performs no checking. Useful in scenarios where you know the value is positive and you want to avoid the overhead of checking.
    /// # Safety
    /// The value must be positive or zero.
    pub unsafe fn from_f64_unchecked(value: f64) -> UnsignedF64 {
        UnsignedF64(value)
    }

    pub const PI: UnsignedF64 = UnsignedF64(std::f64::consts::PI);
    pub const E: UnsignedF64 = UnsignedF64(std::f64::consts::E);
    pub const FRAC_1_PI: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_1_PI);
    pub const FRAC_1_SQRT_2: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_1_SQRT_2);
    pub const FRAC_2_PI: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_2_PI);
    pub const FRAC_2_SQRT_PI: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_2_SQRT_PI);
    pub const FRAC_PI_2: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_PI_2);
    pub const FRAC_PI_3: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_PI_3);
    pub const FRAC_PI_4: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_PI_4);
    pub const FRAC_PI_6: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_PI_6);
    pub const FRAC_PI_8: UnsignedF64 = UnsignedF64(std::f64::consts::FRAC_PI_8);
    pub const LN_10: UnsignedF64 = UnsignedF64(std::f64::consts::LN_10);
    pub const LN_2: UnsignedF64 = UnsignedF64(std::f64::consts::LN_2);
    pub const LOG10_E: UnsignedF64 = UnsignedF64(std::f64::consts::LOG10_E);
    pub const LOG2_E: UnsignedF64 = UnsignedF64(std::f64::consts::LOG2_E);
    pub const SQRT_2: UnsignedF64 = UnsignedF64(std::f64::consts::SQRT_2);
    pub const TAU: UnsignedF64 = UnsignedF64(std::f64::consts::TAU);
    pub const EPSILON: UnsignedF64 = UnsignedF64(f64::EPSILON);
    pub const INFINITY: UnsignedF64 = UnsignedF64(f64::INFINITY);

    pub const ZERO: UnsignedF64 = UnsignedF64(0.0);
    pub const ONE: UnsignedF64 = UnsignedF64(1.0);
}

impl PartialEq<f64> for UnsignedF64 {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<f64> for UnsignedF64 {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<UnsignedF64> for f64 {
    fn from(value: UnsignedF64) -> f64 {
        value.0
    }
}

impl TryFrom<f64> for UnsignedF64 {
    type Error = (); // there's nothing to report, its just negative

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(UnsignedF64(value))
        } else {
            Err(())
        }
    }
}

impl AsRef<f64> for UnsignedF64 {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsMut<f64> for UnsignedF64 {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl fmt::Display for UnsignedF64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for UnsignedF64 {
    type Output = UnsignedF64;
    fn add(self, other: UnsignedF64) -> UnsignedF64 {
        UnsignedF64(self.0 + other.0)
    }
}

impl Add<f64> for UnsignedF64 {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        self.0 + rhs
    }
}

impl Sub for UnsignedF64 {
    type Output = f64;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<f64> for UnsignedF64 {
    type Output = f64;
    fn sub(self, rhs: f64) -> Self::Output {
        self.0 - rhs
    }
}

impl Mul for UnsignedF64 {
    type Output = UnsignedF64;
    fn mul(self, rhs: Self) -> Self::Output {
        UnsignedF64(self.0 * rhs.0)
    }
}

impl Mul<f64> for UnsignedF64 {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Div for UnsignedF64 {
    type Output = UnsignedF64;
    fn div(self, rhs: Self) -> Self::Output {
        UnsignedF64(self.0 / rhs.0)
    }
}

impl Div<f64> for UnsignedF64 {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        self.0 / rhs
    }
}

impl Neg for UnsignedF64 {
    type Output = f64;
    fn neg(self) -> Self::Output {
        -self.0
    }
}

impl AddAssign for UnsignedF64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl MulAssign for UnsignedF64 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for UnsignedF64 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
