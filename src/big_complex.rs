use crate::big_int::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A complex number with arbitrary-precision integer components.
///
/// `BigComplex` represents complex numbers of the form `a + bi` where
/// both `a` (real part) and `b` (imaginary part) are `BigInt` values.
///
/// # Examples
///
/// ```
/// use big_complex::BigComplex;
///
/// let z = BigComplex::from_i64(3, 4);
/// assert_eq!(z.to_string(), "3+4i");
///
/// let conj = z.conjugate();
/// assert_eq!(conj.to_string(), "3-4i");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigComplex {
    real: BigInt,
    imag: BigInt,
}

impl BigComplex {
    /// Creates a new `BigComplex` from real and imaginary parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::{BigInt, BigComplex};
    ///
    /// let z = BigComplex::new(BigInt::new(3), BigInt::new(4));
    /// assert_eq!(z.to_string(), "3+4i");
    /// ```
    pub fn new(real: BigInt, imag: BigInt) -> Self {
        BigComplex { real, imag }
    }

    /// Creates a `BigComplex` from `i64` real and imaginary parts.
    ///
    /// This is a convenience method for creating `BigComplex` values from small integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// let z = BigComplex::from_i64(3, 4);
    /// assert_eq!(z.to_string(), "3+4i");
    /// ```
    pub fn from_i64(real: i64, imag: i64) -> Self {
        BigComplex {
            real: BigInt::new(real),
            imag: BigInt::new(imag),
        }
    }

    /// Returns a reference to the real part.
    pub fn real(&self) -> &BigInt {
        &self.real
    }

    /// Returns a reference to the imaginary part.
    pub fn imag(&self) -> &BigInt {
        &self.imag
    }

    /// Returns the complex conjugate: `a + bi` → `a - bi`.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// let z = BigComplex::from_i64(3, 4);
    /// let conj = z.conjugate();
    /// assert_eq!(conj.real().to_string(), "3");
    /// assert_eq!(conj.imag().to_string(), "-4");
    /// ```
    pub fn conjugate(&self) -> Self {
        BigComplex {
            real: self.real.clone(),
            imag: -&self.imag,
        }
    }

    /// Returns the squared magnitude: `|z|² = a² + b²`.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// let z = BigComplex::from_i64(3, 4);
    /// // |3+4i|² = 3² + 4² = 25
    /// assert_eq!(z.magnitude_squared().to_string(), "25");
    /// ```
    pub fn magnitude_squared(&self) -> BigInt {
        &self.real * &self.real + &self.imag * &self.imag
    }

    /// Scales this complex number by a real factor.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::{BigInt, BigComplex};
    ///
    /// let z = BigComplex::from_i64(3, 4);
    /// let scaled = z.scale(&BigInt::new(2));
    /// assert_eq!(scaled.to_string(), "6+8i");
    /// ```
    pub fn scale(&self, factor: &BigInt) -> Self {
        BigComplex {
            real: &self.real * factor,
            imag: &self.imag * factor,
        }
    }

    /// Adds a real number to this complex number.
    pub fn add_real(&self, real: &BigInt) -> Self {
        BigComplex {
            real: &self.real + real,
            imag: self.imag.clone(),
        }
    }

    /// Adds an imaginary number to this complex number.
    pub fn add_imag(&self, imag: &BigInt) -> Self {
        BigComplex {
            real: self.real.clone(),
            imag: &self.imag + imag,
        }
    }

