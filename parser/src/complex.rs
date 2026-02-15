//! The [`Complex`] type

use std::{fmt, ops::*};

use bytemuck::{Pod, Zeroable};
use serde::*;

use crate::{ComplexNum, num_consts::E};

/// Uiua's complex number type
#[derive(Clone, Copy, PartialOrd, Default, Serialize, Deserialize, Pod, Zeroable)]
#[serde(from = "(ComplexNum, ComplexNum)", into = "(ComplexNum, ComplexNum)")]
#[repr(C)]
pub struct Complex {
    /// The real part
    pub re: ComplexNum,
    /// The imaginary part
    pub im: ComplexNum,
}

impl From<(ComplexNum, ComplexNum)> for Complex {
    fn from((re, im): (ComplexNum, ComplexNum)) -> Self {
        Self { re, im }
    }
}

impl From<Complex> for (ComplexNum, ComplexNum) {
    fn from(c: Complex) -> Self {
        (c.re, c.im)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl Eq for Complex {}

impl Complex {
    /// The complex number 0 + 0i
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    /// The complex number 1 + 0i
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    /// The complex number 0 + 1i
    pub const I: Self = Self { re: 0.0, im: 1.0 };
    /// Create a new complex number
    pub fn new(re: ComplexNum, im: ComplexNum) -> Self {
        Self { re, im }
    }
    /// Get the minimum of the real and imaginary parts of two complex numbers, ignoring NaN
    pub fn min(self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();
        Self {
            re: self.re.min(rhs.re),
            im: self.im.min(rhs.im),
        }
    }
    /// Get the maximum of the real and imaginary parts of two complex numbers, ignoring NaN
    pub fn max(self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();
        Self {
            re: self.re.max(rhs.re),
            im: self.im.max(rhs.im),
        }
    }
    /// Get the floor of the real and imaginary parts of a complex number
    pub fn floor(self) -> Self {
        Self {
            re: self.re.floor(),
            im: self.im.floor(),
        }
    }
    /// Get the ceiling of the real and imaginary parts of a complex number
    pub fn ceil(self) -> Self {
        Self {
            re: self.re.ceil(),
            im: self.im.ceil(),
        }
    }
    /// Round the real and imaginary parts of a complex number
    pub fn round(self) -> Self {
        Self {
            re: self.re.round(),
            im: self.im.round(),
        }
    }
    /// Get the absolute value of a complex number
    pub fn abs(self) -> ComplexNum {
        // Do not use `self.re.hypot(self.im)` because it is slower, especially on WASM
        (self.re * self.re + self.im * self.im).sqrt()
    }
    /// Get the arctangent of a complex number
    pub fn atan2(self, x: impl Into<Self>) -> Complex {
        let y = self;
        let x = x.into();
        -Complex::I * ((x + Complex::I * y) / (y * y + x * x).sqrt()).ln()
    }
    /// Normalize a complex number
    pub fn normalize(self) -> Self {
        let len = self.abs();
        if len == 0.0 { Self::ZERO } else { self / len }
    }
    /// Calculate the principal value of the complex number
    pub fn arg(self) -> ComplexNum {
        self.im.atan2(self.re)
    }
    /// Convert a complex number to polar coordinates
    pub fn to_polar(self) -> (ComplexNum, ComplexNum) {
        (self.abs(), self.arg())
    }
    /// Convert polar coordinates to a complex number
    pub fn from_polar(r: ComplexNum, theta: ComplexNum) -> Self {
        r * Self::new(theta.cos(), theta.sin())
    }
    /// Raise a complex number to a complex power
    pub fn powc(self, power: impl Into<Self>) -> Self {
        let power = power.into();
        if power.im == 0.0 {
            return self.powf(power.re);
        }
        let (r, theta) = self.to_polar();
        ((r.ln() + Self::I * theta) * power).exp()
    }
    /// Raise a complex number to a real power
    pub fn powf(self, power: ComplexNum) -> Self {
        if power == 0.0 {
            return Self::ONE;
        }
        if power.fract() == 0.0 && self.im == 0.0 {
            return self.re.powf(power).into();
        }
        let (r, theta) = self.to_polar();
        Self::from_polar(r.powf(power), theta * power)
    }
    /// Calculate the exponential of a complex number
    pub fn exp(self) -> Self {
        Self::from_polar(E.powf(self.re), self.im)
    }
    /// Calculate the natural logarithm of a complex number
    pub fn ln(self) -> Self {
        let (r, theta) = self.to_polar();
        Self::new(r.ln(), theta)
    }
    /// Calculate the logarithm of a complex number
    pub fn log(self, base: impl Into<Self>) -> Self {
        let base = base.into();
        Self::new(self.abs().ln(), self.arg()) / (Self::new(base.abs().ln(), base.arg()))
    }
    /// Calculate the square root of a complex number
    pub fn sqrt(self) -> Self {
        if self.im == 0.0 {
            return if self.re >= 0.0 {
                Self::new(self.re.sqrt(), 0.0)
            } else {
                Self::new(0.0, self.re.abs().sqrt())
            };
        }
        let (r, theta) = self.to_polar();
        Self::from_polar(r.sqrt(), theta / 2.0)
    }
    /// Calculate the sine of a complex number
    pub fn sin(self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }
    /// Calculate the cosine of a complex number
    pub fn cos(self) -> Self {
        Self::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }
    /// Calculate the arc sine of a complex number
    pub fn asin(self) -> Self {
        -Self::I * ((Self::ONE - self * self).sqrt() + Self::I * self).ln()
    }
    /// Calculate the arc cosine of a complex number
    pub fn acos(self) -> Self {
        -Self::I * (Self::I * (Self::ONE - self * self).sqrt() + self).ln()
    }
    /// Check if either real or imaginary part is NaN
    pub fn is_nan(&self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }
    /// Get the complex number as a real number
    pub fn into_real(self) -> Option<ComplexNum> {
        if self.im.abs() < ComplexNum::EPSILON {
            Some(self.re)
        } else {
            None
        }
    }
    /// Multiply by another complex number, with 0 × ∞ = 0
    pub fn safe_mul(self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();
        Self {
            re: safe_mul(self.re, rhs.re) - safe_mul(self.im, rhs.im),
            im: safe_mul(self.re, rhs.im) + safe_mul(self.im, rhs.re),
        }
    }
    pub fn recip(self) -> Self {
        Self::ONE / self
    }
}

fn safe_mul(a: ComplexNum, b: ComplexNum) -> ComplexNum {
    if a.is_infinite() && b == 0.0 || a == 0.0 && b.is_infinite() {
        0.0
    } else {
        a * b
    }
}

impl From<ComplexNum> for Complex {
    fn from(re: ComplexNum) -> Self {
        Self { re, im: 0.0 }
    }
}

#[cfg(feature = "f32_num")]
impl From<f64> for Complex {
    fn from(re: f64) -> Self {
        Self {
            re: re as ComplexNum,
            im: 0.0,
        }
    }
}

impl From<u8> for Complex {
    fn from(value: u8) -> Self {
        ComplexNum::from(value).into()
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im == 0.0 {
            self.re.fmt(f)
        } else {
            write!(f, "{}r{}i", self.re, self.im)
        }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add<ComplexNum> for Complex {
    type Output = Self;
    fn add(self, rhs: ComplexNum) -> Self::Output {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

impl Add<Complex> for ComplexNum {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self + rhs.re,
            im: rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<ComplexNum> for Complex {
    type Output = Self;
    fn sub(self, rhs: ComplexNum) -> Self::Output {
        Self {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

impl Sub<Complex> for ComplexNum {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self - rhs.re,
            im: -rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<ComplexNum> for Complex {
    type Output = Self;
    fn mul(self, rhs: ComplexNum) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Mul<Complex> for ComplexNum {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl Div<ComplexNum> for Complex {
    type Output = Self;
    fn div(self, rhs: ComplexNum) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Div<Complex> for ComplexNum {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex {
            re: self * rhs.re / denom,
            im: -self * rhs.im / denom,
        }
    }
}

impl Rem for Complex {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self - (self / rhs).floor() * rhs
    }
}

impl Rem<ComplexNum> for Complex {
    type Output = Self;
    fn rem(self, rhs: ComplexNum) -> Self::Output {
        Self {
            re: self.re.rem_euclid(rhs),
            im: self.im.rem_euclid(rhs),
        }
    }
}

impl Rem<Complex> for ComplexNum {
    type Output = Complex;
    fn rem(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self % rhs.re,
            im: self % rhs.im,
        }
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    Complex: Add<T, Output = Complex>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign<T> for Complex
where
    Complex: Sub<T, Output = Complex>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Complex
where
    Complex: Mul<T, Output = Complex>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign<T> for Complex
where
    Complex: Div<T, Output = Complex>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}
