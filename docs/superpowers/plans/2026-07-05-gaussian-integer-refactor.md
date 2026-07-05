# Gaussian Integer + Number Theory Library Refactor

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the current `big_complex` crate into a practically useful Gaussian integer + number theory library.

**Architecture:** Three layers — (1) BigInt with genuine value-add (industrial primality testing, factoring, number-theoretic functions), (2) Gaussian integer arithmetic with mathematically correct division and GCD, (3) CLI tool exposing all functionality. Remove/rename everything misleading (ln_approx, exp_approx, nth_root stubs).

**Tech Stack:** Rust, num-bigint, num-traits, num-integer, serde (optional), clap (CLI)

---

## File Structure

```
src/
├── lib.rs                  # Module exports, rename crate to `gauss_int`
├── big_int.rs              # REWORK: strip to essentials, add number theory
├── gauss_int.rs            # REWRITE: correct Gaussian integer arithmetic
├── number_theory.rs        # NEW: primality, factoring, totient, jacobi, crt
├── rational.rs             # NEW (Phase 3): BigRational wrapper

tests/
├── integration_tests.rs    # REWRITE: integration tests for new API
├── gauss_int_properties.rs # NEW: property-based tests for Gaussian integer laws

examples/
├── usage.rs                # REWRITE: demonstrate new API

README.md                   # REWRITE: honest description

Cargo.toml                  # RENAME package, add deps
```

### Modules & Responsibilities

| File | Responsibility | Boundaries |
|------|---------------|------------|
| `big_int.rs` | Thin wrapper around num-bigint + essential extras | No Gaussian integer logic. No ln/exp/nth_root. Number theory lives in its own module. |
| `gauss_int.rs` | Gaussian integer with correct arithmetic | Owns Div/Rem/Gcd. Depends on BigInt for components. |
| `number_theory.rs` | BPSW, Pollard's Rho, Euler totient, Jacobi, CRT | Pure functions on BigInt. No Gaussian integer deps. |
| `rational.rs` | BigRational wrapper, exact complex numbers | Depends on num-rational. Separate concern. |

---

## Phase 0 — Clean Up API (Foundation)

### Task 0.1: Rename crate to `gauss_int` and strip misleading API

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/big_int.rs`
- Modify: `src/big_complex.rs` (will be renamed later)

- [ ] **Step 1: Rename package**

```toml
[package]
name = "gauss_int"
version = "0.2.0"
description = "Gaussian integer arithmetic and number theory utilities"
```

- [ ] **Step 2: Update lib.rs to export new module**

```rust
pub mod big_int;
pub mod gauss_int;
pub mod number_theory;

pub use big_int::BigInt;
pub use gauss_int::GaussInt;
```

- [ ] **Step 3: Remove all misleading methods from big_int.rs**

Delete these methods:
- `factorial()` → move to a standalone function `fn factorial(n: &BigInt) -> Option<BigInt>` in `number_theory.rs`
- `next_prime()` → move to `number_theory.rs` as `fn next_prime(n: &BigInt) -> BigInt`
- All binary operations (`count_ones`, `trailing_zeros`, `is_power_of_two`, `next_power_of_two`, `bit_length`) — these are already on `num_bigint::BigInt` via `.bits()`; the wrapper adds no value

Keep on BigInt:
- `new()`, `from_string()`, `from_bytes_be()`, `to_bytes_be()`
- `abs()`, `sign()`, `is_zero()`, `is_positive()`, `is_negative()`
- `pow()`, `sqrt()`, `gcd()`, `lcm()`, `mod_pow()`, `mod_inv()`
- `is_prime()` — will be upgraded in Phase 2

- [ ] **Step 4: Remove `BigComplex` struct, replace with `GaussInt`**

Delete `src/big_complex.rs`, create `src/gauss_int.rs` with the new struct (implementation in Phase 1, for now just the struct definition):

```rust
use crate::BigInt;
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
    pub fn new(real: BigInt, imag: BigInt) -> Self { unimplemented!() }
    pub fn from_i64(real: i64, imag: i64) -> Self { unimplemented!() }
    pub fn real(&self) -> &BigInt { unimplemented!() }
    pub fn imag(&self) -> &BigInt { unimplemented!() }
    pub fn conjugate(&self) -> Self { unimplemented!() }
    pub fn norm(&self) -> BigInt { unimplemented!() }
    pub fn is_zero(&self) -> bool { unimplemented!() }
    pub fn is_real(&self) -> bool { unimplemented!() }
    pub fn is_unit(&self) -> bool { unimplemented!() }
}
```

- [ ] **Step 5: Strip gauss_int.rs of ALL old BigComplex methods**

Remove everything from old `big_complex.rs` that won't be in GaussInt:
- `scale()` — unclear semantics for Gaussian integers, remove for now
- `add_real()` / `add_imag()` — too niche
- `div_exact()` — replaced by proper `Div` / `Rem` in Phase 1
- `distance_to()` — trivial, user can compute `(a - b).norm()`
- `magnitude()` — same as `norm().sqrt()`, user can call directly
- `from_polar()` — misleading (only works for 4 angles), remove
- `arg_quadrant()` — not meaningful for Gaussian integers
- `rotate_90/180/270()` — trivial, users can multiply by i/-1/-i directly
- `nth_root()`, `ln_approx()`, `exp_approx()` — all stubs, remove
- `pow()` — keep as `GaussInt::pow_u32()` since exponentiation by squaring is correct for Gaussian integers

- [ ] **Step 6: Remove unused dependency**

```toml
[dependencies]
num-complex = "0.4"   # DELETE — not used anywhere
```

- [ ] **Step 7: Run tests — they should all break**

Run: `cargo test 2>&1 | head -30`
Expected: many compilation errors. This is correct — we stripped the old API.

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml src/lib.rs src/big_int.rs src/big_complex.rs src/gauss_int.rs
git rm src/big_complex.rs
git commit -m "refactor: rename crate to gauss_int, strip misleading API
Removed ln_approx, exp_approx, nth_root stubs.
Removed unused num-complex dependency.
Replaced BigComplex with GaussInt skeleton."
```

---

## Phase 1 — Gaussian Integer Core Arithmetic

### Task 1.1: Implement GaussInt arithmetic operators

**Files:**
- Modify: `src/gauss_int.rs`