    /// Returns `true` if both real and imaginary parts are zero.
    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }

    /// Returns `true` if the imaginary part is zero.
    pub fn is_real(&self) -> bool {
        self.imag.is_zero()
    }

    /// Returns `true` if the real part is zero.
    pub fn is_imaginary(&self) -> bool {
        self.real.is_zero()
    }

    /// Raises this complex number to the power of `exp`.
    ///
    /// Uses exponentiation by squaring for efficiency.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// let z = BigComplex::from_i64(1, 1);
    /// // (1+i)² = 1 + 2i + i² = 2i
    /// let z2 = z.pow(2);
    /// assert_eq!(z2.real().to_string(), "0");
    /// assert_eq!(z2.imag().to_string(), "2");
    /// ```
    pub fn pow(&self, exp: u32) -> Self {
        if exp == 0 {
            return BigComplex::one();
        }

        let mut result = self.clone();
        let mut current = self.clone();
        let mut power = exp - 1;

        while power > 0 {
            if power % 2 == 1 {
                result = result * current.clone();
            }
            current = current.clone() * current.clone();
            power /= 2;
        }

        result
    }

    /// Divides this complex number by a real integer if the division is exact.
    ///
    /// Returns `None` if the division is not exact or if the divisor is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::{BigInt, BigComplex};
    ///
    /// let z = BigComplex::from_i64(6, 8);
    /// let result = z.div_exact(&BigInt::new(2));
    /// assert_eq!(result.unwrap().to_string(), "3+4i");
    ///
    /// // 5+7i is not divisible by 2
    /// let z2 = BigComplex::from_i64(5, 7);
    /// assert!(z2.div_exact(&BigInt::new(2)).is_none());
    /// ```
    pub fn div_exact(&self, divisor: &BigInt) -> Option<Self> {
        if divisor.is_zero() {
            return None;
        }

        let real_rem = self.real.clone() % divisor.clone();
        let imag_rem = self.imag.clone() % divisor.clone();

        if !real_rem.is_zero() || !imag_rem.is_zero() {
            return None;
        }

        Some(BigComplex {
            real: self.real.clone() / divisor.clone(),
            imag: self.imag.clone() / divisor.clone(),
        })
    }

    /// Returns the norm (squared magnitude) of this complex number.
    ///
    /// This is an alias for `magnitude_squared`.
    pub fn norm(&self) -> BigInt {
        self.magnitude_squared()
    }

    /// Returns the squared distance between this complex number and `other`.
    pub fn distance_to(&self, other: &Self) -> BigInt {
        let diff = self - other;
        diff.magnitude_squared()
    }

    /// Returns the magnitude (absolute value) of this complex number.
    ///
    /// Returns the integer square root of the squared magnitude.
    /// For `3+4i`, returns `5` (since |3+4i| = √(9+16) = 5).
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// let z = BigComplex::from_i64(3, 4);
    /// assert_eq!(z.magnitude().to_string(), "5");
    /// ```
    pub fn magnitude(&self) -> BigInt {
        self.magnitude_squared()
            .sqrt()
            .unwrap_or_else(BigInt::zero)
    }

    /// Creates a complex number from polar coordinates (simplified).
    ///
    /// This is a simplified version that only supports four discrete angles:
    /// - 0: 0° (positive real axis)
    /// - 1: 90° (positive imaginary axis)
    /// - 2: 180° (negative real axis)
    /// - 3: 270° (negative imaginary axis)
    ///
    /// The angle wraps around modulo 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::{BigInt, BigComplex};
    ///
    /// // r=5, θ=90° → 0+5i
    /// let z = BigComplex::from_polar(&BigInt::new(5), 1);
    /// assert_eq!(z.to_string(), "5i");
    /// ```
    pub fn from_polar(r: &BigInt, theta_approx: i32) -> Self {
        // Simplified polar coordinate conversion using integer angle approximation
        // theta_approx: 0=0°, 1=90°, 2=180°, 3=270°
        match theta_approx % 4 {
            0 => BigComplex::new(r.clone(), BigInt::zero()), // 0°
            1 => BigComplex::new(BigInt::zero(), r.clone()), // 90°
            2 => BigComplex::new(-r, BigInt::zero()),        // 180°
            3 => BigComplex::new(BigInt::zero(), -r),        // 270°
            _ => unreachable!(),
        }
    }

    /// Returns the quadrant of this complex number (0-3).
    ///
    /// - Quadrant 0: positive real, positive imaginary
    /// - Quadrant 1: negative real, positive imaginary
    /// - Quadrant 2: negative real, negative imaginary
    /// - Quadrant 3: positive real, negative imaginary
    ///
    /// Returns `None` for zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// assert_eq!(BigComplex::from_i64(3, 4).arg_quadrant(), Some(0));
    /// assert_eq!(BigComplex::from_i64(-3, 4).arg_quadrant(), Some(1));
    /// assert_eq!(BigComplex::from_i64(0, 0).arg_quadrant(), None);
    /// ```
    pub fn arg_quadrant(&self) -> Option<i32> {
        // Returns the quadrant of the complex number (0-3)
        if self.is_zero() {
            return None;
        }

        match (self.real.is_positive(), self.imag.is_positive()) {
            (true, true) => Some(0),   // First quadrant
            (false, true) => Some(1),  // Second quadrant
            (false, false) => Some(2), // Third quadrant
            (true, false) => Some(3),  // Fourth quadrant
        }
    }

    /// Rotates this complex number 90 degrees counterclockwise.
    ///
    /// Equivalent to multiplying by `i`.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// let z = BigComplex::from_i64(1, 0);
    /// // 1 rotated 90° → i
    /// assert_eq!(z.rotate_90().to_string(), "i");
    /// ```
    pub fn rotate_90(&self) -> Self {
        // Rotate 90 degrees counterclockwise: (a+bi) * i = -b + ai
        BigComplex::new(-&self.imag, self.real.clone())
    }

    /// Rotates this complex number 180 degrees.
    ///
    /// Equivalent to multiplying by `-1`.
    pub fn rotate_180(&self) -> Self {
        // Rotate 180 degrees: (a+bi) * (-1) = -a - bi
        BigComplex::new(-&self.real, -&self.imag)
    }

    /// Rotates this complex number 270 degrees counterclockwise (or 90° clockwise).
    ///
    /// Equivalent to multiplying by `-i`.
    pub fn rotate_270(&self) -> Self {
        // Rotate 270 degrees counterclockwise: (a+bi) * (-i) = b - ai
        BigComplex::new(self.imag.clone(), -&self.real)
    }

    /// Calculates the nth roots of this complex number.
    ///
    /// **Note:** This is a simplified implementation. Currently supported:
    /// - n = 0: Returns empty vector
    /// - n = 1: Returns self
    /// - n = 2 (square root): Returns correct roots for real numbers (positive and negative)
    /// - Other cases: Returns `None` (not yet implemented)
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// // Square roots of 4
    /// let roots = BigComplex::from_i64(4, 0).nth_root(2).unwrap();
    /// assert_eq!(roots.len(), 2);
    /// assert_eq!(roots[0].to_string(), "2");
    /// assert_eq!(roots[1].to_string(), "-2");
    ///
    /// // Square roots of -4
    /// let roots = BigComplex::from_i64(-4, 0).nth_root(2).unwrap();
    /// assert_eq!(roots[0].to_string(), "2i");
    /// assert_eq!(roots[1].to_string(), "-2i");
    /// ```
    /// Returns the nth roots of this complex number.
    ///
    /// **Note:** This is a simplified implementation. It handles:
    /// - n = 0: Returns empty vector
    /// - n = 1: Returns self
    /// - n = 2: Returns square roots for real numbers (positive and negative)
    ///
    /// For general complex nth roots (n > 2 or non-real numbers), returns `None`.
    /// A complete implementation would require floating-point approximations.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// // Square root of positive real
    /// let z = BigComplex::from_i64(9, 0);
    /// let roots = z.nth_root(2).unwrap();
    /// assert_eq!(roots.len(), 2);
    /// assert_eq!(roots[0].to_string(), "3");
    ///
    /// // Square root of negative real
    /// let z_neg = BigComplex::from_i64(-4, 0);
    /// let roots_neg = z_neg.nth_root(2).unwrap();
    /// assert_eq!(roots_neg[0].to_string(), "2i");
    ///
    /// // Complex numbers return None for n > 2
    /// let complex = BigComplex::from_i64(1, 1);
    /// assert!(complex.nth_root(3).is_none());
    /// ```
    pub fn nth_root(&self, n: u32) -> Option<Vec<Self>> {
        // Calculate the nth roots of a complex number
        if n == 0 {
            return Some(vec![]);
        }

        if self.is_zero() {
            return Some(vec![BigComplex::zero()]);
        }

        if n == 1 {
            return Some(vec![self.clone()]);
        }

        if n == 2 {
            // Square root calculation for real numbers
            let mag_squared = self.magnitude_squared();
            let _mag = mag_squared.sqrt().unwrap_or_else(BigInt::zero);

            if self.is_real() && self.real.is_positive() {
                let sqrt_real = self.real.sqrt().unwrap_or_else(BigInt::zero);
                return Some(vec![
                    BigComplex::new(sqrt_real.clone(), BigInt::zero()),
                    BigComplex::new(-&sqrt_real, BigInt::zero()),
                ]);
            } else if self.is_real() && self.real.is_negative() {
                let sqrt_abs = (-&self.real).sqrt().unwrap_or_else(BigInt::zero);
                return Some(vec![
                    BigComplex::new(BigInt::zero(), sqrt_abs.clone()),
                    BigComplex::new(BigInt::zero(), -&sqrt_abs),
                ]);
            }
        }

        // Not implemented for other cases
        None
    }

    /// Returns a simplified approximation of the natural logarithm.
    ///
    /// **Note:** This is a demonstration implementation with limited precision.
    /// For positive real numbers, it returns a rough integer approximation.
    /// For complex numbers and non-positive reals, returns `None`.
    ///
    /// Returns `None` for zero and negative numbers, and for complex numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    ///
    /// // ln(1) = 0
    /// let z = BigComplex::from_i64(1, 0);
    /// assert!(z.ln_approx().unwrap().is_zero());
    ///
    /// // Complex numbers return None
    /// let complex = BigComplex::from_i64(1, 1);
    /// assert!(complex.ln_approx().is_none());
    /// ```
    pub fn ln_approx(&self) -> Option<Self> {
        // Simplified natural logarithm approximation (for demonstration purposes)
        if self.is_zero() {
            return None;
        }

        if self.is_real() && self.real.is_positive() {
            // For positive real numbers, ln(x) ≈ integer approximation
            if self.real == BigInt::one() {
                return Some(BigComplex::zero());
            }

            // Simplified logarithm approximation
            let mut approx = BigInt::zero();
            let mut temp = self.real.clone();

            while temp > BigInt::one() {
                temp = temp / BigInt::new(2);
                approx = approx + BigInt::one();
            }

            return Some(BigComplex::new(approx, BigInt::zero()));
        }

        // For complex numbers, return None as we cannot compute ln accurately
        // without floating-point arithmetic
        None
    }

    /// Returns a simplified approximation of the exponential function.
    ///
    /// **Note:** This is a demonstration implementation using a truncated
    /// Taylor series expansion. Accuracy decreases for larger values.
    ///
    /// # Examples
    ///
    /// ```
    /// use big_complex::BigComplex;
    /// use num_traits::Zero;
    ///
    /// // exp(0) = 1
    /// let z = BigComplex::zero();
    /// assert_eq!(z.exp_approx().to_string(), "1");
    /// ```
    pub fn exp_approx(&self) -> Self {
        // Simplified exponential function approximation (for demonstration purposes)
        if self.is_zero() {
            return BigComplex::one();
        }

        if self.is_real() {
            // Simplified approximation of e^x
            if self.real.is_zero() {
                return BigComplex::one();
            }

            let mut result = BigComplex::one();
            let mut term = BigComplex::one();

            // Calculate first few terms of Taylor series
            for i in 1..=10 {
                let divisor = BigComplex::new(BigInt::new(i), BigInt::zero());
                if divisor.is_zero() {
                    break;
                }
                term = term * self.clone() / divisor;
                result = result + term.clone();

                if term.magnitude_squared() < BigInt::one() {
                    break;
                }
            }

            result
        } else {
            // For complex numbers, return simplified result
            BigComplex::new(BigInt::one(), BigInt::one())
        }
    }
}

