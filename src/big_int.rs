use num_bigint::{BigInt as NumBigInt, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    inner: NumBigInt,
}

impl BigInt {
    pub fn new(value: i64) -> Self {
        BigInt {
            inner: NumBigInt::from(value),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        NumBigInt::parse_bytes(s.as_bytes(), 10).map(|n| BigInt { inner: n })
    }

    pub fn from_bytes_be(sign: Sign, bytes: &[u8]) -> Self {
        BigInt {
            inner: NumBigInt::from_bytes_be(sign, bytes),
        }
    }

    pub fn to_bytes_be(&self) -> (Sign, Vec<u8>) {
        self.inner.to_bytes_be()
    }

    pub fn abs(&self) -> Self {
        BigInt {
            inner: self.inner.abs(),
        }
    }

    pub fn sign(&self) -> Sign {
        self.inner.sign()
    }

    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.inner.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.inner.is_negative()
    }

    pub fn pow(&self, exp: u32) -> Self {
        BigInt {
            inner: self.inner.pow(exp),
        }
    }

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

    pub fn gcd(&self, other: &Self) -> Self {
        BigInt {
            inner: self.inner.gcd(&other.inner),
        }
    }

    pub fn lcm(&self, other: &Self) -> Self {
        BigInt {
            inner: self.inner.lcm(&other.inner),
        }
    }

    pub fn mod_pow(&self, exp: &Self, modulus: &Self) -> Self {
        BigInt {
            inner: self.inner.modpow(&exp.inner, &modulus.inner),
        }
    }

    pub fn mod_inv(&self, modulus: &Self) -> Option<Self> {
        self.inner
            .modinv(&modulus.inner)
            .map(|n| BigInt { inner: n })
    }

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

    pub fn is_prime(&self) -> bool {
        if self <= &BigInt::one() {
            return false;
        }

        if self == &BigInt::new(2) {
            return true;
        }

        if self % &BigInt::new(2) == BigInt::zero() {
            return false;
        }

        let sqrt_n = self.sqrt().unwrap_or_else(|| BigInt::zero());
        let mut i = BigInt::new(3);

        while i <= sqrt_n {
            if self % &i == BigInt::zero() {
                return false;
            }
            i = i + BigInt::new(2);
        }

        true
    }

    pub fn next_prime(&self) -> Self {
        let mut candidate = if self <= &BigInt::new(2) {
            BigInt::new(2)
        } else if self % &BigInt::new(2) == BigInt::zero() {
            self + &BigInt::one()
        } else {
            self + &BigInt::new(2)
        };

        while !candidate.is_prime() {
            candidate = candidate + BigInt::new(2);
        }

        candidate
    }

    pub fn bit_length(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        self.inner.bits() as usize
    }

    pub fn count_ones(&self) -> u64 {
        if self.is_negative() {
            return 0; // For negative numbers, we don't count ones
        }

        let (_, bytes) = self.to_bytes_be();
        bytes.iter().map(|b| b.count_ones() as u64).sum()
    }

    pub fn trailing_zeros(&self) -> Option<u64> {
        if self.is_zero() {
            return None;
        }

        let (_, bytes) = self.to_bytes_be();
        let mut zeros = 0u64;

        for &byte in bytes.iter().rev() {
            if byte == 0 {
                zeros += 8;
            } else {
                zeros += byte.trailing_zeros() as u64;
                break;
            }
        }

        Some(zeros)
    }

    pub fn is_power_of_two(&self) -> bool {
        if self <= &BigInt::zero() {
            return false;
        }

        self.count_ones() == 1
    }

    pub fn next_power_of_two(&self) -> Self {
        if self <= &BigInt::one() {
            return BigInt::one();
        }

        let bit_len = self.bit_length();
        if self.is_power_of_two() {
            return self.clone();
        }

        BigInt::new(2).pow(bit_len as u32)
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

impl<'a> Rem for &'a BigInt {
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

impl<'a> Add for &'a BigInt {
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

impl<'a> Sub for &'a BigInt {
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

impl<'a> Mul for &'a BigInt {
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

impl<'a> Div for &'a BigInt {
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

impl<'a> Neg for &'a BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {
        BigInt {
            inner: -&self.inner,
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
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
    }

    #[test]
    fn test_big_int_next_prime() {
        assert_eq!(BigInt::new(0).next_prime().to_string(), "2");
        assert_eq!(BigInt::new(1).next_prime().to_string(), "2");
        assert_eq!(BigInt::new(2).next_prime().to_string(), "2");
        assert_eq!(BigInt::new(3).next_prime().to_string(), "5");
        assert_eq!(BigInt::new(4).next_prime().to_string(), "5");
        assert_eq!(BigInt::new(10).next_prime().to_string(), "11");
        assert_eq!(BigInt::new(14).next_prime().to_string(), "17");
        assert_eq!(BigInt::new(97).next_prime().to_string(), "101");
    }

    #[test]
    fn test_big_int_binary_operations() {
        // Test bit length
        assert_eq!(BigInt::new(0).bit_length(), 0);
        assert_eq!(BigInt::new(1).bit_length(), 1);
        assert_eq!(BigInt::new(2).bit_length(), 2);
        assert_eq!(BigInt::new(7).bit_length(), 3); // 111 in binary
        assert_eq!(BigInt::new(8).bit_length(), 4); // 1000 in binary
        assert_eq!(BigInt::new(255).bit_length(), 8); // 11111111 in binary

        // Test count of ones
        assert_eq!(BigInt::new(0).count_ones(), 0);
        assert_eq!(BigInt::new(1).count_ones(), 1);
        assert_eq!(BigInt::new(3).count_ones(), 2); // 11 in binary
        assert_eq!(BigInt::new(7).count_ones(), 3); // 111 in binary
        assert_eq!(BigInt::new(15).count_ones(), 4); // 1111 in binary
        assert_eq!(BigInt::new(-5).count_ones(), 0); // Negative numbers return 0

        // Test trailing zeros
        assert_eq!(BigInt::new(0).trailing_zeros(), None);
        assert_eq!(BigInt::new(1).trailing_zeros(), Some(0)); // 1
        assert_eq!(BigInt::new(2).trailing_zeros(), Some(1)); // 10
        assert_eq!(BigInt::new(4).trailing_zeros(), Some(2)); // 100
        assert_eq!(BigInt::new(8).trailing_zeros(), Some(3)); // 1000
        assert_eq!(BigInt::new(12).trailing_zeros(), Some(2)); // 1100

        // Test if power of two
        assert!(!BigInt::new(0).is_power_of_two());
        assert!(BigInt::new(1).is_power_of_two());
        assert!(BigInt::new(2).is_power_of_two());
        assert!(!BigInt::new(3).is_power_of_two());
        assert!(BigInt::new(4).is_power_of_two());
        assert!(!BigInt::new(5).is_power_of_two());
        assert!(BigInt::new(8).is_power_of_two());
        assert!(BigInt::new(16).is_power_of_two());
        assert!(!BigInt::new(-4).is_power_of_two());

        // Test next power of two
        assert_eq!(BigInt::new(0).next_power_of_two().to_string(), "1");
        assert_eq!(BigInt::new(1).next_power_of_two().to_string(), "1");
        assert_eq!(BigInt::new(2).next_power_of_two().to_string(), "2");
        assert_eq!(BigInt::new(3).next_power_of_two().to_string(), "4");
        assert_eq!(BigInt::new(5).next_power_of_two().to_string(), "8");
        assert_eq!(BigInt::new(9).next_power_of_two().to_string(), "16");
    }
}