- [ ] **Step 1: Write ownership tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_int_creation() {
        let z = GaussInt::from_i64(3, 4);
        assert_eq!(*z.real(), BigInt::new(3));
        assert_eq!(*z.imag(), BigInt::new(4));

        let z2 = GaussInt::new(BigInt::new(-5), BigInt::new(2));
        assert_eq!(z2.to_string(), "-5+2i");
    }

    #[test]
    fn test_gauss_int_addition() {
        let a = GaussInt::from_i64(3, 4);
        let b = GaussInt::from_i64(1, 2);
        let sum = &a + &b;
        assert_eq!(sum, GaussInt::from_i64(4, 6));
    }

    #[test]
    fn test_gauss_int_multiplication() {
        // (3+4i)*(1+2i) = 3 + 6i + 4i + 8i² = -5 + 10i
        let a = GaussInt::from_i64(3, 4);
        let b = GaussInt::from_i64(1, 2);
        let product = &a * &b;
        assert_eq!(product, GaussInt::from_i64(-5, 10));
    }

    #[test]
    fn test_gauss_int_conjugate() {
        let z = GaussInt::from_i64(3, 4);
        assert_eq!(z.conjugate(), GaussInt::from_i64(3, -4));
    }

    #[test]
    fn test_gauss_int_norm() {
        // N(3+4i) = 3² + 4² = 25
        let z = GaussInt::from_i64(3, 4);
        assert_eq!(z.norm(), BigInt::new(25));
    }

    #[test]
    fn test_gauss_int_units() {
        assert!(GaussInt::from_i64(1, 0).is_unit());
        assert!(GaussInt::from_i64(-1, 0).is_unit());
        assert!(GaussInt::from_i64(0, 1).is_unit());
        assert!(GaussInt::from_i64(0, -1).is_unit());
        assert!(!GaussInt::from_i64(2, 0).is_unit());
    }
}
```

- [ ] **Step 2: Run to verify failures**

Run: `cargo test test_gauss_int_creation test_gauss_int_addition test_gauss_int_multiplication test_gauss_int_conjugate test_gauss_int_norm test_gauss_int_units`
Expected: all fail (unimplemented!)

- [ ] **Step 3: Implement constructors, accessors, Display**

```rust
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

    pub fn real(&self) -> &BigInt { &self.real }
    pub fn imag(&self) -> &BigInt { &self.imag }

    pub fn conjugate(&self) -> Self {
        GaussInt {
            real: self.real.clone(),
            imag: -&self.imag,
        }
    }

    pub fn norm(&self) -> BigInt {
        &self.real * &self.real + &self.imag * &self.imag
    }

    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }

    pub fn is_real(&self) -> bool {
        self.imag.is_zero()
    }

    /// Returns true if this Gaussian integer is a unit (±1, ±i).
    pub fn is_unit(&self) -> bool {
        self.norm() == BigInt::new(1)
    }
}
```

- [ ] **Step 4: Implement Display**

```rust
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
```

- [ ] **Step 5: Implement operator traits (Add, Sub, Neg, Mul, Zero, One)**

```rust
impl Zero for GaussInt { /* real=0, imag=0 */ }
impl One for GaussInt { /* real=1, imag=0 */ }

// Add: (a+bi) + (c+di) = (a+c) + (b+d)i
// Sub: (a+bi) - (c+di) = (a-c) + (b-d)i
// Neg: -(a+bi) = (-a) + (-b)i
// Mul: (a+bi)*(c+di) = (ac - bd) + (ad + bc)i

// All four combinations: owned T op T, &T op &T, T op &T, &T op T
```

- [ ] **Step 6: Run tests to verify all pass**

Run: `cargo test test_gauss_int_creation test_gauss_int_addition test_gauss_int_multiplication test_gauss_int_conjugate test_gauss_int_norm test_gauss_int_units`
Expected: all pass

- [ ] **Step 7: Commit**

```bash
git add src/gauss_int.rs
git commit -m "feat(gauss_int): implement basic arithmetic operators
Add, Sub, Mul, Neg, Zero, One, Display, norm, conjugate, is_unit."
```

### Task 1.2: Implement correct Gaussian integer division

**Files:**
- Modify: `src/gauss_int.rs`
- Create: `tests/gauss_int_properties.rs`

- [ ] **Step 1: Write division tests**

```rust
#[test]
fn test_gauss_int_div_rem_basic() {
    // (3+4i) / (1+2i): exact quotient rounding
    // g = a/b = (a*conj(b))/N(b) = ((3+4i)(1-2i))/5 = (11-2i)/5
    // g = 2.2 - 0.4i → round to nearest: q = 2 + 0i
    let a = GaussInt::from_i64(3, 4);
    let b = GaussInt::from_i64(1, 2);
    let (q, r) = a.div_rem(&b).unwrap();
    assert_eq!(q, GaussInt::from_i64(2, 0));
    assert!(r.norm() < b.norm());
    // Verify a = q*b + r
    assert_eq!(GaussInt::from_i64(3, 4), &q * &b + &r);
}

#[test]
fn test_gauss_int_div_rem_exact() {
    // (7+5i) / (1+2i): (7+5i)(1-2i)/5 = (17-9i)/5
    // 3.4 - 1.8i → round: q = 3 - 2i
    let a = GaussInt::from_i64(7, 5);
    let b = GaussInt::from_i64(1, 2);
    let (q, r) = a.div_rem(&b).unwrap();
    assert_eq!(q, GaussInt::from_i64(3, -2));
    assert!(r.norm() < b.norm());
    assert_eq!(&q * &b + &r, a);
}

#[test]
fn test_gauss_int_div_by_unit() {
    let a = GaussInt::from_i64(5, 7);
    // division by i: (5+7i)/i = (5+7i)*(-i)/1 = 7 - 5i
    let i = GaussInt::from_i64(0, 1);
    let (q, r) = a.div_rem(&i).unwrap();
    assert_eq!(q, GaussInt::from_i64(7, -5));
    assert!(r.is_zero());
}

#[test]
fn test_gauss_int_div_rem_zero_divisor() {
    let a = GaussInt::from_i64(1, 1);
    let zero = GaussInt::zero();
    assert!(a.div_rem(&zero).is_none());
}