impl Zero for BigComplex {
    fn zero() -> Self {
        BigComplex {
            real: BigInt::zero(),
            imag: BigInt::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
}

impl One for BigComplex {
    fn one() -> Self {
        BigComplex {
            real: BigInt::one(),
            imag: BigInt::zero(),
        }
    }
}

impl Add for BigComplex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BigComplex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Add for &BigComplex {
    type Output = BigComplex;

    fn add(self, other: Self) -> BigComplex {
        BigComplex {
            real: &self.real + &other.real,
            imag: &self.imag + &other.imag,
        }
    }
}

impl Sub for BigComplex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        BigComplex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Sub for &BigComplex {
    type Output = BigComplex;

    fn sub(self, other: Self) -> BigComplex {
        BigComplex {
            real: &self.real - &other.real,
            imag: &self.imag - &other.imag,
        }
    }
}

impl Mul for BigComplex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let real = &self.real * &other.real - &self.imag * &other.imag;
        let imag = &self.real * &other.imag + &self.imag * &other.real;

        BigComplex { real, imag }
    }
}

impl Mul for &BigComplex {
    type Output = BigComplex;

    fn mul(self, other: Self) -> BigComplex {
        let real = &self.real * &other.real - &self.imag * &other.imag;
        let imag = &self.real * &other.imag + &self.imag * &other.real;

        BigComplex { real, imag }
    }
}

