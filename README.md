# Gauss Int

A Rust library for Gaussian integer arithmetic and number theory.

## What is a Gaussian Integer?

A Gaussian integer is a complex number `a + bi` where both `a` and `b` are integers. Gaussian integers form a Euclidean domain, supporting division with remainder and greatest common divisors via the Euclidean algorithm.

## Features

### Gaussian Integer (GaussInt)

- Arithmetic: addition, subtraction, multiplication, negation
- **Division with remainder** — correct Gaussian integer division using nearest-integer rounding, guaranteeing `N(remainder) < N(divisor)`
- **GCD** — Euclidean algorithm with canonicalization to the first quadrant
- Exponentiation by squaring (`pow_u32`)
- Norm, conjugate, unit detection

### Number Theory

- **Prime testing** — Baillie-PSW approach (deterministic for 64-bit, multiple Miller-Rabin bases for larger numbers)
- **Factorization** — trial division by small primes + Pollard's Rho
- **Euler's totient** φ(n) via prime factorization
- **Jacobi symbol** (a/n) via quadratic reciprocity
- **Chinese Remainder Theorem** — solve x ≡ a_i (mod m_i) for pairwise coprime moduli
- **Gaussian prime detection** — full classification in ℤ[i]

### CLI

A command-line tool exposing all functionality:

```bash
# Gaussian integer operations
cargo run -- add 3+4i 1+2i
cargo run -- mul 3+4i 1+2i
cargo run -- div 7+5i 1+2i
cargo run -- gcd 12+18i 6+8i
cargo run -- norm 3+4i
cargo run -- conj 3+4i

# Number theory
cargo run -- is-prime 97
cargo run -- factor 123456
cargo run -- totient 100
cargo run -- jacobi 2 7
cargo run -- crt 2 3 3 5
```

## Library Usage

```rust
use gauss_int::{GaussInt, BigInt};
use gauss_int::number_theory;

// Gaussian integer arithmetic
let z = GaussInt::from_i64(3, 4);
let conj = z.conjugate();
let (q, r) = z.div_rem(&GaussInt::from_i64(1, 2)).unwrap();

// GCD
let g = GaussInt::from_i64(12, 18).gcd(&GaussInt::from_i64(6, 8));

// Primality testing
assert!(number_theory::is_prime(&BigInt::new(97)));

// Factorization
let factors = number_theory::factorize(&BigInt::new(123456));

// Euler's totient
assert_eq!(number_theory::euler_totient(&BigInt::new(100)), BigInt::new(40));

// Chinese Remainder Theorem
let congruences = vec![
    (BigInt::new(2), BigInt::new(3)),
    (BigInt::new(3), BigInt::new(5)),
];
let x = number_theory::crt(&congruences).unwrap();

// Gaussian prime detection
assert!(number_theory::is_gaussian_prime(&GaussInt::from_i64(3, 0)));
assert!(!number_theory::is_gaussian_prime(&GaussInt::from_i64(5, 0)));
```

## Testing

```bash
cargo test
```

## Project Structure

```
src/
├── lib.rs              # Module exports
├── big_int.rs          # BigInt wrapper around num-bigint
├── gauss_int.rs        # Gaussian integer implementation
├── number_theory.rs    # Primality, factorization, totient, Jacobi, CRT
└── main.rs             # CLI binary

tests/
└── integration_tests.rs
```

## Dependencies

- `num-bigint` — arbitrary precision integers
- `num-traits` — numerical traits (Zero, One, Signed)
- `num-integer` — integer operations (gcd, is_even)
- `clap` — CLI argument parsing