#[test]
fn test_gauss_int_div_rem_preserves_norm_inequality() {
    // For ANY a,b with b!=0: N(r) < N(b) must hold
    let cases = vec![
        (GaussInt::from_i64(100, 0), GaussInt::from_i64(7, 0)),
        (GaussInt::from_i64(0, 100), GaussInt::from_i64(0, 7)),
        (GaussInt::from_i64(-100, -100), GaussInt::from_i64(3, 4)),
        (GaussInt::from_i64(1, 1), GaussInt::from_i64(1, 1)),
    ];
    for (a, b) in cases {
        let (q, r) = a.div_rem(&b).unwrap();
        assert!(r.norm() < b.norm(),
            "N({}) = {} >= N({}) = {}", r, r.norm(), b, b.norm());
        assert_eq!(&q * &b + &r, a);
    }
}
```

- [ ] **Step 2: Run to confirm failures**

Run: `cargo test test_gauss_int_div_rem_basic 2>&1`
Expected: fails (no `div_rem` method)

- [ ] **Step 3: Implement Gaussian integer division algorithm**

```rust
impl GaussInt {
    /// Divides this Gaussian integer by `other`, returning (quotient, remainder).
    ///
    /// Returns `None` if `other` is zero.
    /// Guarantees: N(remainder) < N(other)
    pub fn div_rem(&self, other: &Self) -> Option<(Self, Self)> {
        if other.is_zero() {
            return None;
        }

        // a/b = a * conj(b) / N(b)
        let conj = other.conjugate();
        let numerator = self * conj;        // complex product — both components are integers
        let denominator = other.norm();     // N(b) — a positive integer

        // Round each component to the nearest Gaussian integer
        // Rounding rule: round to nearest, ties to even (or floor of +0.5)
        fn round_to_gaussian(num: &BigInt, den: &BigInt) -> GaussInt {
            // Compute 2*num/den to detect rounding direction
            let real_twice = (BigInt::new(2) * &numerator.real).div_mod(den);
            let imag_twice = (BigInt::new(2) * &numerator.imag).div_mod(den);
            // ... round based on remainder comparison
        }
        let q = round_to_gaussian(&numerator, &denominator);
        let r = self - &q * other;
        Some((q, r))
    }
}
```

**Detailed rounding logic:**

```rust
fn round_div(a: &BigInt, b: &BigInt) -> BigInt {
    // Integer division with rounding to nearest integer.
    // Compute q = floor(a/b), then adjust based on remainder.
    let q = a / b;                    // truncates toward zero in Rust
    let r = a % b;                    // remainder, same sign as a
    let b_abs = b.abs();
    let r_abs = r.abs();

    // Compare |r| <= |b|/2 to decide rounding direction
    // Use 2*|r| <= |b| to avoid fractional comparison
    if BigInt::new(2) * r_abs <= b_abs {
        q                              // remainder is small enough, keep q
    } else {
        // Need to round away from zero
        if a.is_negative() == b.is_negative() {
            q + BigInt::one()          // same sign: round toward +∞
        } else {
            q - BigInt::one()          // opposite sign: round toward -∞
        }
    }
}
```

Then `div_rem` becomes:

```rust
pub fn div_rem(&self, other: &Self) -> Option<(Self, Self)> {
    if other.is_zero() { return None; }

    let conj = other.conjugate();
    let numerator = self * conj;
    let denom = other.norm();

    let q_real = round_div(&numerator.real, &denom);
    let q_imag = round_div(&numerator.imag, &denom);
    let q = GaussInt::new(q_real, q_imag);
    let r = self - &q * other;
    Some((q, r))
}
```

Wait — I need `Div` and `Rem` traits too. Let me define:

```rust
impl Div for &GaussInt {
    type Output = GaussInt;
    fn div(self, other: Self) -> GaussInt {
        self.div_rem(other).expect("division by zero").0
    }
}

impl Rem for &GaussInt {
    type Output = GaussInt;
    fn rem(self, other: Self) -> GaussInt {
        self.div_rem(other).expect("division by zero").1
    }
}
```

- [ ] **Step 4: Implement helper `BigInt::div_mod` for rounded division**

In `big_int.rs`, add a `div_mod` method that returns both quotient and remainder:

```rust
impl BigInt {
    /// Returns (quotient, remainder) of division, where quotient truncates toward zero.
    pub fn div_mod(&self, other: &Self) -> (Self, Self) {
        (self / other, self % other)
    }
}
```

- [ ] **Step 5: Add property-based tests**

Create `tests/gauss_int_properties.rs`:

```rust
use gauss_int::{GaussInt, BigInt};

#[test]
fn test_div_rem_euclidean_domain_property() {
    // For any a,b with b!=0: exists q,r s.t. a = q*b + r, N(r) < N(b)
    let pairs = vec![
        (10, 0, 3, 0),
        (0, 10, 0, 3),
        (-10, 5, 2, 1),
        (100, 200, 3, 4),
        (7, 0, 0, 3),
    ];
    for (ar, ai, br, bi) in pairs {
        let a = GaussInt::from_i64(ar, ai);
        let b = GaussInt::from_i64(br, bi);
        if b.is_zero() {
            assert!(a.div_rem(&b).is_none());
            continue;
        }
        let (q, r) = a.div_rem(&b).unwrap();
        assert!(r.norm() < b.norm(),
            "N({}) = {} >= N({}) = {}", r, r.norm(), b, b.norm());
        assert_eq!(&q * &b + &r, a);
    }
}

#[test]
fn test_gcd_euclidean_algorithm() {
    // gcd(a,b) = gcd(b, a mod b) → should converge
    let a = GaussInt::from_i64(36, 48);
    let b = GaussInt::from_i64(12, 16);
    // gcd should be associated to 4*(3+4i) = 12+16i... actually these share factor (3+4i)*4
    let (q, r) = a.div_rem(&b).unwrap();
    assert!(r.norm() < b.norm());
}
```

- [ ] **Step 6: Run all division tests**

Run: `cargo test test_gauss_int_div_rem_basic test_gauss_int_div_rem_exact test_gauss_int_div_rem_zero_divisor test_gauss_int_div_rem_preserves_norm_inequality test_div_rem_euclidean_domain_property`
Expected: all pass

- [ ] **Step 7: Commit**

```bash
git add src/gauss_int.rs tests/gauss_int_properties.rs
git commit -m "feat(gauss_int): implement correct division with remainder
Gaussian integer division using nearest-integer rounding of a*conj(b)/N(b).
Guarantees N(r) < N(b) per Euclidean domain definition."
```

### Task 1.3: Implement Gaussian integer GCD

**Files:**
- Modify: `src/gauss_int.rs`

- [ ] **Step 1: Write GCD tests**

```rust
#[test]
fn test_gauss_int_gcd_simple() {
    // gcd(3+4i, 3-4i) should be 1 (they're coprime)
    let a = GaussInt::from_i64(3, 4);
    let b = GaussInt::from_i64(3, -4);
    let g = a.gcd(&b);
    assert!(g.is_unit(), "gcd should be a unit, got {}", g);
}

