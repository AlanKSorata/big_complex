use big_complex::{BigComplex, BigInt};
use num_traits::{One, Zero};

#[test]
fn test_large_number_operations() {
    let a = BigInt::from_string("123456789012345678901234567890").unwrap();
    let b = BigInt::from_string("987654321098765432109876543210").unwrap();

    let sum = &a + &b;
    assert_eq!(sum.to_string(), "1111111110111111111011111111100");

    let product = &a * &b;
    assert!(product.to_string().starts_with("121932631137"));
}

#[test]
fn test_complex_with_large_numbers() {
    let real = BigInt::from_string("12345678901234567890").unwrap();
    let imag = BigInt::from_string("98765432109876543210").unwrap();

    let z = BigComplex::new(real, imag);
    let conj = z.conjugate();

    assert_eq!(conj.real().to_string(), "12345678901234567890");
    assert_eq!(conj.imag().to_string(), "-98765432109876543210");

    let magnitude = z.magnitude_squared();
    let expected = "9907026367383020893179393387654016156200";
    assert_eq!(magnitude.to_string(), expected);
}

#[test]
fn test_complex_arithmetic_chain() {
    let z1 = BigComplex::from_i64(1, 1);
    let z2 = BigComplex::from_i64(2, 3);
    let z3 = BigComplex::from_i64(4, -1);

    let z2z3 = &z2 * &z3;
    assert_eq!(z2z3.real().to_string(), "11"); // (2+3i)(4-i) = 8-2i+12i+3 = 11+10i
    assert_eq!(z2z3.imag().to_string(), "10");

    let final_result = &z1 + &z2z3;
    assert_eq!(final_result.real().to_string(), "12");
    assert_eq!(final_result.imag().to_string(), "11");
}

#[test]
fn test_modular_arithmetic_with_complex() {
    let z = BigComplex::from_i64(5, 7);
    let modulus = BigInt::new(3);

    let scaled = z.scale(&modulus);
    assert_eq!(scaled.real().to_string(), "15");
    assert_eq!(scaled.imag().to_string(), "21");

    let div_result = scaled.div_exact(&modulus);
    assert!(div_result.is_some());
    let original = div_result.unwrap();
    assert_eq!(original.real().to_string(), "5");
    assert_eq!(original.imag().to_string(), "7");
}

#[test]
fn test_complex_pow_with_large_exponents() {
    let z = BigComplex::from_i64(1, 1);
    let z_pow = z.pow(8);

    assert_eq!(z_pow.real().to_string(), "16"); // (1+i)⁸ = 16
    assert_eq!(z_pow.imag().to_string(), "0");
}

#[test]
fn test_gcd_and_lcm_with_complex_scaling() {
    let big_num1 = BigInt::from_string("1234567890").unwrap();
    let big_num2 = BigInt::from_string("9876543210").unwrap();

    let gcd = big_num1.gcd(&big_num2);
    let _lcm = big_num1.lcm(&big_num2);

    assert_eq!(gcd.to_string(), "90");

    let z1 = BigComplex::new(big_num1.clone(), BigInt::new(0));
    let z2 = BigComplex::new(big_num2.clone(), BigInt::new(0));

    let scaled1 = z1.scale(&gcd);
    let scaled2 = z2.scale(&gcd);

    assert_eq!(scaled1.real().to_string(), "111111110100");
    assert_eq!(scaled2.real().to_string(), "888888888900");
}

#[test]
fn test_complex_display_formatting() {
    let test_cases = vec![
        (BigComplex::from_i64(0, 0), "0"),
        (BigComplex::from_i64(1, 0), "1"),
        (BigComplex::from_i64(0, 1), "i"),
        (BigComplex::from_i64(0, -1), "-i"),
        (BigComplex::from_i64(5, 3), "5+3i"),
        (BigComplex::from_i64(5, -3), "5-3i"),
    ];

    for (complex, expected) in test_cases {
        assert_eq!(complex.to_string(), expected);
    }
}

#[test]
fn test_complex_polynomial_evaluation() {
    let coeffs = vec![
        BigComplex::from_i64(1, 0),
        BigComplex::from_i64(2, 0),
        BigComplex::from_i64(3, 0),
    ];

    let x = BigComplex::from_i64(1, 1);
    let mut result = BigComplex::new(BigInt::new(0), BigInt::new(0));
    let mut power = BigComplex::one();

    for coeff in coeffs {
        result = result + (&coeff * &power);
        power = power * x.clone();
    }

    assert_eq!(result.real().to_string(), "3");
    assert_eq!(result.imag().to_string(), "8"); // 1 + 2(1+i) + 3(1+i)² = 1 + 2+2i + 6i = 3+8i
}

