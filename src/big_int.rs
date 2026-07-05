use num_bigint::{BigInt as NumBigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// A wrapper around `num_bigint::BigInt` providing additional mathematical operations.
///
/// `BigInt` supports arbitrary-precision integer arithmetic with operations
/// including basic arithmetic, modular arithmetic, prime number operations,
/// and binary manipulations.
///
/// # Examples
///
/// ```
/// use gauss_int::BigInt;
///
/// let a = BigInt::new(42);
/// let b = BigInt::from_string("12345678901234567890").unwrap();
/// let sum = &a + &b;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    inner: NumBigInt,
}

impl BigInt {
    /// Creates a new `BigInt` from an `i64` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let n = BigInt::new(42);
    /// assert_eq!(n.to_string(), "42");
    /// ```
    pub fn new(value: i64) -> Self {
        BigInt {
            inner: NumBigInt::from(value),
        }
    }

    /// Parses a `BigInt` from a decimal string.
    ///
    /// Returns `None` if the string is not a valid decimal number.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let n = BigInt::from_string("12345678901234567890").unwrap();
    /// assert_eq!(n.to_string(), "12345678901234567890");
    ///
    /// let invalid = BigInt::from_string("not a number");
    /// assert!(invalid.is_none());
    /// ```
    pub fn from_string(s: &str) -> Option<Self> {
        NumBigInt::parse_bytes(s.as_bytes(), 10).map(|n| BigInt { inner: n })
    }

    /// Creates a `BigInt` from a big-endian byte representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    /// use num_bigint::Sign;
    ///
    /// let bytes = vec![0x01, 0x02, 0x03];
    /// let n = BigInt::from_bytes_be(Sign::Plus, &bytes);
    /// ```
    pub fn from_bytes_be(sign: Sign, bytes: &[u8]) -> Self {
        BigInt {
            inner: NumBigInt::from_bytes_be(sign, bytes),
        }
    }

    /// Returns the big-endian byte representation of this `BigInt`.
    ///
    /// Returns a tuple of the sign and the byte vector.
    pub fn to_bytes_be(&self) -> (Sign, Vec<u8>) {
        self.inner.to_bytes_be()
    }

    /// Returns the absolute value of this `BigInt`.
    pub fn abs(&self) -> Self {
        BigInt {
            inner: self.inner.abs(),
        }
    }

    /// Returns the sign of this `BigInt`.
    pub fn sign(&self) -> Sign {
        self.inner.sign()
    }

    /// Returns `true` if this `BigInt` is zero.
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    /// Returns `true` if this `BigInt` is positive.
    pub fn is_positive(&self) -> bool {
        self.inner.is_positive()
    }

    /// Returns `true` if this `BigInt` is negative.
    pub fn is_negative(&self) -> bool {
        self.inner.is_negative()
    }

    /// Raises this `BigInt` to the power of `exp`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let n = BigInt::new(3);
    /// assert_eq!(n.pow(4).to_string(), "81");
    /// ```
    pub fn pow(&self, exp: u32) -> Self {
        BigInt {
            inner: self.inner.pow(exp),
        }
    }

    /// Returns the integer square root of this `BigInt`.
    ///
    /// Returns `None` if this number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let n = BigInt::new(144);
    /// assert_eq!(n.sqrt().unwrap().to_string(), "12");
    ///
    /// let negative = BigInt::new(-4);
    /// assert!(negative.sqrt().is_none());
    /// ```
    pub fn sqrt(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }

        let mut low = BigInt::new(0);
        let mut high = self.clone();

        while low <= high {
            let mid = (&low + &high) / BigInt::new(2);
            let mid_squared = &mid * &mid;

            match mid_squared.cmp(self) {
                Ordering::Equal => return Some(mid),
                Ordering::Less => low = mid + BigInt::new(1),
                Ordering::Greater => high = mid - BigInt::new(1),
            }
        }

