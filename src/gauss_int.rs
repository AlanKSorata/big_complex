use crate::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// A Gaussian integer a + bi where a, b ∈ ℤ (arbitrary precision integers).
///
/// Gaussian integers extend the integers with the imaginary unit i (i² = -1).
/// They form a Euclidean domain, supporting division with remainder and GCD
/// via the Euclidean algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaussInt {
    real: BigInt,
    imag: BigInt,
}

impl GaussInt {
    pub fn new(real: BigInt, imag: BigInt) -> Self {
        GaussInt { real, imag }
    }

    pub fn from_i64(real: i64, imag: i64) -> Self {
        GaussInt {
            real: BigInt::new(real),
            imag: BigInt::new(imag),
        }
    }

    pub fn real(&self) -> &BigInt {
        &self.real
    }
    pub fn imag(&self) -> &BigInt {
        &self.imag
    }

    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }

    pub fn is_real(&self) -> bool {
        self.imag.is_zero()
    }

    pub fn conjugate(&self) -> Self {
        GaussInt {
            real: self.real.clone(),
            imag: -&self.imag,
        }
    }

    pub fn norm(&self) -> BigInt {
        &self.real * &self.real + &self.imag * &self.imag
    }

    /// Returns true if this Gaussian integer is a unit (+/-1, +/-i).
    pub fn is_unit(&self) -> bool {
        self.norm() == BigInt::new(1)
    }

    /// Raises to a non-negative integer power using exponentiation by squaring.
    pub fn pow_u32(&self, exp: u32) -> Self {
        if exp == 0 {
            return GaussInt::one();
        }
        let mut result = GaussInt::one();
        let mut base = self.clone();
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            e >>= 1;
        }
        result
    }
}

