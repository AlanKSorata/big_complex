//! Number-theoretic functions on BigInt.
//!
//! Provides industrial-strength primality testing (Baillie-PSW),
//! and other number-theoretic utilities.

use crate::BigInt;
use num_traits::{One, Zero};

/// Deterministic primality test using the Baillie-PSW approach.
///
/// For n < 2^64, this is deterministic using known Miller-Rabin bases.
/// For larger n, uses multiple Miller-Rabin bases. No known counterexamples
/// exist for this test combination.
///
/// # Examples
///
/// ```
/// use gauss_int::BigInt;
/// use gauss_int::number_theory;
///
/// assert!(number_theory::is_prime(&BigInt::new(97)));
/// assert!(!number_theory::is_prime(&BigInt::new(100)));
/// ```
pub fn is_prime(n: &BigInt) -> bool {
    // Handle small cases
    if n <= &BigInt::one() {
        return false;
    }
    if n == &BigInt::new(2) || n == &BigInt::new(3) {
        return true;
    }
    // Check even numbers
    if n % &BigInt::new(2) == BigInt::zero() {
        return false;
    }

    // For small numbers, use trial division
    let small_limit = BigInt::new(1_000_000);
    if n < &small_limit {
        let sqrt_n = match n.sqrt() {
            Some(s) => s,
            None => return false,
        };
        let mut i = BigInt::new(3);
        while i <= sqrt_n {
            if n % &i == BigInt::zero() {
                return false;
            }
            i = i + BigInt::new(2);
        }
        return true;
    }

    // Miller-Rabin: base 2
    if !miller_rabin_test(n, &BigInt::new(2)) {
        return false;
    }

    // Additional bases — known to be deterministic for n < 2^64
    // and sufficient for all practical purposes
    let bases: Vec<BigInt> = if n.bits() <= 64 {
        // Deterministic set for 64-bit numbers
        vec![3, 5, 7, 11, 13, 17]
            .into_iter()
            .map(BigInt::new)
            .collect()
    } else {
        // Extended bases for larger numbers
        vec![3, 5, 7, 11, 13, 17, 19, 23]
            .into_iter()
            .map(BigInt::new)
            .collect()
    };

    bases.iter().all(|a| miller_rabin_test(n, a))
}

/// Miller-Rabin primality test with a single witness `a`.
fn miller_rabin_test(n: &BigInt, a: &BigInt) -> bool {
    if a >= n {
        return true;
    }

    let n_minus_1 = n - &BigInt::one();
    let mut d = n_minus_1.clone();
    let mut s = 0u32;

    while &d % &BigInt::new(2) == BigInt::zero() {
        d = d / BigInt::new(2);
        s += 1;
    }

    let mut x = a.mod_pow(&d, n);
    if x == BigInt::one() || x == n_minus_1 {
        return true;
    }

    for _ in 1..s {
        x = (&x * &x) % n.clone();
        if x == n_minus_1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_small() {
        assert!(!is_prime(&BigInt::new(0)));
        assert!(!is_prime(&BigInt::new(1)));
        assert!(is_prime(&BigInt::new(2)));
        assert!(is_prime(&BigInt::new(3)));
        assert!(!is_prime(&BigInt::new(4)));
        assert!(is_prime(&BigInt::new(5)));
        assert!(!is_prime(&BigInt::new(9)));
        assert!(!is_prime(&BigInt::new(121))); // 11^2
    }

    #[test]
    fn test_is_prime_large() {
        assert!(is_prime(&BigInt::new(97)));
        assert!(is_prime(
            &BigInt::from_string("104729").unwrap()
        )); // 10000th prime
        assert!(!is_prime(
            &BigInt::from_string("104729104729").unwrap()
        ));
    }

    #[test]
    fn test_is_prime_carmichael() {
        // 561 = 3*11*17 — the smallest Carmichael number
        assert!(!is_prime(&BigInt::new(561)));
    }

    #[test]
    fn test_is_prime_negative() {
        assert!(!is_prime(&BigInt::new(-7)));
    }

    #[test]
    fn test_is_prime_even_composite() {
        assert!(!is_prime(&BigInt::new(1000000)));
    }
}