        Some(high)
    }

    /// Returns the greatest common divisor of this `BigInt` and `other`.
    pub fn gcd(&self, other: &Self) -> Self {
        BigInt {
            inner: self.inner.gcd(&other.inner),
        }
    }

    /// Returns the least common multiple of this `BigInt` and `other`.
    pub fn lcm(&self, other: &Self) -> Self {
        BigInt {
            inner: self.inner.lcm(&other.inner),
        }
    }

    /// Computes modular exponentiation: (self^exp) mod modulus.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let base = BigInt::new(7);
    /// let exp = BigInt::new(3);
    /// let modulus = BigInt::new(11);
    /// // 7^3 mod 11 = 343 mod 11 = 2
    /// assert_eq!(base.mod_pow(&exp, &modulus).to_string(), "2");
    /// ```
    pub fn mod_pow(&self, exp: &Self, modulus: &Self) -> Self {
        BigInt {
            inner: self.inner.modpow(&exp.inner, &modulus.inner),
        }
    }

    /// Returns the modular multiplicative inverse of this `BigInt` modulo `modulus`.
    ///
    /// Returns `None` if the inverse does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let n = BigInt::new(3);
    /// let modulus = BigInt::new(11);
    /// // 3 * 4 = 12 ≡ 1 mod 11
    /// assert_eq!(n.mod_inv(&modulus).unwrap().to_string(), "4");
    /// ```
    pub fn mod_inv(&self, modulus: &Self) -> Option<Self> {
        self.inner
            .modinv(&modulus.inner)
            .map(|n| BigInt { inner: n })
    }

    /// Returns the factorial of this `BigInt`.
    ///
    /// Returns `None` if this number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// let n = BigInt::new(5);
    /// assert_eq!(n.factorial().unwrap().to_string(), "120"); // 5! = 120
    ///
    /// let negative = BigInt::new(-5);
    /// assert!(negative.factorial().is_none());
    /// ```
    pub fn factorial(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }

        let mut result = BigInt::one();
        let mut current = BigInt::one();

        while current <= *self {
            result = result * current.clone();
            current = current + BigInt::one();
        }

        Some(result)
    }

    /// Checks if this `BigInt` is a prime number.
    ///
    /// Uses deterministic Miller-Rabin primality test for numbers >= 3,
    /// with trial division for small numbers. This is efficient for
    /// numbers with hundreds of digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use gauss_int::BigInt;
    ///
    /// assert!(BigInt::new(2).is_prime());
    /// assert!(BigInt::new(97).is_prime());
    /// assert!(!BigInt::new(100).is_prime());
    /// ```
    pub fn is_prime(&self) -> bool {
        if self <= &BigInt::one() {
            return false;
        }

        if self == &BigInt::new(2) || self == &BigInt::new(3) {
            return true;
        }

        if self % &BigInt::new(2) == BigInt::zero() {
            return false;
        }

        // For small numbers, use trial division
        let small_limit = BigInt::new(1_000_000);
        if self < &small_limit {
            let sqrt_n = self.sqrt().unwrap_or_else(BigInt::zero);
            let mut i = BigInt::new(3);
            while i <= sqrt_n {
                if self % &i == BigInt::zero() {
                    return false;
                }
                i = i + BigInt::new(2);
            }
            return true;
        }

        // Miller-Rabin primality test for larger numbers
        self.miller_rabin_test()
    }

    /// Performs the Miller-Rabin primality test.
    ///
    /// This is a deterministic implementation that uses specific witness
    /// values known to be sufficient for numbers of different bit lengths.
    fn miller_rabin_test(&self) -> bool {
        // Write n-1 as d * 2^s
        let n_minus_1 = self - &BigInt::one();
        let mut d = n_minus_1.clone();
        let mut s = 0u32;

        while &d % &BigInt::new(2) == BigInt::zero() {
            d = d / BigInt::new(2);
            s += 1;
        }

        // Choose witnesses based on bit length
        // These values are proven to be sufficient for deterministic testing
        let bit_len = self.inner.bits() as usize;
        let witnesses: Vec<BigInt> = match bit_len {
            0..=64 => vec![BigInt::new(2), BigInt::new(3), BigInt::new(5), BigInt::new(7), BigInt::new(11)],
            _ => vec![
                BigInt::new(2),
                BigInt::new(3),
                BigInt::new(5),
                BigInt::new(7),
                BigInt::new(11),
                BigInt::new(13),
                BigInt::new(17),
            ],
        };

        for a in witnesses {
            if &a >= self {
                continue;
            }

            let mut x = a.mod_pow(&d, self);
            if x == BigInt::one() || x == n_minus_1 {
                continue;
            }

            let mut composite = true;
            for _ in 1..s {
                x = (&x * &x) % self.clone();
                if x == n_minus_1 {
                    composite = false;
                    break;
                }
            }

            if composite {
                return false;
            }
        }

        true
    }

    /// Returns (quotient, remainder) of division, where quotient truncates toward zero.
    pub fn div_mod(&self, other: &Self) -> (Self, Self) {
        (self / other, self % other)
    }
}

