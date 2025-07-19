# Big Complex Number Calculator

[Chinese Version](./README-zh.md)

A calculation module implemented in Rust language that supports large number complex operations, providing a rich set of mathematical operation functions.

## Core Modules

### BigInt - Large Integer Module

#### Basic Functions

- ✅ Creation and basic operations (addition, subtraction, multiplication, division, modulo) of large integers
- ✅ String parsing and display
- ✅ Byte sequence conversion
- ✅ Sign detection and absolute value calculation
- ✅ Comparison operations

#### Mathematical Operation Functions

- ✅ Power operation (`pow`)
- ✅ Square root calculation (`sqrt`)
- ✅ Greatest common divisor (`gcd`)
- ✅ Least common multiple (`lcm`)
- ✅ Modular power operation (`mod_pow`)
- ✅ Modular inverse operation (`mod_inv`)

#### New Functions

- ✅ **Factorial calculation** (`factorial`) - Calculate n!
- ✅ **Prime number detection** (`is_prime`) - Determine whether it is a prime number
- ✅ **Next prime number** (`next_prime`) - Find the smallest prime number greater than the current number
- ✅ **Binary operations**:
  - `bit_length()` - Get the binary bit length
  - `count_ones()` - Calculate the number of 1s in the binary
  - `trailing_zeros()` - Calculate the number of trailing zeros
  - `is_power_of_two()` - Determine whether it is a power of 2
  - `next_power_of_two()` - Get the next power of 2

### BigComplex - Large Complex Number Module

#### Basic Functions

- ✅ Creation and basic operations (addition, subtraction, multiplication, division) of complex numbers
- ✅ Access to real and imaginary parts
- ✅ Complex conjugate (`conjugate`)
- ✅ Square of the magnitude (`magnitude_squared`)
- ✅ Scaling operation (`scale`)
- ✅ Complex power operation (`pow`)

#### Geometric and Polar Coordinate Functions

- ✅ **Magnitude calculation** (`magnitude`) - Calculate the magnitude of a complex number
- ✅ **Polar coordinate construction** (`from_polar`) - Create a complex number from polar coordinates
- ✅ **Quadrant determination** (`arg_quadrant`) - Determine the quadrant where the complex number lies
- ✅ **Rotation operations**:
  - `rotate_90()` - Rotate counterclockwise by 90 degrees
  - `rotate_180()` - Rotate by 180 degrees
  - `rotate_270()` - Rotate counterclockwise by 270 degrees

#### Advanced Mathematical Operations

- ✅ **nth root** (`nth_root`) - Calculate the nth root of a complex number
- ✅ **Natural logarithm approximation** (`ln_approx`) - Simplified calculation of the natural logarithm of a complex number
- ✅ **Exponential function approximation** (`exp_approx`) - Simplified calculation of the exponential function of a complex number

## Test Coverage

### Unit Tests

- ✅ BigInt: 18 test functions, covering all functions
- ✅ BigComplex: 17 test functions, covering all functions

### Integration Tests

- ✅ 12 integration tests, including:
  - Large number operation tests
  - Complex number operation chain tests
  - Polynomial evaluation tests
  - Mathematical property verification tests
  - Comprehensive tests of new functions

## Performance Features

- ✅ Supports arbitrary precision large integer operations
- ✅ Efficient algorithm implementation (e.g., optimized trial division for prime number detection)
- ✅ Memory-safe Rust implementation
- ✅ Zero-copy reference operation support

## Usage Examples

```rust
use big_complex::{BigInt, BigComplex};

// Large integer operations
let n = BigInt::new(10);
println!("10! = {}", n.factorial().unwrap());

let num = BigInt::new(97);
println!("{} is prime: {}", num, num.is_prime());

// Complex number operations
let z = BigComplex::from_i64(3, 4);
println!("Magnitude: {}", z.magnitude());
println!("Rotated 90°: {}", z.rotate_90());

// Advanced operations
let roots = BigComplex::from_i64(16, 0).nth_root(2);
println!("Square roots: {:?}", roots);
```

## Compilation and Execution

```bash
# Run all tests
cargo test

# Run the example program
cargo run --example usage

# Run specific tests
cargo test test_big_int_factorial
cargo test test_big_complex_rotation
```

## Project Structure

```
src/
├── lib.rs          # Module export
├── big_int.rs      # Large integer implementation
└── big_complex.rs  # Large complex number implementation

tests/
└── integration_tests.rs  # Integration tests

examples/
└── usage.rs        # Usage example
```

## Dependencies

- `num-bigint` - Underlying implementation of large integers
- `num-traits` - Numerical traits
- `num-complex` - Complex number support
- `num-integer` - Integer operations

## Summary

This project successfully implements a fully functional large number complex operation calculation module, including:

- **28 unit tests** all passed
- **12 integration tests** all passed
- **15 new functions** with corresponding tests
- **Complete example programs** demonstrating all functions

The implementation of each new function follows the "implement - test" development model to ensure code quality and functional correctness.