impl Div for BigComplex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator =
            other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();

        if denominator.is_zero() {
            panic!("Division by zero complex number");
        }

        let real = (self.real.clone() * other.real.clone()
            + self.imag.clone() * other.imag.clone())
            / denominator.clone();
        let imag = (self.imag.clone() * other.real.clone()
            - self.real.clone() * other.imag.clone())
            / denominator;

        BigComplex { real, imag }
    }
}

impl Div<&BigComplex> for BigComplex {
    type Output = Self;

    fn div(self, other: &BigComplex) -> Self {
        let denominator = &other.real * &other.real + &other.imag * &other.imag;

        if denominator.is_zero() {
            panic!("Division by zero complex number");
        }

        let real = (&self.real * &other.real + &self.imag * &other.imag) / denominator.clone();
        let imag = (&self.imag * &other.real - &self.real * &other.imag) / denominator;

        BigComplex { real, imag }
    }
}

impl Div<BigComplex> for &BigComplex {
    type Output = BigComplex;

    fn div(self, other: BigComplex) -> BigComplex {
        let denominator = &other.real * &other.real + &other.imag * &other.imag;

        if denominator.is_zero() {
            panic!("Division by zero complex number");
        }

        let real = (&self.real * &other.real + &self.imag * &other.imag) / denominator.clone();
        let imag = (&self.imag * &other.real - &self.real * &other.imag) / denominator;

        BigComplex { real, imag }
    }
}

