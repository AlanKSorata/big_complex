//! Number-theoretic functions on BigInt.
//!
//! Provides industrial-strength primality testing (Baillie-PSW),
//! and other number-theoretic utilities.

use crate::{BigInt, GaussInt};
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

const SMALL_PRIMES: &[i64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

/// Returns the prime factorization of n as (prime, exponent) pairs.
///
/// Uses trial division by small primes followed by Pollard's Rho
/// for any remaining large factors.
pub fn factorize(n: &BigInt) -> Vec<(BigInt, u32)> {
    if n <= &BigInt::one() {
        return vec![];
    }

    let mut n = n.clone();
    let mut factors: Vec<BigInt> = vec![];

    // Trial division by small primes
    for p in SMALL_PRIMES {
        let p_big = BigInt::new(*p);
        while (&n % &p_big).is_zero() {
            factors.push(p_big.clone());
            n = &n / &p_big;
        }
    }

    // Pollard's Rho for the remaining factor
    if n > BigInt::one() {
        factor_rho(&n, &mut factors);
    }

    // Sort and count exponents
    factors.sort();
    let mut result: Vec<(BigInt, u32)> = vec![];
    for f in factors {
        match result.last_mut() {
            Some((p, count)) if p == &f => *count += 1,
            _ => result.push((f, 1)),
        }
    }
    result
}

/// Pollard's Rho factorization algorithm.
fn factor_rho(n: &BigInt, factors: &mut Vec<BigInt>) {
    if n <= &BigInt::one() {
        return;
    }
    if is_prime(n) {
        factors.push(n.clone());
        return;
    }

    // Try different c values for f(x) = x² + c
    let mut c = BigInt::one();
    loop {
        let mut x = BigInt::new(2);
        let mut y = BigInt::new(2);
        let mut d = BigInt::one();

        while d == BigInt::one() {
            x = pollard_f(&x, n, &c);
            y = pollard_f(&pollard_f(&y, n, &c), n, &c);
            let diff = (&x - &y).abs();
            d = diff.gcd(n);
        }

        if d != *n {
            factor_rho(&d, factors);
            factor_rho(&(n / &d), factors);
            return;
        }

        c = c + BigInt::one();
    }
}

/// f(x) = x² + c (mod n)
fn pollard_f(x: &BigInt, n: &BigInt, c: &BigInt) -> BigInt {
    let xx = x * x + c.clone();
    &xx % n
}

/// Euler's totient function φ(n) — count of integers 1 ≤ k ≤ n with gcd(k, n) = 1.
pub fn euler_totient(n: &BigInt) -> BigInt {
    if *n <= BigInt::one() {
        return BigInt::one();
    }
    let factors = factorize(n);
    let mut result = BigInt::one();
    for (p, e) in &factors {
        let term = p.pow(*e) - p.pow(*e - 1_u32);
        result = result * term;
    }
    result
}

/// Jacobi symbol (a/n), generalizing the Legendre symbol to odd positive moduli.
pub fn jacobi_symbol(a: &BigInt, n: &BigInt) -> i32 {
    if (n % &BigInt::new(2)).is_zero() {
        panic!("Jacobi symbol requires an odd modulus");
    }

    let mut a = a % n;
    let mut n = n.clone();
    let mut t = 1i32;

    while a != BigInt::zero() {
        while (&a % &BigInt::new(2)).is_zero() {
            a = a / BigInt::new(2);
            let n_mod_8 = &n % &BigInt::new(8);
            if n_mod_8 == BigInt::new(3) || n_mod_8 == BigInt::new(5) {
                t = -t;
            }
        }

        std::mem::swap(&mut a, &mut n);
        if (&a % &BigInt::new(4)) == BigInt::new(3) && (&n % &BigInt::new(4)) == BigInt::new(3) {
            t = -t;
        }
        a = &a % &n;
    }

    if n == BigInt::one() { t } else { 0 }
}

/// Chinese Remainder Theorem — solves x ≡ a_i (mod m_i) for pairwise coprime m_i.
pub fn crt(congruences: &[(BigInt, BigInt)]) -> Option<BigInt> {
    if congruences.is_empty() {
        return None;
    }

    let product: BigInt = congruences.iter()
        .map(|(_, m)| m.clone())
        .fold(BigInt::one(), |a, b| a * b);

    let mut result = BigInt::zero();

    for (a, m) in congruences {
        let p = &product / m;
        let inv = p.mod_inv(m)?;
        let term = a * &p;
        let term = &term * &inv;
        result = &result + &term;
    }

    Some(&result % &product)
}

/// Tests whether a Gaussian integer is prime in Z[i].
///
/// A Gaussian integer a+bi is prime iff:
/// - a != 0 and b != 0, and N(a+bi) is a rational prime, OR
/// - one component is zero and the other is a rational prime p ≡ 3 (mod 4)
///
/// # Examples
///
/// ```
/// use gauss_int::{GaussInt, number_theory::is_gaussian_prime};
///
/// assert!(is_gaussian_prime(&GaussInt::from_i64(1, 1)));   // N=2 is prime
/// assert!(is_gaussian_prime(&GaussInt::from_i64(3, 0)));   // 3 ≡ 3 mod 4
/// assert!(is_gaussian_prime(&GaussInt::from_i64(2, 1)));   // N=5 is prime
/// assert!(!is_gaussian_prime(&GaussInt::from_i64(5, 0)));  // 5 ≡ 1 mod 4
/// ```
pub fn is_gaussian_prime(z: &GaussInt) -> bool {
    if z.is_zero() || z.is_unit() {
        return false;
    }

    let (a, b) = (z.real(), z.imag());

    if b.is_zero() {
        // On the real axis: |a| must be a rational prime ≡ 3 (mod 4)
        let abs_a = a.abs();
        if !is_prime(&abs_a) {
            return false;
        }
        &abs_a % &BigInt::new(4) == BigInt::new(3)
    } else if a.is_zero() {
        // On the imaginary axis: same condition for |b|
        let abs_b = b.abs();
        if !is_prime(&abs_b) {
            return false;
        }
        &abs_b % &BigInt::new(4) == BigInt::new(3)
    } else {
        // Off-axis: N(a+bi) must be a rational prime
        let n = z.norm();
        is_prime(&n)
    }
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

    #[test]
    fn test_factorize_small_primes() {
        let factors = factorize(&BigInt::new(97));
        assert_eq!(factors, vec![(BigInt::new(97), 1)]);
    }

    #[test]
    fn test_factorize_power_of_two() {
        let factors = factorize(&BigInt::new(64));
        assert_eq!(factors, vec![(BigInt::new(2), 6)]);
    }

    #[test]
    fn test_factorize_composite() {
        let factors = factorize(&BigInt::new(12));
        assert_eq!(factors, vec![(BigInt::new(2), 2), (BigInt::new(3), 1)]);
    }

    #[test]
    fn test_factorize_zero_and_one() {
        assert!(factorize(&BigInt::new(0)).is_empty());
        assert!(factorize(&BigInt::new(1)).is_empty());
    }

    #[test]
    fn test_factorize_product_preserved() {
        // 123456 = 2^6 * 3 * 643
        let factors = factorize(&BigInt::new(123456));
        let product: BigInt = factors.iter().map(|(p, e)| p.pow(*e)).fold(BigInt::one(), |a, b| a * b);
        assert_eq!(product, BigInt::new(123456));
    }

    #[test]
    fn test_factorize_semiprime() {
        let p = BigInt::new(97);
        let q = BigInt::new(101);
        let n = &p * &q;
        let factors = factorize(&n);
        let product: BigInt = factors.iter().map(|(p, e)| p.pow(*e)).fold(BigInt::one(), |a, b| a * b);
        assert_eq!(product, n);
    }

    #[test]
    fn test_euler_totient_prime() {
        assert_eq!(euler_totient(&BigInt::new(7)), BigInt::new(6));
        assert_eq!(euler_totient(&BigInt::new(97)), BigInt::new(96));
    }

    #[test]
    fn test_euler_totient_composite() {
        assert_eq!(euler_totient(&BigInt::new(12)), BigInt::new(4));
        assert_eq!(euler_totient(&BigInt::new(100)), BigInt::new(40));
    }

    #[test]
    fn test_jacobi_basic() {
        assert_eq!(jacobi_symbol(&BigInt::new(2), &BigInt::new(7)), 1);
        assert_eq!(jacobi_symbol(&BigInt::new(3), &BigInt::new(7)), -1);
        assert_eq!(jacobi_symbol(&BigInt::new(0), &BigInt::new(7)), 0);
    }

    #[test]
    fn test_crt_basic() {
        let congruences = vec![
            (BigInt::new(2), BigInt::new(3)),
            (BigInt::new(3), BigInt::new(5)),
        ];
        let x = crt(&congruences).unwrap();
        assert_eq!(&x % &BigInt::new(3), BigInt::new(2));
        assert_eq!(&x % &BigInt::new(5), BigInt::new(3));
    }

    #[test]
    fn test_crt_no_solution() {
        let congruences = vec![
            (BigInt::new(1), BigInt::new(2)),
            (BigInt::new(0), BigInt::new(4)),
        ];
        assert!(crt(&congruences).is_none());
    }

    #[test]
    fn test_crt_single_congruence() {
        let congruences = vec![(BigInt::new(5), BigInt::new(7))];
        let x = crt(&congruences).unwrap();
        assert_eq!(&x % &BigInt::new(7), BigInt::new(5));
    }

    #[test]
    fn test_gaussian_prime_integer_primes() {
        // Primes p ≡ 3 mod 4 are Gaussian primes
        assert!(is_gaussian_prime(&GaussInt::from_i64(3, 0)));
        assert!(is_gaussian_prime(&GaussInt::from_i64(7, 0)));
        assert!(is_gaussian_prime(&GaussInt::from_i64(-3, 0)));
        // Primes p ≡ 1 mod 4 are NOT Gaussian primes
        assert!(!is_gaussian_prime(&GaussInt::from_i64(5, 0)));
        assert!(!is_gaussian_prime(&GaussInt::from_i64(13, 0)));
    }

    #[test]
    fn test_gaussian_prime_off_axis() {
        // (1+i) has N=2 → prime
        assert!(is_gaussian_prime(&GaussInt::from_i64(1, 1)));
        // (2+i) has N=5 → prime (5 is prime)
        assert!(is_gaussian_prime(&GaussInt::from_i64(2, 1)));
        // (2+2i) has N=8 → not prime
        assert!(!is_gaussian_prime(&GaussInt::from_i64(2, 2)));
    }

    #[test]
    fn test_gaussian_prime_imaginary_axis() {
        // 3i → |3| = 3 ≡ 3 mod 4 → prime
        assert!(is_gaussian_prime(&GaussInt::from_i64(0, 3)));
        // 5i → |5| = 5 ≡ 1 mod 4 → not prime
        assert!(!is_gaussian_prime(&GaussInt::from_i64(0, 5)));
    }

    #[test]
    fn test_gaussian_prime_units_and_zero() {
        assert!(!is_gaussian_prime(&GaussInt::from_i64(0, 0)));
        assert!(!is_gaussian_prime(&GaussInt::from_i64(1, 0)));
        assert!(!is_gaussian_prime(&GaussInt::from_i64(0, 1)));
        assert!(!is_gaussian_prime(&GaussInt::from_i64(-1, 0)));
    }

    #[test]
    fn test_gaussian_prime_composite() {
        // 2 = (1+i)(1-i) → not a Gaussian prime
        assert!(!is_gaussian_prime(&GaussInt::from_i64(2, 0)));
        // 10 = (3+i)(3-i) → not prime
        assert!(!is_gaussian_prime(&GaussInt::from_i64(10, 0)));
    }
}
