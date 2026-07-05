use crate::BigInt;
use num_traits::One;
use std::fmt;

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
}