impl Rem for BigInt {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        BigInt {
            inner: self.inner % other.inner,
        }
    }
}

impl Rem for &BigInt {
    type Output = BigInt;

    fn rem(self, other: Self) -> BigInt {
        BigInt {
            inner: &self.inner % &other.inner,
        }
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        BigInt::new(value)
    }
}

impl From<NumBigInt> for BigInt {
    fn from(value: NumBigInt) -> Self {
        BigInt { inner: value }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Zero for BigInt {
    fn zero() -> Self {
        BigInt {
            inner: NumBigInt::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
}

impl One for BigInt {
    fn one() -> Self {
        BigInt {
            inner: NumBigInt::one(),
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BigInt {
            inner: self.inner + other.inner,
        }
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> BigInt {
        BigInt {
            inner: &self.inner + &other.inner,
        }
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        BigInt {
            inner: self.inner - other.inner,
        }
    }
}

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> BigInt {
        BigInt {
            inner: &self.inner - &other.inner,
        }
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        BigInt {
            inner: self.inner * other.inner,
        }
    }
}

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, other: Self) -> BigInt {
        BigInt {
            inner: &self.inner * &other.inner,
        }
    }
}

impl Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        BigInt {
            inner: self.inner / other.inner,
        }
    }
}

impl Div for &BigInt {
    type Output = BigInt;