#[test]
fn test_complex_field_properties() {
    let zero = BigComplex::zero();
    let one = BigComplex::one();
    let z = BigComplex::from_i64(3, 4);

    assert_eq!(&z + &zero, z);
    assert_eq!(&z * &one, z);
    assert_eq!(&z + &(-&z), BigComplex::zero());

    let z_conj = z.conjugate();
    let product = &z * &z_conj;
    assert!(product.is_real());
    assert_eq!(product.real().to_string(), "25");
}

#[test]
fn test_new_features_integration() {
    // 测试BigInt的新功能
    println!("Testing BigInt new features...");

    // 阶乘测试
    let five = BigInt::new(5);
    let factorial_5 = five.factorial().unwrap();
    assert_eq!(factorial_5.to_string(), "120");

    // 素数测试
    let num = BigInt::new(97);
    assert!(num.is_prime());
    let next_prime = num.next_prime();
    assert_eq!(next_prime.to_string(), "101");

    // 二进制操作测试
    let binary_num = BigInt::new(15);
    assert_eq!(binary_num.bit_length(), 4);
    assert_eq!(binary_num.count_ones(), 4);
    assert!(!binary_num.is_power_of_two());
    assert_eq!(binary_num.next_power_of_two().to_string(), "16");

    // 测试BigComplex的新功能
    println!("Testing BigComplex new features...");

    // 极坐标和旋转测试
    let z = BigComplex::from_i64(3, 4);
    assert_eq!(z.magnitude().to_string(), "5");
    assert_eq!(z.arg_quadrant(), Some(0));

    let z_rotated = z.rotate_90();
    assert_eq!(z_rotated.real().to_string(), "-4");
    assert_eq!(z_rotated.imag().to_string(), "3");

    // 极坐标构造测试
    let polar_z = BigComplex::from_polar(&BigInt::new(5), 1);
    assert_eq!(polar_z.real().to_string(), "0");
    assert_eq!(polar_z.imag().to_string(), "5");

    // 高级数学运算测试
    let square_root_test = BigComplex::from_i64(9, 0);
    let roots = square_root_test.nth_root(2);
    assert_eq!(roots.len(), 2);
    assert_eq!(roots[0].real().to_string(), "3");

    let ln_test = BigComplex::from_i64(1, 0);
    let ln_result = ln_test.ln_approx().unwrap();
    assert!(ln_result.is_zero());

    let exp_test = BigComplex::zero();
    let exp_result = exp_test.exp_approx();
    assert_eq!(exp_result.real().to_string(), "1");
    assert_eq!(exp_result.imag().to_string(), "0");
}

#[test]
fn test_large_number_advanced_operations() {
    // 测试大数的高级运算
    let large_num = BigInt::from_string("1234567890123456789").unwrap();

    // 测试大数的二进制操作
    let bit_len = large_num.bit_length();
    assert!(bit_len > 60); // 应该有很多位

    let ones_count = large_num.count_ones();
    assert!(ones_count > 0);

    // 测试大数的素数检测（这个数不是素数）
    assert!(!large_num.is_prime());

    // 测试大复数的运算
    let large_complex = BigComplex::new(
        BigInt::from_string("123456789").unwrap(),
        BigInt::from_string("987654321").unwrap(),
    );

    let magnitude = large_complex.magnitude();
    assert!(magnitude > BigInt::new(900000000)); // 应该是一个很大的数，约995340462

    let rotated = large_complex.rotate_180();
    assert_eq!(rotated.real().to_string(), "-123456789");
    assert_eq!(rotated.imag().to_string(), "-987654321");
}

#[test]
fn test_mathematical_properties() {
    // 测试数学性质

    // 测试阶乘的性质: n! = n * (n-1)!
    let n = BigInt::new(6);
    let n_factorial = n.factorial().unwrap();
    let n_minus_1 = BigInt::new(5);
    let n_minus_1_factorial = n_minus_1.factorial().unwrap();
    assert_eq!(n_factorial, n.clone() * n_minus_1_factorial);

    // 测试素数的性质
    let prime = BigInt::new(17);
    assert!(prime.is_prime());
    let next_prime_after_17 = prime.next_prime();
    assert_eq!(next_prime_after_17.to_string(), "19"); // 17的下一个素数是19

    // 测试复数旋转的性质: 旋转4次90度应该回到原点
    let z = BigComplex::from_i64(3, 4);
    let z_rotated_360 = z.rotate_90().rotate_90().rotate_90().rotate_90();
    assert_eq!(z, z_rotated_360);

    // 测试复数共轭的性质: (z*)* = z
    let z_conj_conj = z.conjugate().conjugate();
    assert_eq!(z, z_conj_conj);

    // 测试复数模长的性质: |z * z*| = |z|²
    let z_conj = z.conjugate();
    let product = &z * &z_conj;
    let magnitude_squared = z.magnitude_squared();
    assert_eq!(product.real(), &magnitude_squared);
    assert!(product.imag().is_zero());
}