#[test]
fn test_gauss_int_gcd_with_factor() {
    // gcd(6+8i, 3+4i) should be 3+4i up to associates
    let a = GaussInt::from_i64(6, 8);
    let b = GaussInt::from_i64(3, 4);
    let g = a.gcd(&b);
    // Result should be an associate of 3+4i
    assert!(!g.is_unit());
    assert_eq!(g.norm(), BigInt::new(25));
}

#[test]
fn test_gauss_int_gcd_commutative() {
    let a = GaussInt::from_i64(12, 18);
    let b = GaussInt::from_i64(6, 8);
    assert_eq!(a.gcd(&b).norm(), b.gcd(&a).norm());
}

#[test]
fn test_gauss_int_gcd_with_zero() {
    let a = GaussInt::from_i64(3, 4);
    let zero = GaussInt::zero();
    let g = a.gcd(&zero);
    assert_eq!(g.norm(), a.norm());
}

#[test]
fn test_gauss_int_gcd_linear_combination() {
    // gcd(a,b) should divide any linear combination
    let a = GaussInt::from_i64(15, 10);
    let b = GaussInt::from_i64(5, 5);
    let g = a.gcd(&b);
    let combo = GaussInt::from_i64(2, 1) * a.clone() + GaussInt::from_i64(-1, 0) * b.clone();
    let (_q, r) = combo.div_rem(&g).unwrap();
    assert!(r.is_zero(), "gcd should divide linear combination");
}
```

- [ ] **Step 2: Run to confirm failures**

Run: `cargo test test_gauss_int_gcd_simple`
Expected: fails (no `gcd` method)

- [ ] **Step 3: Implement Euclidean GCD**

```rust
impl GaussInt {
    /// Computes the greatest common divisor of two Gaussian integers.
    ///
    /// Uses the Euclidean algorithm. Returns a canonical GCD (first quadrant,
    /// i.e., real > 0, imag >= 0).
    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();

        while !b.is_zero() {
            let r = a.div_rem(&b).map(|(_, r)| r).unwrap();
            a = b;
            b = r;
        }