impl Zero for GaussInt {
    fn zero() -> Self {
        GaussInt {
            real: BigInt::zero(),
            imag: BigInt::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl One for GaussInt {
    fn one() -> Self {
        GaussInt {
            real: BigInt::one(),
            imag: BigInt::zero(),
        }
    }
}

// --- Neg ---

impl Neg for GaussInt {
    type Output = GaussInt;

    fn neg(self) -> GaussInt {
        GaussInt {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Neg for &GaussInt {
    type Output = GaussInt;

    fn neg(self) -> GaussInt {
        GaussInt {
            real: -&self.real,
            imag: -&self.imag,
        }
    }
}

// --- Add ---

impl Add for GaussInt {
    type Output = GaussInt;

    fn add(self, other: GaussInt) -> GaussInt {
        GaussInt {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Add for &GaussInt {
    type Output = GaussInt;

    fn add(self, other: &GaussInt) -> GaussInt {
        GaussInt {
            real: &self.real + &other.real,
            imag: &self.imag + &other.imag,
        }
    }
}

impl Add<&GaussInt> for GaussInt {
    type Output = GaussInt;

    fn add(self, other: &GaussInt) -> GaussInt {
        &self + other
    }
}

impl Add<GaussInt> for &GaussInt {
    type Output = GaussInt;

    fn add(self, other: GaussInt) -> GaussInt {
        self + &other
    }
}

// --- Sub ---

impl Sub for GaussInt {
    type Output = GaussInt;

    fn sub(self, other: GaussInt) -> GaussInt {
        GaussInt {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Sub for &GaussInt {
    type Output = GaussInt;

    fn sub(self, other: &GaussInt) -> GaussInt {
        GaussInt {
            real: &self.real - &other.real,
            imag: &self.imag - &other.imag,
        }
    }
}

impl Sub<&GaussInt> for GaussInt {
    type Output = GaussInt;

    fn sub(self, other: &GaussInt) -> GaussInt {
        &self - other
    }
}

impl Sub<GaussInt> for &GaussInt {
    type Output = GaussInt;

    fn sub(self, other: GaussInt) -> GaussInt {
        self - &other
    }
}

// --- Mul ---

impl Mul for GaussInt {
    type Output = GaussInt;

    fn mul(self, other: GaussInt) -> GaussInt {
        // (a+bi)*(c+di) = (ac - bd) + (ad + bc)i
        let ac = self.real.clone() * other.real.clone();
        let bd = self.imag.clone() * other.imag.clone();
        let ad = self.real * other.imag;
        let bc = self.imag * other.real;
        GaussInt {
            real: ac - bd,
            imag: ad + bc,
        }
    }
}

impl Mul for &GaussInt {
    type Output = GaussInt;

    fn mul(self, other: &GaussInt) -> GaussInt {
        // (a+bi)*(c+di) = (ac - bd) + (ad + bc)i
        let ac = &self.real * &other.real;
        let bd = &self.imag * &other.imag;
        let ad = &self.real * &other.imag;
        let bc = &self.imag * &other.real;
        GaussInt {
            real: ac - bd,
            imag: ad + bc,
        }
    }
}

impl Mul<&GaussInt> for GaussInt {
    type Output = GaussInt;

    fn mul(self, other: &GaussInt) -> GaussInt {
        &self * other
    }
}

impl Mul<GaussInt> for &GaussInt {
    type Output = GaussInt;

    fn mul(self, other: GaussInt) -> GaussInt {
        self * &other
    }
}

// --- Division helpers and implementations ---

/// Integer division rounding to nearest, ties away from zero.
fn round_div(a: &BigInt, b: &BigInt) -> BigInt {
    let q = a / b;
    let r = a % b;
    let two_r = BigInt::new(2) * r.abs();
    let b_abs = b.abs();

    if two_r > b_abs || two_r == b_abs {
        // Round away from zero
        if (a.is_negative() && b.is_negative()) || (!a.is_negative() && !b.is_negative()) {
            q + BigInt::one()
        } else {
            q - BigInt::one()
        }
    } else {
        q
    }
}

impl GaussInt {
    /// Divides this Gaussian integer by `other`, returning `(quotient, remainder)`.
    /// Returns `None` if `other` is zero.
    ///
    /// Guarantees `N(remainder) < N(divisor)` (Euclidean domain property).
    pub fn div_rem(&self, other: &Self) -> Option<(Self, Self)> {
        if other.is_zero() {
            return None;
        }

        let conj = other.conjugate();
        let numerator = self * conj; // GaussInt
        let denominator = other.norm(); // BigInt, always positive

        let q_real = round_div(numerator.real(), &denominator);
        let q_imag = round_div(numerator.imag(), &denominator);
        let q = GaussInt::new(q_real, q_imag);
        let r = self - &q * other;

        Some((q, r))
    }
}

impl Div for &GaussInt {
    type Output = GaussInt;

    fn div(self, other: Self) -> GaussInt {
        self.div_rem(other).expect("division by zero").0
    }
}

impl Div for GaussInt {
    type Output = GaussInt;

    fn div(self, other: Self) -> GaussInt {
        self.div_rem(&other).expect("division by zero").0
    }
}

impl Rem for &GaussInt {
    type Output = GaussInt;

    fn rem(self, other: Self) -> GaussInt {
        self.div_rem(other).expect("division by zero").1
    }
}

impl Rem for GaussInt {
    type Output = GaussInt;

    fn rem(self, other: Self) -> GaussInt {
        self.div_rem(&other).expect("division by zero").1
    }
}

impl fmt::Display for GaussInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag.is_zero() {
            write!(f, "{}", self.real)
        } else if self.real.is_zero() {
            if self.imag == BigInt::one() {
                write!(f, "i")
            } else if self.imag == -BigInt::one() {
                write!(f, "-i")
            } else {
                write!(f, "{}i", self.imag)
            }
        } else {
            let sign = if self.imag.is_positive() { "+" } else { "" };
            write!(f, "{}{}{}i", self.real, sign, self.imag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_int_creation() {
        let z = GaussInt::from_i64(3, 4);
        assert_eq!(*z.real(), BigInt::new(3));
        assert_eq!(*z.imag(), BigInt::new(4));
    }

    #[test]
    fn test_gauss_int_display() {
        assert_eq!(GaussInt::from_i64(3, 4).to_string(), "3+4i");
        assert_eq!(GaussInt::from_i64(3, -4).to_string(), "3-4i");
        assert_eq!(GaussInt::from_i64(0, 5).to_string(), "5i");
        assert_eq!(GaussInt::from_i64(7, 0).to_string(), "7");
        assert_eq!(GaussInt::from_i64(0, 1).to_string(), "i");
        assert_eq!(GaussInt::from_i64(0, -1).to_string(), "-i");
        assert_eq!(GaussInt::from_i64(0, 0).to_string(), "0");
    }

    #[test]
    fn test_gauss_int_conjugate() {
        let z = GaussInt::from_i64(3, 4);
        assert_eq!(z.conjugate(), GaussInt::from_i64(3, -4));
        assert_eq!(z.conjugate().conjugate(), z);
    }

    #[test]
    fn test_gauss_int_norm() {
        assert_eq!(GaussInt::from_i64(3, 4).norm(), BigInt::new(25));
        assert_eq!(GaussInt::from_i64(0, 0).norm(), BigInt::new(0));
        assert_eq!(GaussInt::from_i64(1, 0).norm(), BigInt::new(1));
    }

    #[test]
    fn test_gauss_int_arithmetic() {
        let a = GaussInt::from_i64(3, 4);
        let b = GaussInt::from_i64(1, 2);

        assert_eq!(&a + &b, GaussInt::from_i64(4, 6));
        assert_eq!(&a - &b, GaussInt::from_i64(2, 2));
        // (3+4i)*(1+2i) = 3 + 6i + 4i - 8 = -5 + 10i
        assert_eq!(&a * &b, GaussInt::from_i64(-5, 10));
    }

    #[test]
    fn test_gauss_int_neg() {
        assert_eq!(-GaussInt::from_i64(3, 4), GaussInt::from_i64(-3, -4));
    }

    #[test]
    fn test_gauss_int_units() {
        assert!(GaussInt::from_i64(1, 0).is_unit());
        assert!(GaussInt::from_i64(-1, 0).is_unit());
        assert!(GaussInt::from_i64(0, 1).is_unit());
        assert!(GaussInt::from_i64(0, -1).is_unit());
        assert!(!GaussInt::from_i64(2, 0).is_unit());
        assert!(!GaussInt::from_i64(0, 2).is_unit());
    }

    #[test]
    fn test_gauss_int_pow() {
        // (1+i)^2 = 2i
        assert_eq!(GaussInt::from_i64(1, 1).pow_u32(2), GaussInt::from_i64(0, 2));
        // (1+i)^4 = -4
        assert_eq!(GaussInt::from_i64(1, 1).pow_u32(4), GaussInt::from_i64(-4, 0));
        // (1+i)^8 = 16
        assert_eq!(GaussInt::from_i64(1, 1).pow_u32(8), GaussInt::from_i64(16, 0));
        // z^0 = 1
        assert_eq!(GaussInt::from_i64(5, 7).pow_u32(0), GaussInt::one());
    }

    #[test]
    fn test_gauss_int_zero_one() {
        assert!(GaussInt::zero().is_zero());
        assert_eq!(GaussInt::one(), GaussInt::from_i64(1, 0));
    }

    #[test]
    fn test_gauss_int_field_properties() {
        let z = GaussInt::from_i64(3, 4);
        let zero = GaussInt::zero();
        let one = GaussInt::one();

        // z + 0 = z
        assert_eq!(&z + &zero, z);
        // z * 1 = z
        assert_eq!(&z * &one, z);
        // z + (-z) = 0
        assert_eq!(&z + &(-&z), GaussInt::zero());

        // z * conj(z) = N(z) as a real number
        let product = GaussInt::new(z.real().clone(), z.imag().clone()) * z.conjugate();
        assert!(product.is_real());
        assert_eq!(product.real, BigInt::new(25));
    }

    #[test]
    fn test_gauss_int_div_exact() {
        // (3+4i) / (1+2i):
        // (3+4i)(1-2i)/5 = (3-6i+4i+8)/5 = (11-2i)/5 = 2.2 - 0.4i
        // q = 2+0i, r = (3+4i) - 2*(1+2i) = 3+4i-2-4i = 1+0i
        // N(r) = 1 < N(b) = 5
        let a = GaussInt::from_i64(3, 4);
        let b = GaussInt::from_i64(1, 2);
        let (q, r) = a.div_rem(&b).unwrap();
        assert_eq!(q, GaussInt::from_i64(2, 0));
        assert!(r.norm() < b.norm());
        assert_eq!(&q * &b + &r, a);
    }

    #[test]
    fn test_gauss_int_div_different_quadrants() {
        // (-3+4i) / (1-2i):
        // (-3+4i)(1+2i)/5 = (-3-6i+4i-8)/5 = (-11-2i)/5 = -2.2 - 0.4i
        // q = -2+0i
        // r = (-3+4i) - (-2)*(1-2i) = -3+4i+2-4i = -1+0i
        // N(r)=1 < N(b)=5
        let a = GaussInt::from_i64(-3, 4);
        let b = GaussInt::from_i64(1, -2);
        let (q, r) = a.div_rem(&b).unwrap();
        assert!(r.norm() < b.norm(), "N(r)={} >= N(b)={}", r.norm(), b.norm());
        assert_eq!(&q * &b + &r, a);
    }

    #[test]
    fn test_gauss_int_div_by_unit() {
        // (5+7i) / i: multiply by conj(i)/N(i) = -i/1 = -i
        // (5+7i)*(-i) = -5i -7i² = 7 - 5i
        let a = GaussInt::from_i64(5, 7);
        let i = GaussInt::from_i64(0, 1);
        let (q, r) = a.div_rem(&i).unwrap();
        assert_eq!(q, GaussInt::from_i64(7, -5));
        assert!(r.is_zero());
    }

    #[test]
    fn test_gauss_int_div_zero_returns_none() {
        let a = GaussInt::from_i64(1, 1);
        let zero = GaussInt::zero();
        assert!(a.div_rem(&zero).is_none());
    }

    #[test]
    fn test_gauss_int_div_trait() {
        // Just test that the Div trait works
        let a = GaussInt::from_i64(10, 0);
        let b = GaussInt::from_i64(3, 0);
        let q = a / b;
        // 10/3 -> round_div: 10/3=3 rem 1, 2*1=2 <= 3 -> q=3
        assert_eq!(q, GaussInt::from_i64(3, 0));
    }

    #[test]
    fn test_gauss_int_rem_trait() {
        let a = GaussInt::from_i64(10, 0);
        let b = GaussInt::from_i64(3, 0);
        let r = a % b;
        assert_eq!(r, GaussInt::from_i64(1, 0));
    }

    #[test]
    fn test_gauss_int_div_rem_euclidean_property() {
        let cases = vec![
            (GaussInt::from_i64(100, 0), GaussInt::from_i64(7, 0)),
            (GaussInt::from_i64(0, 100), GaussInt::from_i64(0, 7)),
            (GaussInt::from_i64(-100, -100), GaussInt::from_i64(3, 4)),
            (GaussInt::from_i64(1, 1), GaussInt::from_i64(1, 1)),
            (GaussInt::from_i64(7, 5), GaussInt::from_i64(1, 2)),
            (GaussInt::from_i64(-3, -4), GaussInt::from_i64(2, 0)),
        ];
        for (a, b) in cases {
            let (q, r) = a.clone().div_rem(&b).unwrap();
            assert!(r.norm() < b.norm(),
                "N({}) = {} >= N({}) = {} for a={}, b={}", r, r.norm(), b, b.norm(), a, b);
            assert_eq!(&q * &b + &r, a,
                "a = q*b + r failed: {} != {}*{} + {}", a, q, b, r);
        }
    }

    #[test]
    fn test_round_div_negative() {
        // -11/5 = -2.2 -> round to -2
        assert_eq!(round_div(&BigInt::new(-11), &BigInt::new(5)), BigInt::new(-2));
        // -13/5 = -2.6 -> round to -3
        assert_eq!(round_div(&BigInt::new(-13), &BigInt::new(5)), BigInt::new(-3));
        // 11/5 = 2.2 -> round to 2
        assert_eq!(round_div(&BigInt::new(11), &BigInt::new(5)), BigInt::new(2));
        // 13/5 = 2.6 -> round to 3
        assert_eq!(round_div(&BigInt::new(13), &BigInt::new(5)), BigInt::new(3));
        // -11/(-5) = 2.2 -> round to 2
        assert_eq!(round_div(&BigInt::new(-11), &BigInt::new(-5)), BigInt::new(2));
        // -13/(-5) = 2.6 -> round to 3
        assert_eq!(round_div(&BigInt::new(-13), &BigInt::new(-5)), BigInt::new(3));
        // 5/2 = 2.5 (tie) -> round to 3
        assert_eq!(round_div(&BigInt::new(5), &BigInt::new(2)), BigInt::new(3));
        // -5/2 = -2.5 (tie) -> round to -3
        assert_eq!(round_div(&BigInt::new(-5), &BigInt::new(2)), BigInt::new(-3));
    }
}