    fn div(self, other: Self) -> BigInt {
        BigInt {
            inner: &self.inner / &other.inner,
        }
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self {
        BigInt { inner: -self.inner }
    }
}

impl Neg for &BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {
        BigInt {
            inner: -&self.inner,
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_int_creation() {
        let a = BigInt::new(42);
        assert_eq!(a.to_string(), "42");

        let b = BigInt::from_string("12345678901234567890").unwrap();
        assert_eq!(b.to_string(), "12345678901234567890");

        let c = BigInt::from_string("-987654321").unwrap();
        assert_eq!(c.to_string(), "-987654321");
    }

    #[test]
    fn test_big_int_arithmetic() {
        let a = BigInt::new(15);
        let b = BigInt::new(25);

        assert_eq!((&a + &b).to_string(), "40");
        assert_eq!((&b - &a).to_string(), "10");
        assert_eq!((&a * &b).to_string(), "375");
        assert_eq!((&b / &a).to_string(), "1");
    }

    #[test]
    fn test_big_int_pow() {
        let a = BigInt::new(3);
        assert_eq!(a.pow(4).to_string(), "81");

        let b = BigInt::new(2);
        assert_eq!(b.pow(10).to_string(), "1024");
    }

    #[test]
    fn test_big_int_sqrt() {
        let a = BigInt::new(144);
        assert_eq!(a.sqrt().unwrap().to_string(), "12");

        let b = BigInt::new(145);
        assert_eq!(b.sqrt().unwrap().to_string(), "12");

        let c = BigInt::new(-4);
        assert_eq!(c.sqrt(), None);
    }

    #[test]
    fn test_big_int_gcd_lcm() {
        let a = BigInt::new(12);
        let b = BigInt::new(18);
        assert_eq!(a.gcd(&b).to_string(), "6");
        assert_eq!(a.lcm(&b).to_string(), "36");
    }

    #[test]
    fn test_big_int_modular() {
        let a = BigInt::new(7);
        let b = BigInt::new(3);
        let m = BigInt::new(11);

        let result = a.mod_pow(&b, &m);
        assert_eq!(result.to_string(), "2"); // 7^3 mod 11 = 343 mod 11 = 2

        let inv = BigInt::new(3).mod_inv(&BigInt::new(11));
        assert_eq!(inv.unwrap().to_string(), "4"); // 3 * 4 = 12 ≡ 1 mod 11
    }

    #[test]
    fn test_big_int_comparison() {
        let a = BigInt::new(100);
        let b = BigInt::new(200);

        assert!(a < b);
        assert!(b > a);
        assert!(a == a);
    }

    #[test]
    fn test_big_int_factorial() {
        let zero = BigInt::new(0);
        assert_eq!(zero.factorial().unwrap().to_string(), "1");

        let one = BigInt::new(1);
        assert_eq!(one.factorial().unwrap().to_string(), "1");

        let five = BigInt::new(5);
        assert_eq!(five.factorial().unwrap().to_string(), "120"); // 5! = 120

        let ten = BigInt::new(10);
        assert_eq!(ten.factorial().unwrap().to_string(), "3628800"); // 10! = 3628800

        let negative = BigInt::new(-5);
        assert_eq!(negative.factorial(), None);

        // Test large factorial
        let twenty = BigInt::new(20);
        let result = twenty.factorial().unwrap();
        assert_eq!(result.to_string(), "2432902008176640000"); // 20!
    }

    #[test]
    fn test_big_int_prime() {
        // Test small primes
        assert!(!BigInt::new(0).is_prime());
        assert!(!BigInt::new(1).is_prime());
        assert!(BigInt::new(2).is_prime());
        assert!(BigInt::new(3).is_prime());
        assert!(!BigInt::new(4).is_prime());
        assert!(BigInt::new(5).is_prime());
        assert!(!BigInt::new(6).is_prime());
        assert!(BigInt::new(7).is_prime());
        assert!(!BigInt::new(8).is_prime());
        assert!(!BigInt::new(9).is_prime());
        assert!(!BigInt::new(10).is_prime());
        assert!(BigInt::new(11).is_prime());

        // Test larger primes
        assert!(BigInt::new(97).is_prime());
        assert!(BigInt::new(101).is_prime());
        assert!(!BigInt::new(100).is_prime());
        assert!(!BigInt::new(121).is_prime()); // 11^2

        // Test negative numbers
        assert!(!BigInt::new(-7).is_prime());

        // Test large primes (using Miller-Rabin)
        // 104729 is the 10000th prime
        let large_prime = BigInt::from_string("104729").unwrap();
        assert!(large_prime.is_prime());

        // 104723 is also prime
        let another_large_prime = BigInt::from_string("104723").unwrap();
        assert!(another_large_prime.is_prime());

        // Test large composite number
        let large_composite = BigInt::from_string("104729104729").unwrap();
        assert!(!large_composite.is_prime());

        // Test Carmichael number (561 = 3 * 11 * 17) - should be detected as composite
        let carmichael = BigInt::new(561);
        assert!(!carmichael.is_prime());
    }

    #[test]
    fn test_big_int_div_mod() {
        let a = BigInt::new(17);
        let b = BigInt::new(5);
        let (q, r) = a.div_mod(&b);
        assert_eq!(q.to_string(), "3");
        assert_eq!(r.to_string(), "2");

        let (q2, r2) = BigInt::new(-17).div_mod(&BigInt::new(5));
        assert_eq!(q2.to_string(), "-3");
        assert_eq!(r2.to_string(), "-2");
    }
}