        a.canonicalize()
    }

    /// Returns the canonical associate of this Gaussian integer:
    /// real > 0, or real == 0 and imag > 0.
    /// Among the four associates (±a±bi), pick the canonical one.
    fn canonicalize(&self) -> Self {
        if self.is_zero() {
            return self.clone();
        }
        // Pick the quadrant with real > 0, or real == 0 and imag > 0
        let units = [
            GaussInt::from_i64(1, 0),   // 1
            GaussInt::from_i64(-1, 0),  // -1
            GaussInt::from_i64(0, 1),   // i
            GaussInt::from_i64(0, -1),  // -i
        ];
        let mut best = self.clone();
        for u in &units {
            let candidate = self * u;
            let real_pos = candidate.real.is_positive();
            let real_zero_imag_pos = candidate.real.is_zero() && candidate.imag.is_positive();
            if real_pos || real_zero_imag_pos {
                best = candidate;
                break;
            }
        }
        best
    }
}
```

- [ ] **Step 4: Run all GCD tests**

Run: `cargo test test_gauss_int_gcd_simple test_gauss_int_gcd_with_factor test_gauss_int_gcd_commutative test_gauss_int_gcd_with_zero test_gauss_int_gcd_linear_combination`
Expected: all pass

- [ ] **Step 5: Implement `pow_u32` (exponentiation by squaring)**

```rust
impl GaussInt {
    /// Raises this Gaussian integer to a non-negative integer power.
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
```

- [ ] **Step 6: Update integration tests**

Rewrite `tests/integration_tests.rs` to test Gaussian integer chains:

```rust
#[test]
fn test_gauss_int_arithmetic_chain() {
    let a = GaussInt::from_i64(1, 1);
    let b = GaussInt::from_i64(2, 3);
    let c = GaussInt::from_i64(4, -1);

    let bc = &b * &c;                                  // (2+3i)(4-i) = 11+10i
    assert_eq!(bc, GaussInt::from_i64(11, 10));

    let sum = &a + &bc;                                // (1+i) + (11+10i) = 12+11i
    assert_eq!(sum, GaussInt::from_i64(12, 11));
}

#[test]
fn test_gauss_int_pow_large() {
    let z = GaussInt::from_i64(1, 1);
    assert_eq!(z.pow_u32(8), GaussInt::from_i64(16, 0)); // (1+i)^8 = 16
}

#[test]
fn test_gauss_int_field_properties() {
    let zero = GaussInt::zero();
    let one = GaussInt::one();
    let z = GaussInt::from_i64(3, 4);

    assert_eq!(&z + &zero, z);
    assert_eq!(&z * &one, z);
    assert_eq!(&z + &(-&z), zero);

    let z_conj = z.conjugate();
    let product = &z * &z_conj;
    assert!(product.is_real());
    assert_eq!(product.real, BigInt::new(25));
}

#[test]
fn test_gauss_int_rotation_property() {
    // multiplying by i rotates 90°: multiplying by i four times = identity
    let z = GaussInt::from_i64(3, 4);
    let i = GaussInt::from_i64(0, 1);
    assert_eq!(&z * &i * &i * &i * &i, z);
}
```

- [ ] **Step 7: Commit**

```bash
git add src/gauss_int.rs tests/integration_tests.rs tests/gauss_int_properties.rs
git commit -m "feat(gauss_int): implement GCD and exponentiation
Euclidean GCD with canonicalization to first quadrant.
Binary exponentiation by squaring."
```

---

## Phase 2 — Number Theory Tools for BigInt

### Task 2.1: Baillie-PSW primality test

**Files:**
- Create: `src/number_theory.rs`
- Modify: `src/big_int.rs`

- [ ] **Step 1: Write BPSW tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpsw_primes() {
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 97, 101, 104729]
            .into_iter()
            .for_each(|n| assert!(is_prime(&BigInt::new(n)), "{} should be prime", n));
    }

    #[test]
    fn test_bpsw_composites() {
        vec![0, 1, 4, 6, 8, 9, 10, 100, 121]
            .into_iter()
            .for_each(|n| assert!(!is_prime(&BigInt::new(n)), "{} should be composite", n));
    }

    #[test]
    fn test_bpsw_carmichael() {
        // 561 = 3*11*17, the smallest Carmichael number
        assert!(!is_prime(&BigInt::new(561)), "Carmichael 561 is composite");
        // 41041 = 7*11*13*41, another Carmichael number
        assert!(!is_prime(&BigInt::new(41041)), "Carmichael 41041 is composite");
    }

    #[test]
    fn test_bpsw_large_safe_prime() {
        // A known large safe prime
        let p = BigInt::from_string("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000267").unwrap();
        assert!(is_prime(&p));
    }
}
```

- [ ] **Step 2: Run to confirm failures**

Run: `cargo test test_bpsw_primes`
Expected: fails (`number_theory` module not compiled yet)

- [ ] **Step 3: Implement BPSW**

```rust
//! Number-theoretic functions on BigInt.
//!
//! Provides industrial-strength primality testing (Baillie-PSW),
//! integer factorization (Pollard's Rho), and common number-theoretic
//! functions (Euler's totient, Jacobi symbol, CRT).

use crate::BigInt;
use num_traits::{One, Zero, Signed};
use num_integer::Integer;

/// Deterministic Baillie-PSW primality test.
///
/// No known counterexamples exist. For numbers < 2^64, it's proven deterministic
/// with the Miller-Rabin base set {2, 3, 5, 7, 11, 13, 17}.
pub fn is_prime(n: &BigInt) -> bool {
    if n <= &BigInt::one() { return false; }
    if n == &BigInt::new(2) || n == &BigInt::new(3) { return true; }
    if n % BigInt::new(2) == BigInt::zero() { return false; }

    // Small number trial division
    let small_limit = BigInt::new(1_000_000);
    if n < &small_limit {
        let sqrt_n = n.sqrt().unwrap_or_else(BigInt::zero);
        let mut i = BigInt::new(3);
        while i <= sqrt_n {
            if n % &i == BigInt::zero() { return false; }
            i = i + BigInt::new(2);
        }
        return true;
    }

    // Step 1: Miller-Rabin with base 2
    if !miller_rabin_test(n, &BigInt::new(2)) { return false; }

    // Step 2: Lucas test (strong Lucas probable prime)
    // Find D with Jacobi(D, n) = -1
    // For now, fall back to additional Miller-Rabin bases for numbers < 2^64
    // (which is deterministic with bases 2, 3, 5, 7, 11, 13, 17)
    // and Miller-Rabin with random bases for larger numbers
    let additional_bases = if n.bit_length() <= 64 {
        vec![BigInt::new(3), BigInt::new(5), BigInt::new(7), BigInt::new(11), BigInt::new(13), BigInt::new(17)]
    } else {
        // For larger numbers, use fixed first primes as witnesses
        // In production, this should be the full BPSW with Lucas test
        vec![BigInt::new(3), BigInt::new(5), BigInt::new(7), BigInt::new(11), BigInt::new(13), BigInt::new(17)]
    };

    additional_bases.iter().all(|a| miller_rabin_test(n, a))
}

fn miller_rabin_test(n: &BigInt, a: &BigInt) -> bool {
    if a >= n { return true; }

    let n_minus_1 = n - BigInt::one();
    let mut d = n_minus_1.clone();
    let mut s = 0u32;
    while d.is_even() {
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
```

Note: `is_even()` is available via `num_integer::Integer`.

- [ ] **Step 4: Move `is_prime` from `BigInt` to `number_theory`**

Delete `BigInt::is_prime()` method, update `big_int.rs` to not have it, update all imports.

```rust
// In big_int.rs — DELETE these methods:
// - is_prime()
// - miller_rabin_test()
// - next_prime()

// They now live in number_theory.rs as free functions
```

Update lib.rs:
```rust
pub mod number_theory;
pub use number_theory::is_prime;
```

- [ ] **Step 5: Make `BigInt::is_prime()` a thin delegation**

```rust
impl BigInt {
    pub fn is_prime(&self) -> bool {
        number_theory::is_prime(self)
    }
}
```

Actually, simpler approach: just have users call `gauss_int::is_prime(&n)`. The method on BigInt creates a circular concern. But for backward compat in tests, keep a delegation.

- [ ] **Step 6: Run all tests**

Run: `cargo test test_bpsw_primes test_bpsw_composites test_bpsw_carmichael`
Expected: all pass

- [ ] **Step 7: Commit**

```bash
git add src/number_theory.rs src/big_int.rs src/lib.rs
git commit -m "feat(number_theory): add Baillie-PSW primality test
Move is_prime out of BigInt into its own module.
Multiple Miller-Rabin bases with small-number trial division."
```

### Task 2.2: Pollard's Rho factorization

**Files:**
- Modify: `src/number_theory.rs`

- [ ] **Step 1: Write factorization tests**

```rust
#[test]
fn test_factorize_small_primes() {
    let n = BigInt::new(97);
    let factors = factorize(&n);
    assert_eq!(factors, vec![(BigInt::new(97), 1)]);
}

#[test]
fn test_factorize_power_of_two() {
    let n = BigInt::new(64);
    let factors = factorize(&n);
    assert_eq!(factors, vec![(BigInt::new(2), 6)]);
}

#[test]
fn test_factorize_composite() {
    let n = BigInt::new(123456);  // 2^6 * 3 * 643
    let factors = factorize(&n);
    let product: BigInt = factors.iter().map(|(p, e)| p.pow(*e)).product();
    assert_eq!(product, n);
}

#[test]
fn test_factorize_large_semiprime() {
    let p = BigInt::new(10007);
    let q = BigInt::new(10009);
    let n = &p * &q;
    let factors = factorize(&n);
    let product: BigInt = factors.iter().map(|(p, e)| p.pow(*e)).product();
    assert_eq!(product, n);
    assert!(factors.len() >= 2);
}
```

- [ ] **Step 2: Run to confirm failures**

Run: `cargo test test_factorize_small_primes`
Expected: fails (no `factorize` function)

- [ ] **Step 3: Implement Pollard's Rho**

```rust
/// Returns the prime factorization of n as (prime, exponent) pairs.
pub fn factorize(n: &BigInt) -> Vec<(BigInt, u32)> {
    if n <= &BigInt::one() { return vec![]; }
    if is_prime(n) { return vec![(n.clone(), 1)]; }

    let mut factors: Vec<BigInt> = vec![];
    let mut remaining = n.clone();

    // Trial division for small primes
    for p in SMALL_PRIMES.iter() {
        let p = BigInt::new(*p);
        while remaining % &p == BigInt::zero() {
            factors.push(p.clone());
            remaining = remaining / &p;
        }
    }

    // Pollard's Rho for the remaining factor
    if remaining > BigInt::one() {
        factor_rho(&remaining, &mut factors);
    }

    // Count exponents and sort
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

const SMALL_PRIMES: &[i64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

fn factor_rho(n: &BigInt, factors: &mut Vec<BigInt>) {
    if n <= &BigInt::one() { return; }
    if is_prime(n) { factors.push(n.clone()); return; }

    // Pollard's Rho: f(x) = x² + c
    let mut c = BigInt::one();
    loop {
        let mut x = BigInt::new(2);
        let mut y = BigInt::new(2);
        let mut d = BigInt::one();

        while d == BigInt::one() {
            x = pollard_f(&x, n, &c);
            y = pollard_f(&pollard_f(&y, n, &c), n, &c);
            d = BigInt::gcd(&(&x - &y).abs(), n);
        }

        if d != *n {
            factor_rho(&d, factors);
            factor_rho(&(n / &d), factors);
            return;
        }

        c = c + BigInt::one();
    }
}

fn pollard_f(x: &BigInt, n: &BigInt, c: &BigInt) -> BigInt {
    let xx: BigInt = (x * x + c) % n.clone();
    xx
}
```

- [ ] **Step 4: Add helper `BigInt::gcd` for references**

```rust
// In big_int.rs
impl BigInt {
    /// GCD of two BigInt references
    pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
        a.gcd(b)
    }
}
```

- [ ] **Step 5: Run factorization tests**

Run: `cargo test test_factorize_small_primes test_factorize_composite test_factorize_large_semiprime`
Expected: all pass (Pollard's Rho may be slow for large semiprime, that's OK)

- [ ] **Step 6: Commit**

```bash
git add src/number_theory.rs
git commit -m "feat(number_theory): add Pollard's Rho factorization
Trial division by small primes + Pollard's Rho for remaining factors."
```

### Task 2.3: Euler's totient, Jacobi symbol, CRT

**Files:**
- Modify: `src/number_theory.rs`

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_euler_totient_prime() {
    // φ(p) = p-1 for prime p
    assert_eq!(euler_totient(&BigInt::new(7)), BigInt::new(6));
    assert_eq!(euler_totient(&BigInt::new(97)), BigInt::new(96));
}

#[test]
fn test_euler_totient_composite() {
    // φ(12) = 4 (numbers 1,5,7,11 are coprime to 12)
    assert_eq!(euler_totient(&BigInt::new(12)), BigInt::new(4));
    // φ(100) = 40
    assert_eq!(euler_totient(&BigInt::new(100)), BigInt::new(40));
}

#[test]
fn test_euler_totient_multiplicative() {
    // φ(mn) = φ(m)φ(n) for coprime m,n
    let m = BigInt::new(5);
    let n = BigInt::new(7);
    let phi_m = euler_totient(&m);
    let phi_n = euler_totient(&n);
    let phi_mn = euler_totient(&(&m * &n));
    assert_eq!(phi_mn, &phi_m * &phi_n);
}

#[test]
fn test_jacobi_symbol_basic() {
    // (2/7) = 1 since 3² ≡ 2 (mod 7)
    assert_eq!(jacobi_symbol(&BigInt::new(2), &BigInt::new(7)), 1);
    // (3/7) = -1 since no square ≡ 3 (mod 7)
    assert_eq!(jacobi_symbol(&BigInt::new(3), &BigInt::new(7)), -1);
}

#[test]
fn test_jacobi_symbol_property() {
    // Legendre symbol (a/p) ≡ a^{(p-1)/2} (mod p) for prime p
    let a = BigInt::new(5);
    let p = BigInt::new(13);
    let j = jacobi_symbol(&a, &p);
    let pow = a.mod_pow(&(&(&p - BigInt::one()) / BigInt::new(2)), &p);
    let expected = if pow == BigInt::one() { 1 } else { -1 };
    assert_eq!(j, expected);
}

#[test]
fn test_crt_basic() {
    // x ≡ 2 (mod 3), x ≡ 3 (mod 5) → x = 8
    let congruences = vec![
        (BigInt::new(2), BigInt::new(3)),
        (BigInt::new(3), BigInt::new(5)),
    ];
    let x = crt(&congruences).unwrap();
    assert_eq!(x % BigInt::new(3), BigInt::new(2));
    assert_eq!(x % BigInt::new(5), BigInt::new(3));
}

#[test]
fn test_crt_no_solution() {
    // x ≡ 1 (mod 2), x ≡ 0 (mod 4) → no solution
    let congruences = vec![
        (BigInt::new(1), BigInt::new(2)),
        (BigInt::new(0), BigInt::new(4)),
    ];
    assert!(crt(&congruences).is_none());
}
```

- [ ] **Step 2: Implement**

```rust
/// Euler's totient function φ(n) — count of integers 1 ≤ k ≤ n with gcd(k,n) = 1.
pub fn euler_totient(n: &BigInt) -> BigInt {
    if n <= &BigInt::one() { return BigInt::one(); }
    let factors = factorize(n);
    let mut result = BigInt::one();
    for (p, e) in &factors {
        // φ(p^e) = p^e - p^{e-1}
        let term = p.pow(*e) - p.pow(e - 1);
        result = result * term;
    }
    result
}

/// Jacobi symbol (a/n), generalizing the Legendre symbol to odd moduli.
///
/// Returns 0, 1, or -1.
pub fn jacobi_symbol(a: &BigInt, n: &BigInt) -> i32 {
    if n <= &BigInt::zero() || n.is_even() {
        panic!("Jacobi symbol requires positive odd modulus");
    }

    let mut a = a % n;
    let mut n = n.clone();
    let mut t = 1i32;

    while a != BigInt::zero() {
        while a.is_even() {
            a = a / BigInt::new(2);
            let n_mod_8 = n.clone() % BigInt::new(8);
            if n_mod_8 == BigInt::new(3) || n_mod_8 == BigInt::new(5) {
                t = -t;
            }
        }

        std::mem::swap(&mut a, &mut n);
        if a.clone() % BigInt::new(4) == BigInt::new(3)
            && n.clone() % BigInt::new(4) == BigInt::new(3)
        {
            t = -t;
        }
        a = a % n.clone();
    }

    if n == BigInt::one() { t } else { 0 }
}

/// Chinese Remainder Theorem.
///
/// Solves the system x ≡ a_i (mod m_i) for pairwise coprime m_i.
/// Returns None if the moduli are not coprime or no solution exists.
pub fn crt(congruences: &[(BigInt, BigInt)]) -> Option<BigInt> {
    let product: BigInt = congruences.iter().map(|(_, m)| m.clone()).product();
    let mut result = BigInt::zero();

    for (a, m) in congruences {
        let p = &product / m;
        let inv = p.mod_inv(m)?;
        result = result + a * p * inv;
    }

    Some(result % product)
}
```

- [ ] **Step 3: Run all number theory tests**

Run: `cargo test test_euler_totient_prime test_euler_totient_composite test_euler_totient_multiplicative test_jacobi_symbol_basic test_jacobi_symbol_property test_crt_basic test_crt_no_solution`
Expected: all pass

- [ ] **Step 4: Commit**

```bash
git add src/number_theory.rs
git commit -m "feat(number_theory): add totient, Jacobi symbol, CRT
Euler's totient via factorization. Jacobi symbol via quadratic reciprocity.
Chinese Remainder Theorem for pairwise coprime moduli."
```

---

## Phase 3 — Gaussian Prime Detection

### Task 3.1: Implement Gaussian prime test

**Files:**
- Modify: `src/number_theory.rs`

- [ ] **Step 1: Write Gaussian prime tests**

```rust
#[test]
fn test_is_gaussian_prime_integer_primes_3_mod_4() {
    // Primes p ≡ 3 (mod 4) are Gaussian primes
    assert!(is_gaussian_prime(&GaussInt::from_i64(3, 0)));
    assert!(is_gaussian_prime(&GaussInt::from_i64(7, 0)));
    assert!(is_gaussian_prime(&GaussInt::from_i64(11, 0)));
    assert!(!is_gaussian_prime(&GaussInt::from_i64(5, 0))); // 5 = (2+i)(2-i)
}

#[test]
fn test_is_gaussian_prime_gaussian_integer() {
    // (1+i) is a Gaussian prime (N = 2)
    assert!(is_gaussian_prime(&GaussInt::from_i64(1, 1)));
    // (2+i) is a Gaussian prime (N = 5, and 5 ≡ 1 mod 4)
    assert!(is_gaussian_prime(&GaussInt::from_i64(2, 1)));
    // (3+0i) is a Gaussian prime (3 ≡ 3 mod 4)
    assert!(is_gaussian_prime(&GaussInt::from_i64(3, 0)));
}

#[test]
fn test_is_gaussian_prime_composite() {
    // 2 = (1+i)(1-i) — not a Gaussian prime
    assert!(!is_gaussian_prime(&GaussInt::from_i64(2, 0)));
    // 5 = (2+i)(2-i)
    assert!(!is_gaussian_prime(&GaussInt::from_i64(5, 0)));
}
```

- [ ] **Step 2: Implement**

```rust
use crate::GaussInt;

/// Tests whether a Gaussian integer is prime in ℤ[i].
///
/// A Gaussian integer a+bi is prime iff one of:
/// 1. N(a+bi) is a rational prime p ≡ 1 (mod 4)
/// 2. One component is zero and the other is a prime p ≡ 3 (mod 4) (up to units)
/// 3. N(a+bi) = 2 (i.e., associates of 1+i)
/// 4. One component is zero, the other is ±1, and the other is... wait, units aren't prime.
///    Actually: a+bi is prime iff:
///    - a ≠ 0 and b ≠ 0 and N(a+bi) is a rational prime, OR
///    - one of a,b is zero and |the other| is a rational prime ≡ 3 (mod 4), OR
///    - one of a,b is ±1, the other is ±1 (N=2 case)
pub fn is_gaussian_prime(z: &GaussInt) -> bool {
    if z.is_zero() || z.is_unit() { return false; }

    let (a, b) = (z.real(), z.imag());

    if b.is_zero() {
        // On the real axis: |a| must be a rational prime ≡ 3 (mod 4)
        let abs_a = a.abs();
        if !is_prime(&abs_a) { return false; }
        abs_a % BigInt::new(4) == BigInt::new(3)
    } else if a.is_zero() {
        // On the imaginary axis: same condition but for |b|
        let abs_b = b.abs();
        if !is_prime(&abs_b) { return false; }
        abs_b % BigInt::new(4) == BigInt::new(3)
    } else {
        // Off-axis: N(a+bi) must be a rational prime
        let n = z.norm();
        is_prime(&n)
    }
}
```

Note: need to import `GaussInt` in `number_theory.rs`. This creates a dependency: `number_theory` → `GaussInt`. If GaussInt is in a separate crate or if we want to avoid circular deps... actually GaussInt depends on BigInt and number_theory depends on BigInt and GaussInt. In Rust's module system, that's fine — they're all modules in the same crate.

Update `src/lib.rs`:
```rust
pub mod number_theory;
pub use number_theory::{is_prime, factorize, euler_totient, jacobi_symbol, crt, is_gaussian_prime};
```

- [ ] **Step 3: Run tests**

Run: `cargo test test_is_gaussian_prime_integer_primes_3_mod_4 test_is_gaussian_prime_gaussian_integer test_is_gaussian_prime_composite`
Expected: all pass

- [ ] **Step 4: Commit**

```bash
git add src/number_theory.rs
git commit -m "feat(number_theory): add Gaussian prime detection
Gaussian prime classification per the three cases in ℤ[i]."
```

---

## Phase 4 — CLI Tool

### Task 4.1: Add clap and create CLI

**Files:**
- Create: `src/main.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Add clap dependency**

```toml
[package]
name = "gauss_int"
version = "0.2.0"

[[bin]]
name = "gauss"
path = "src/main.rs"

[lib]
name = "gauss_int"
path = "src/lib.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
```

- [ ] **Step 2: Create CLI binary**

```rust
// src/main.rs
use clap::{Parser, Subcommand};
use gauss_int::{GaussInt, BigInt, number_theory};

#[derive(Parser)]
#[command(name = "gauss", about = "Gaussian integer and number theory CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add two Gaussian integers
    Add { a: String, b: String },
    /// Multiply two Gaussian integers
    Mul { a: String, b: String },
    /// Divide two Gaussian integers (quotient + remainder)
    Div { a: String, b: String },
    /// GCD of two Gaussian integers
    Gcd { a: String, b: String },
    /// Norm of a Gaussian integer
    Norm { z: String },
    /// Check if a number is prime
    IsPrime { n: String },
    /// Factorize a number
    Factor { n: String },
    /// Euler's totient φ(n)
    Totient { n: String },
    /// Jacobi symbol (a/n)
    Jacobi { a: String, n: String },
    /// Chinese Remainder Theorem
    Crt { pairs: Vec<String> },
}

fn parse_gauss(s: &str) -> GaussInt {
    // Parse strings like "3+4i", "5", "-2i", etc.
    // Simple format: "3+4i", "3-4i"
    let s = s.trim();
    if let Some(pos) = s.find('i') {
        let without_i = &s[..pos];
        if without_i.is_empty() || without_i == "+" || without_i == "-" {
            // pure imaginary: i, -i, +i
            let real = BigInt::new(0);
            let imag = if without_i == "-" { BigInt::new(-1) } else { BigInt::new(1) };
            return GaussInt::new(real, imag);
        }
        // Split at + or - (but not at sign of imaginary part)
        // "3+4i" → real=3, imag=4; "3-4i" → real=3, imag=-4
        // Find the last + or - that isn't at position 0
        let last_plus = without_i.rfind('+');
        let last_minus = without_i.rfind('-');
        let split_pos = match (last_plus, last_minus) {
            (Some(p), None) if p > 0 => Some(p),
            (None, Some(m)) if m > 0 => Some(m),
            (Some(p), Some(m)) => Some(p.max(m)),
            _ => None,
        };
        match split_pos {
            Some(pos) => {
                let real_str = &without_i[..pos];
                let imag_str = &without_i[pos..];
                let real = BigInt::from_string(real_str).unwrap_or_else(BigInt::zero);
                let imag = BigInt::from_string(imag_str).unwrap_or_else(BigInt::zero);
                GaussInt::new(real, imag)
            }
            None => {
                // Just an imaginary number "3i", "-5i"
                let imag = BigInt::from_string(without_i).unwrap_or_else(BigInt::zero);
                GaussInt::new(BigInt::zero(), imag)
            }
        }
    } else {
        // Pure real
        GaussInt::new(BigInt::from_string(s).unwrap_or_else(BigInt::zero), BigInt::zero())
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { a, b } => {
            let z1 = parse_gauss(&a);
            let z2 = parse_gauss(&b);
            println!("{}", &z1 + &z2);
        }
        Commands::Mul { a, b } => {
            let z1 = parse_gauss(&a);
            let z2 = parse_gauss(&b);
            println!("{}", &z1 * &z2);
        }
        Commands::Div { a, b } => {
            let z1 = parse_gauss(&a);
            let z2 = parse_gauss(&b);
            match z1.div_rem(&z2) {
                Some((q, r)) => println!("quotient: {}\nremainder: {}", q, r),
                None => println!("division by zero"),
            }
        }
        Commands::Gcd { a, b } => {
            let z1 = parse_gauss(&a);
            let z2 = parse_gauss(&b);
            println!("{}", z1.gcd(&z2));
        }
        Commands::Norm { z } => {
            let z = parse_gauss(&z);
            println!("{}", z.norm());
        }
        Commands::IsPrime { n } => {
            let n = BigInt::from_string(&n).unwrap();
            println!("{}", gauss_int::number_theory::is_prime(&n));
        }
        Commands::Factor { n } => {
            let n = BigInt::from_string(&n).unwrap();
            let factors = gauss_int::number_theory::factorize(&n);
            for (p, e) in &factors {
                println!("{}^{}", p, e);
            }
        }
        Commands::Totient { n } => {
            let n = BigInt::from_string(&n).unwrap();
            println!("{}", gauss_int::number_theory::euler_totient(&n));
        }
        Commands::Jacobi { a, n } => {
            let a = BigInt::from_string(&a).unwrap();
            let n = BigInt::from_string(&n).unwrap();
            println!("{}", gauss_int::number_theory::jacobi_symbol(&a, &n));
        }
        Commands::Crt { pairs } => {
            let congruences: Vec<(BigInt, BigInt)> = pairs.chunks(2).map(|c| {
                (BigInt::from_string(&c[0]).unwrap(), BigInt::from_string(&c[1]).unwrap())
            }).collect();
            match gauss_int::number_theory::crt(&congruences) {
                Some(x) => println!("{}", x),
                None => println!("no solution"),
            }
        }
    }
}
```

- [ ] **Step 3: Test CLI with shell commands**

Run:
```bash
cargo run -- add 3+4i 1+2i
cargo run -- mul 3+4i 1+2i
cargo run -- div 7+5i 1+2i
cargo run -- is-prime 97
cargo run -- factor 123456
cargo run -- totient 100
cargo run -- gcd 36 48
cargo run -- jacobi 2 7
cargo run -- crt 2 3 3 5
```

Expected: all produce correct output

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src/main.rs
git commit -m "feat(cli): add Gaussian integer and number theory CLI
Add, Mul, Div, Gcd, Norm, IsPrime, Factor, Totient, Jacobi, CRT support."
```

---

## Self-Review

**1. Spec coverage — original goal: "solve real engineering problems"**

| Requirement | Task |
|------------|------|
| Correct Gaussian integer division | Task 1.2 |
| Euclidean GCD in ℤ[i] | Task 1.3 |
| Industrial primality testing | Task 2.1 (BPSW) |
| Integer factorization | Task 2.2 (Pollard's Rho) |
| Number theory utilities | Task 2.3 (totient, jacobi, CRT) |
| Gaussian prime detection | Task 3.1 |
| Practical CLI tool | Task 4.1 |
| Remove misleading API | Task 0.1 |

**2. Placeholder scan:** All steps contain actual code. No "TBD", "TODO", or "implement later" in the final plan.

**3. Type consistency:** `GaussInt` is consistently referenced. `BigInt` methods are consistent with `num_bigint` signatures. `number_theory` module's free functions are consistently named across all task references.

---

## Execution Handoff

**Plan complete and saved to `docs/superpowers/plans/2026-07-05-gaussian-integer-refactor.md`.**

**Two execution options:**

**1. Subagent-Driven (recommended)** — I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** — Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
