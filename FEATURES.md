# Gauss Int — Feature Summary

A Rust library for Gaussian integer arithmetic and number theory.

## GaussInt — Gaussian Integer Module

### Basic Operations

- ✅ Creation (`new`, `from_i64`)
- ✅ Component access (`real`, `imag`)
- ✅ Display formatting (`a+bi`, `a-bi`, `i`, `-i`, etc.)
- ✅ Addition, subtraction, multiplication, negation (all ownership patterns)
- ✅ **Division with remainder** — correct Gaussian integer division with `N(r) < N(b)` guarantee

### Advanced Operations

- ✅ Conjugate (`conjugate`)
- ✅ Norm (`norm`)
- ✅ Unit detection (`is_unit`)
- ✅ GCD via Euclidean algorithm (`gcd`)
- ✅ Exponentiation by squaring (`pow_u32`)

## BigInt — Big Integer Wrapper

### Basic Operations

- ✅ Creation (`new`, `from_string`)
- ✅ Byte sequence conversion (`from_bytes_be`, `to_bytes_be`)
- ✅ Sign and zero detection
- ✅ Comparison and ordering

### Mathematical Operations

- ✅ Power (`pow`)
- ✅ Square root (`sqrt`)
- ✅ GCD and LCM (`gcd`, `lcm`)
- ✅ Modular exponentiation (`mod_pow`)
- ✅ Modular inverse (`mod_inv`)
- ✅ Factorial (`factorial`)
- ✅ Divisibility (`div_mod`)
- ✅ Bit length (`bits`)

## Number Theory Module

- ✅ **Baillie-PSW primality test** (`is_prime`) — deterministic for 64-bit, multiple Miller-Rabin bases for larger numbers
- ✅ **Pollard's Rho factorization** (`factorize`) — trial division + Pollard's Rho
- ✅ **Euler's totient** (`euler_totient`)
- ✅ **Jacobi symbol** (`jacobi_symbol`)
- ✅ **Chinese Remainder Theorem** (`crt`)
- ✅ **Gaussian prime detection** (`is_gaussian_prime`) — full ℤ[i] classification

## CLI

- ✅ `add`, `sub`, `mul` — basic binary operations
- ✅ `div` — division showing quotient and remainder
- ✅ `gcd` — Gaussian integer GCD
- ✅ `norm`, `conj` — unary operations
- ✅ `is-prime`, `factor`, `totient` — number theory
- ✅ `jacobi`, `crt` — modular arithmetic

## Testing

- ✅ 54 unit tests
- ✅ 6 integration tests
- ✅ 13 documentation tests