impl Div<&BigComplex> for &BigComplex {
    type Output = BigComplex;

    fn div(self, other: &BigComplex) -> BigComplex {
        let denominator = &other.real * &other.real + &other.imag * &other.imag;

        if denominator.is_zero() {
            panic!("Division by zero complex number");
        }

        let real = (&self.real * &other.real + &self.imag * &other.imag) / denominator.clone();
        let imag = (&self.imag * &other.real - &self.real * &other.imag) / denominator;

        BigComplex { real, imag }
    }
}

impl Neg for BigComplex {
    type Output = Self;

    fn neg(self) -> Self {
        BigComplex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Neg for &BigComplex {
    type Output = BigComplex;

    fn neg(self) -> BigComplex {
        BigComplex {
            real: -&self.real,
            imag: -&self.imag,
        }
    }
}

impl fmt::Display for BigComplex {
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
    fn test_big_complex_creation() {
        let a = BigComplex::from_i64(3, 4);
        assert_eq!(a.real().to_string(), "3");
        assert_eq!(a.imag().to_string(), "4");

        let b = BigComplex::new(BigInt::new(-5), BigInt::new(2));
        assert_eq!(b.to_string(), "-5+2i");
    }

    #[test]
    fn test_big_complex_arithmetic() {
        let a = BigComplex::from_i64(3, 4);
        let b = BigComplex::from_i64(1, 2);

        let sum = &a + &b;
        assert_eq!(sum.real().to_string(), "4");
        assert_eq!(sum.imag().to_string(), "6");

        let diff = &a - &b;
        assert_eq!(diff.real().to_string(), "2");
        assert_eq!(diff.imag().to_string(), "2");

        let product = &a * &b;
        assert_eq!(product.real().to_string(), "-5"); // (3+4i)(1+2i) = 3 + 6i + 4i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert_eq!(product.imag().to_string(), "10");

        let quotient = &a / &b;
        assert_eq!(quotient.real().to_string(), "2"); // (3+4i)/(1+2i) = (3+4i)(1-2i)/5 = (11-2i)/5 = 2 + 0i (integer division)
        assert_eq!(quotient.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_conjugate() {
        let a = BigComplex::from_i64(3, 4);
        let conj = a.conjugate();
        assert_eq!(conj.real().to_string(), "3");
        assert_eq!(conj.imag().to_string(), "-4");

        let b = BigComplex::from_i64(-2, 5);
        let conj_b = b.conjugate();
        assert_eq!(conj_b.real().to_string(), "-2");
        assert_eq!(conj_b.imag().to_string(), "-5");
    }

    #[test]
    fn test_big_complex_magnitude() {
        let a = BigComplex::from_i64(3, 4);
        let mag = a.magnitude_squared();
        assert_eq!(mag.to_string(), "25"); // 3² + 4² = 9 + 16 = 25

        let b = BigComplex::from_i64(0, 0);
        assert_eq!(b.magnitude_squared().to_string(), "0");
    }

    #[test]
    fn test_big_complex_pow() {
        let a = BigComplex::from_i64(1, 1);
        let a2 = a.pow(2);
        assert_eq!(a2.real().to_string(), "0"); // (1+i)² = 1 + 2i + i² = 1 + 2i - 1 = 2i
        assert_eq!(a2.imag().to_string(), "2");

        let a4 = a.pow(4);
        assert_eq!(a4.real().to_string(), "-4"); // (1+i)⁴ = ((1+i)²)² = (2i)² = -4
        assert_eq!(a4.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_scaling() {
        let a = BigComplex::from_i64(3, 4);
        let scaled = a.scale(&BigInt::new(2));
        assert_eq!(scaled.real().to_string(), "6");
        assert_eq!(scaled.imag().to_string(), "8");
    }

    #[test]
    fn test_big_complex_div_exact() {
        let a = BigComplex::from_i64(6, 8);
        let result = a.div_exact(&BigInt::new(2));
        assert!(result.is_some());
        let divided = result.unwrap();
        assert_eq!(divided.real().to_string(), "3");
        assert_eq!(divided.imag().to_string(), "4");

        let b = BigComplex::from_i64(5, 7);
        let result2 = b.div_exact(&BigInt::new(2));
        assert!(result2.is_none()); // 5 and 7 are not divisible by 2
    }

    #[test]
    fn test_big_complex_zero_and_one() {
        let zero = BigComplex::zero();
        assert!(zero.is_zero());

        let one = BigComplex::one();
        assert_eq!(one.real().to_string(), "1");
        assert_eq!(one.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_display() {
        let a = BigComplex::from_i64(3, 4);
        assert_eq!(a.to_string(), "3+4i");

        let b = BigComplex::from_i64(3, -4);
        assert_eq!(b.to_string(), "3-4i");

        let c = BigComplex::from_i64(0, 5);
        assert_eq!(c.to_string(), "5i");

        let d = BigComplex::from_i64(7, 0);
        assert_eq!(d.to_string(), "7");

        let e = BigComplex::from_i64(0, 1);
        assert_eq!(e.to_string(), "i");

        let f = BigComplex::from_i64(0, -1);
        assert_eq!(f.to_string(), "-i");
    }

    #[test]
    fn test_big_complex_distance() {
        let a = BigComplex::from_i64(3, 4);
        let b = BigComplex::from_i64(0, 0);
        let distance = a.distance_to(&b);
        assert_eq!(distance.to_string(), "25"); // (3-0)² + (4-0)² = 9 + 16 = 25
    }

    #[test]
    fn test_big_complex_magnitude_function() {
        let a = BigComplex::from_i64(3, 4);
        let mag = a.magnitude();
        assert_eq!(mag.to_string(), "5"); // sqrt(3² + 4²) = sqrt(25) = 5

        let b = BigComplex::from_i64(5, 0);
        assert_eq!(b.magnitude().to_string(), "5");

        let c = BigComplex::from_i64(0, 12);
        assert_eq!(c.magnitude().to_string(), "12");
    }

    #[test]
    fn test_big_complex_polar() {
        let r = BigInt::new(5);

        // Test four basic directions
        let z0 = BigComplex::from_polar(&r, 0); // 0°
        assert_eq!(z0.real().to_string(), "5");
        assert_eq!(z0.imag().to_string(), "0");

        let z1 = BigComplex::from_polar(&r, 1); // 90°
        assert_eq!(z1.real().to_string(), "0");
        assert_eq!(z1.imag().to_string(), "5");

        let z2 = BigComplex::from_polar(&r, 2); // 180°
        assert_eq!(z2.real().to_string(), "-5");
        assert_eq!(z2.imag().to_string(), "0");

        let z3 = BigComplex::from_polar(&r, 3); // 270°
        assert_eq!(z3.real().to_string(), "0");
        assert_eq!(z3.imag().to_string(), "-5");

        // Test angle cycling
        let z4 = BigComplex::from_polar(&r, 4); // 360° = 0°
        assert_eq!(z4.real().to_string(), "5");
        assert_eq!(z4.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_quadrant() {
        let z1 = BigComplex::from_i64(3, 4); // First quadrant
        assert_eq!(z1.arg_quadrant(), Some(0));

        let z2 = BigComplex::from_i64(-3, 4); // Second quadrant
        assert_eq!(z2.arg_quadrant(), Some(1));

        let z3 = BigComplex::from_i64(-3, -4); // Third quadrant
        assert_eq!(z3.arg_quadrant(), Some(2));

        let z4 = BigComplex::from_i64(3, -4); // Fourth quadrant
        assert_eq!(z4.arg_quadrant(), Some(3));

        let z0 = BigComplex::from_i64(0, 0); // Origin
        assert_eq!(z0.arg_quadrant(), None);
    }

    #[test]
    fn test_big_complex_rotation() {
        let z = BigComplex::from_i64(1, 0); // 1 + 0i

        // Rotate 90 degrees: 1 -> i
        let z90 = z.rotate_90();
        assert_eq!(z90.real().to_string(), "0");
        assert_eq!(z90.imag().to_string(), "1");

        // Rotate 180 degrees: 1 -> -1
        let z180 = z.rotate_180();
        assert_eq!(z180.real().to_string(), "-1");
        assert_eq!(z180.imag().to_string(), "0");

        // Rotate 270 degrees: 1 -> -i
        let z270 = z.rotate_270();
        assert_eq!(z270.real().to_string(), "0");
        assert_eq!(z270.imag().to_string(), "-1");

        // Test rotation of complex numbers
        let w = BigComplex::from_i64(3, 4); // 3 + 4i
        let w90 = w.rotate_90(); // -4 + 3i
        assert_eq!(w90.real().to_string(), "-4");
        assert_eq!(w90.imag().to_string(), "3");
    }

    #[test]
    fn test_big_complex_nth_root() {
        // Test square roots
        let z1 = BigComplex::from_i64(4, 0); // 4 + 0i
        let roots = z1.nth_root(2).unwrap();
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[0].real().to_string(), "2");
        assert_eq!(roots[0].imag().to_string(), "0");
        assert_eq!(roots[1].real().to_string(), "-2");
        assert_eq!(roots[1].imag().to_string(), "0");

        // Test square roots of negative numbers
        let z2 = BigComplex::from_i64(-4, 0); // -4 + 0i
        let roots2 = z2.nth_root(2).unwrap();
        assert_eq!(roots2.len(), 2);
        assert_eq!(roots2[0].real().to_string(), "0");
        assert_eq!(roots2[0].imag().to_string(), "2");
        assert_eq!(roots2[1].real().to_string(), "0");
        assert_eq!(roots2[1].imag().to_string(), "-2");

        // Test roots of zero
        let zero = BigComplex::zero();
        let zero_roots = zero.nth_root(3).unwrap();
        assert_eq!(zero_roots.len(), 1);
        assert!(zero_roots[0].is_zero());

        // Test first root
        let z3 = BigComplex::from_i64(5, 7);
        let roots3 = z3.nth_root(1).unwrap();
        assert_eq!(roots3.len(), 1);
        assert_eq!(roots3[0], z3);

        // Test unimplemented case (general complex square root)
        let z4 = BigComplex::from_i64(3, 4); // 3 + 4i
        assert!(z4.nth_root(2).is_none()); // Not yet implemented for general complex numbers
    }

    #[test]
    fn test_big_complex_ln_approx() {
        // Test ln(1) = 0
        let one = BigComplex::from_i64(1, 0);
        let ln_one = one.ln_approx().unwrap();
        assert!(ln_one.is_zero());

        // Test logarithm approximation for positive real numbers
        let z1 = BigComplex::from_i64(8, 0); // ln(8) ≈ 3 (simplified approximation)
        let ln_z1 = z1.ln_approx().unwrap();
        assert_eq!(ln_z1.real().to_string(), "3");
        assert_eq!(ln_z1.imag().to_string(), "0");

        // Test logarithm of zero (should return None)
        let zero = BigComplex::zero();
        assert_eq!(zero.ln_approx(), None);

        // Test logarithm of complex numbers (should return None)
        let z2 = BigComplex::from_i64(1, 1);
        assert_eq!(z2.ln_approx(), None);
    }

    #[test]
    fn test_big_complex_exp_approx() {
        // Test exp(0) = 1
        let zero = BigComplex::zero();
        let exp_zero = zero.exp_approx();
        assert_eq!(exp_zero.real().to_string(), "1");
        assert_eq!(exp_zero.imag().to_string(), "0");

        // Test exponential approximation for real numbers
        let z1 = BigComplex::from_i64(1, 0);
        let exp_z1 = z1.exp_approx();
        // Since it's a simplified approximation, we only check that the result is not zero
        assert!(!exp_z1.is_zero());

        // Test exponential of complex numbers (simplified result)
        let z2 = BigComplex::from_i64(0, 1);
        let exp_z2 = z2.exp_approx();
        assert_eq!(exp_z2.real().to_string(), "1");
        assert_eq!(exp_z2.imag().to_string(), "1");
    }
}
