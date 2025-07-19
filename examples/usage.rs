use big_complex::{BigComplex, BigInt};
use num_traits::Zero;

fn main() {
    println!("=== Big Complex Number Calculator Demo ===\n");

    println!("1. Creating Big Integers:");
    let a = BigInt::from_string("123456789012345678901234567890").unwrap();
    let b = BigInt::new(987654321);
    println!("a = {}", a);
    println!("b = {}", b);

    println!("\n2. Big Integer Operations:");
    println!("a + b = {}", &a + &b);
    println!("a * b = {}", &a * &b);
    println!("a mod b = {}", &a % &b);
    println!("gcd(a, b) = {}", a.gcd(&b));
    println!("lcm(a, b) = {}", a.lcm(&b));

    println!("\n3. Creating Complex Numbers:");
    let z1 = BigComplex::from_i64(3, 4);
    let z2 = BigComplex::new(
        BigInt::from_string("123456789").unwrap(),
        BigInt::from_string("987654321").unwrap(),
    );
    println!("z1 = {}", z1);
    println!("z2 = {}", z2);

    println!("\n4. Complex Arithmetic:");
    println!("z1 + z2 = {}", &z1 + &z2);
    println!("z1 * z2 = {}", &z1 * &z2);
    println!("z1.conjugate() = {}", z1.conjugate());
    println!("|z1|² = {}", z1.magnitude_squared());

    println!("\n5. Complex Powers:");
    let z = BigComplex::from_i64(1, 1);
    for i in 1..=4 {
        let power = z.pow(i);
        println!("(1+i)^{} = {}", i, power);
    }

    println!("\n6. Solving Quadratic Equations:");
    let a = BigComplex::from_i64(1, 0);
    let b = BigComplex::from_i64(-3, 0);
    let c = BigComplex::from_i64(2, 0);

    let discriminant = &b * &b - BigComplex::from_i64(4, 0) * a.clone() * c.clone();
    println!("Discriminant: {}", discriminant);

    let sqrt_disc = if discriminant.is_real() {
        let real_sqrt = discriminant.real().sqrt().unwrap_or_else(|| BigInt::zero());
        BigComplex::new(real_sqrt, BigInt::zero())
    } else {
        BigComplex::zero()
    };

    let root1 = (-&b + sqrt_disc.clone()) / (BigComplex::from_i64(2, 0) * a.clone());
    let root2 = (-&b - sqrt_disc) / (BigComplex::from_i64(2, 0) * a);

    println!("Roots: {} and {}", root1, root2);

    println!("\n7. Geometric Operations:");
    let point1 = BigComplex::from_i64(3, 4);
    let point2 = BigComplex::from_i64(6, 8);
    println!(
        "Distance between {} and {}: {}",
        point1,
        point2,
        point1
            .distance_to(&point2)
            .sqrt()
            .unwrap_or_else(|| BigInt::zero())
    );

    println!("\n8. Modular Arithmetic:");
    let base = BigInt::from_string("123456789").unwrap();
    let exp = BigInt::new(100);
    let modulus = BigInt::new(1000000007);
    let result = base.mod_pow(&exp, &modulus);
    println!("{}^{} mod {} = {}", base, exp, modulus, result);

    println!("\n9. New BigInt Features:");

    // 阶乘
    let n = BigInt::new(10);
    println!("{}! = {}", n, n.factorial().unwrap());

    // 素数检测
    let num = BigInt::new(97);
    println!("{} is prime: {}", num, num.is_prime());
    println!("Next prime after {}: {}", num, num.next_prime());

    // 二进制操作
    let binary_num = BigInt::new(255);
    println!(
        "{} in binary has {} bits",
        binary_num,
        binary_num.bit_length()
    );
    println!(
        "{} has {} ones in binary",
        binary_num,
        binary_num.count_ones()
    );
    println!(
        "{} is power of two: {}",
        binary_num,
        binary_num.is_power_of_two()
    );
    println!(
        "Next power of two after {}: {}",
        binary_num,
        binary_num.next_power_of_two()
    );

    println!("\n10. New BigComplex Features:");

    // 极坐标和旋转
    let complex_num = BigComplex::from_i64(3, 4);
    println!("Complex number: {}", complex_num);
    println!("Magnitude: {}", complex_num.magnitude());
    println!("Quadrant: {:?}", complex_num.arg_quadrant());

    // 旋转操作
    println!("Rotated 90°: {}", complex_num.rotate_90());
    println!("Rotated 180°: {}", complex_num.rotate_180());
    println!("Rotated 270°: {}", complex_num.rotate_270());

    // 极坐标构造
    let polar_complex = BigComplex::from_polar(&BigInt::new(5), 1);
    println!("From polar (r=5, θ=90°): {}", polar_complex);

    // 高级数学运算
    let square_test = BigComplex::from_i64(16, 0);
    let roots = square_test.nth_root(2);
    println!(
        "Square roots of {}: {:?}",
        square_test,
        roots.iter().map(|r| r.to_string()).collect::<Vec<_>>()
    );

    let ln_test = BigComplex::from_i64(8, 0);
    if let Some(ln_result) = ln_test.ln_approx() {
        println!("ln({}) ≈ {}", ln_test, ln_result);
    }

    let exp_test = BigComplex::from_i64(2, 0);
    println!("exp({}) ≈ {}", exp_test, exp_test.exp_approx());

    println!("\n11. Performance Test with Large Numbers:");
    let very_large =
        BigInt::from_string("12345678901234567890123456789012345678901234567890").unwrap();
    println!("Very large number: {}", very_large);
    println!("Bit length: {}", very_large.bit_length());
    println!("Is prime: {}", very_large.is_prime());

    let large_complex = BigComplex::new(
        BigInt::from_string("123456789012345").unwrap(),
        BigInt::from_string("987654321098765").unwrap(),
    );
    println!("Large complex: {}", large_complex);
    println!("Large complex magnitude: {}", large_complex.magnitude());
}
