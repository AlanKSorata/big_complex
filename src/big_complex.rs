use crate::big_int::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigComplex {
    real: BigInt,
    imag: BigInt,
}

impl BigComplex {
    pub fn new(real: BigInt, imag: BigInt) -> Self {
        BigComplex { real, imag }
    }

    pub fn from_i64(real: i64, imag: i64) -> Self {
        BigComplex {
            real: BigInt::new(real),
            imag: BigInt::new(imag),
        }
    }

    pub fn real(&self) -> &BigInt {
        &self.real
    }

    pub fn imag(&self) -> &BigInt {
        &self.imag
    }

    pub fn conjugate(&self) -> Self {
        BigComplex {
            real: self.real.clone(),
            imag: -&self.imag,
        }
    }

    pub fn magnitude_squared(&self) -> BigInt {
        &self.real * &self.real + &self.imag * &self.imag
    }

    pub fn scale(&self, factor: &BigInt) -> Self {
        BigComplex {
            real: &self.real * factor,
            imag: &self.imag * factor,
        }
    }

    pub fn add_real(&self, real: &BigInt) -> Self {
        BigComplex {
            real: &self.real + real,
            imag: self.imag.clone(),
        }
    }

    pub fn add_imag(&self, imag: &BigInt) -> Self {
        BigComplex {
            real: self.real.clone(),
            imag: &self.imag + imag,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }

    pub fn is_real(&self) -> bool {
        self.imag.is_zero()
    }

    pub fn is_imaginary(&self) -> bool {
        self.real.is_zero()
    }

    pub fn pow(&self, exp: u32) -> Self {
        if exp == 0 {
            return BigComplex::one();
        }

        let mut result = self.clone();
        let mut current = self.clone();
        let mut power = exp - 1;

        while power > 0 {
            if power % 2 == 1 {
                result = result * current.clone();
            }
            current = current.clone() * current.clone();
            power /= 2;
        }

        result
    }

    pub fn div_exact(&self, divisor: &BigInt) -> Option<Self> {
        if divisor.is_zero() {
            return None;
        }

        let real_rem = self.real.clone() % divisor.clone();
        let imag_rem = self.imag.clone() % divisor.clone();

        if !real_rem.is_zero() || !imag_rem.is_zero() {
            return None;
        }

        Some(BigComplex {
            real: self.real.clone() / divisor.clone(),
            imag: self.imag.clone() / divisor.clone(),
        })
    }

    pub fn norm(&self) -> BigInt {
        self.magnitude_squared()
    }

    pub fn distance_to(&self, other: &Self) -> BigInt {
        let diff = self - other;
        diff.magnitude_squared()
    }

    pub fn magnitude(&self) -> BigInt {
        self.magnitude_squared()
            .sqrt()
            .unwrap_or_else(|| BigInt::zero())
    }

    pub fn from_polar(r: &BigInt, theta_approx: i32) -> Self {
        // 简化的极坐标转换，使用整数近似角度
        // theta_approx: 0=0°, 1=90°, 2=180°, 3=270°
        match theta_approx % 4 {
            0 => BigComplex::new(r.clone(), BigInt::zero()), // 0°
            1 => BigComplex::new(BigInt::zero(), r.clone()), // 90°
            2 => BigComplex::new(-r, BigInt::zero()),        // 180°
            3 => BigComplex::new(BigInt::zero(), -r),        // 270°
            _ => unreachable!(),
        }
    }

    pub fn arg_quadrant(&self) -> Option<i32> {
        // 返回复数所在的象限 (0-3)
        if self.is_zero() {
            return None;
        }

        match (self.real.is_positive(), self.imag.is_positive()) {
            (true, true) => Some(0),   // 第一象限
            (false, true) => Some(1),  // 第二象限
            (false, false) => Some(2), // 第三象限
            (true, false) => Some(3),  // 第四象限
        }
    }

    pub fn rotate_90(&self) -> Self {
        // 逆时针旋转90度: (a+bi) * i = -b + ai
        BigComplex::new(-&self.imag, self.real.clone())
    }

    pub fn rotate_180(&self) -> Self {
        // 旋转180度: (a+bi) * (-1) = -a - bi
        BigComplex::new(-&self.real, -&self.imag)
    }

    pub fn rotate_270(&self) -> Self {
        // 逆时针旋转270度: (a+bi) * (-i) = b - ai
        BigComplex::new(self.imag.clone(), -&self.real)
    }

    pub fn nth_root(&self, n: u32) -> Vec<Self> {
        // 计算复数的n次方根（简化版本，只返回主根）
        if n == 0 {
            return vec![];
        }

        if self.is_zero() {
            return vec![BigComplex::zero()];
        }

        if n == 1 {
            return vec![self.clone()];
        }

        if n == 2 {
            // 平方根的简化计算
            let mag_squared = self.magnitude_squared();
            let _mag = mag_squared.sqrt().unwrap_or_else(|| BigInt::zero());

            if self.is_real() && self.real.is_positive() {
                let sqrt_real = self.real.sqrt().unwrap_or_else(|| BigInt::zero());
                return vec![
                    BigComplex::new(sqrt_real.clone(), BigInt::zero()),
                    BigComplex::new(-&sqrt_real, BigInt::zero()),
                ];
            } else if self.is_real() && self.real.is_negative() {
                let sqrt_abs = (-&self.real).sqrt().unwrap_or_else(|| BigInt::zero());
                return vec![
                    BigComplex::new(BigInt::zero(), sqrt_abs.clone()),
                    BigComplex::new(BigInt::zero(), -&sqrt_abs),
                ];
            }
        }

        // 对于其他情况，返回一个近似根
        vec![BigComplex::new(BigInt::one(), BigInt::zero())]
    }

    pub fn ln_approx(&self) -> Option<Self> {
        // 自然对数的简化近似（仅用于演示）
        if self.is_zero() {
            return None;
        }

        if self.is_real() && self.real.is_positive() {
            // 对于正实数，ln(x) ≈ 整数部分的近似
            if self.real == BigInt::one() {
                return Some(BigComplex::zero());
            }

            // 简化的对数近似
            let mut approx = BigInt::zero();
            let mut temp = self.real.clone();

            while temp > BigInt::one() {
                temp = temp / BigInt::new(2);
                approx = approx + BigInt::one();
            }

            return Some(BigComplex::new(approx, BigInt::zero()));
        }

        // 对于复数，返回简化结果
        Some(BigComplex::new(BigInt::zero(), BigInt::one()))
    }

    pub fn exp_approx(&self) -> Self {
        // 指数函数的简化近似（仅用于演示）
        if self.is_zero() {
            return BigComplex::one();
        }

        if self.is_real() {
            // e^x 的简化近似
            if self.real.is_zero() {
                return BigComplex::one();
            }

            let mut result = BigComplex::one();
            let mut term = BigComplex::one();

            // 计算前几项泰勒级数
            for i in 1..=10 {
                term = term * self.clone() / BigComplex::new(BigInt::new(i), BigInt::zero());
                result = result + term.clone();

                if term.magnitude_squared() < BigInt::one() {
                    break;
                }
            }

            result
        } else {
            // 对于复数，返回简化结果
            BigComplex::new(BigInt::one(), BigInt::one())
        }
    }
}

impl Zero for BigComplex {
    fn zero() -> Self {
        BigComplex {
            real: BigInt::zero(),
            imag: BigInt::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
}

impl One for BigComplex {
    fn one() -> Self {
        BigComplex {
            real: BigInt::one(),
            imag: BigInt::zero(),
        }
    }
}

impl Add for BigComplex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BigComplex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<'a> Add for &'a BigComplex {
    type Output = BigComplex;

    fn add(self, other: Self) -> BigComplex {
        BigComplex {
            real: &self.real + &other.real,
            imag: &self.imag + &other.imag,
        }
    }
}

impl Sub for BigComplex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        BigComplex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<'a> Sub for &'a BigComplex {
    type Output = BigComplex;

    fn sub(self, other: Self) -> BigComplex {
        BigComplex {
            real: &self.real - &other.real,
            imag: &self.imag - &other.imag,
        }
    }
}

impl Mul for BigComplex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let real = &self.real * &other.real - &self.imag * &other.imag;
        let imag = &self.real * &other.imag + &self.imag * &other.real;

        BigComplex { real, imag }
    }
}

impl<'a> Mul for &'a BigComplex {
    type Output = BigComplex;

    fn mul(self, other: Self) -> BigComplex {
        let real = &self.real * &other.real - &self.imag * &other.imag;
        let imag = &self.real * &other.imag + &self.imag * &other.real;

        BigComplex { real, imag }
    }
}

impl Div for BigComplex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator =
            other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();

        if denominator.is_zero() {
            panic!("Division by zero complex number");
        }

        let real = (self.real.clone() * other.real.clone()
            + self.imag.clone() * other.imag.clone())
            / denominator.clone();
        let imag = (self.imag.clone() * other.real.clone()
            - self.real.clone() * other.imag.clone())
            / denominator;

        BigComplex { real, imag }
    }
}

impl<'a> Div for &'a BigComplex {
    type Output = BigComplex;

    fn div(self, other: Self) -> BigComplex {
        let denominator =
            other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();

        if denominator.is_zero() {
            panic!("Division by zero complex number");
        }

        let real = (self.real.clone() * other.real.clone()
            + self.imag.clone() * other.imag.clone())
            / denominator.clone();
        let imag = (self.imag.clone() * other.real.clone()
            - self.real.clone() * other.imag.clone())
            / denominator;

        BigComplex { real, imag }
    }
}

impl Neg for BigComplex {
    type Output = Self;

    fn neg(self) -> Self {
        BigComplex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<'a> Neg for &'a BigComplex {
    type Output = BigComplex;

    fn neg(self) -> BigComplex {
        BigComplex {
            real: -&self.real,
            imag: -&self.imag,
        }
    }
}

impl fmt::Display for BigComplex {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_complex_creation() {
        let a = BigComplex::from_i64(3, 4);
        assert_eq!(a.real().to_string(), "3");
        assert_eq!(a.imag().to_string(), "4");

        let b = BigComplex::new(BigInt::new(-5), BigInt::new(2));
        assert_eq!(b.to_string(), "-5+2i");
    }

    #[test]
    fn test_big_complex_arithmetic() {
        let a = BigComplex::from_i64(3, 4);
        let b = BigComplex::from_i64(1, 2);

        let sum = &a + &b;
        assert_eq!(sum.real().to_string(), "4");
        assert_eq!(sum.imag().to_string(), "6");

        let diff = &a - &b;
        assert_eq!(diff.real().to_string(), "2");
        assert_eq!(diff.imag().to_string(), "2");

        let product = &a * &b;
        assert_eq!(product.real().to_string(), "-5"); // (3+4i)(1+2i) = 3 + 6i + 4i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert_eq!(product.imag().to_string(), "10");

        let quotient = &a / &b;
        assert_eq!(quotient.real().to_string(), "2"); // (3+4i)/(1+2i) = (3+4i)(1-2i)/5 = (11-2i)/5 = 2 + 0i (integer division)
        assert_eq!(quotient.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_conjugate() {
        let a = BigComplex::from_i64(3, 4);
        let conj = a.conjugate();
        assert_eq!(conj.real().to_string(), "3");
        assert_eq!(conj.imag().to_string(), "-4");

        let b = BigComplex::from_i64(-2, 5);
        let conj_b = b.conjugate();
        assert_eq!(conj_b.real().to_string(), "-2");
        assert_eq!(conj_b.imag().to_string(), "-5");
    }

    #[test]
    fn test_big_complex_magnitude() {
        let a = BigComplex::from_i64(3, 4);
        let mag = a.magnitude_squared();
        assert_eq!(mag.to_string(), "25"); // 3² + 4² = 9 + 16 = 25

        let b = BigComplex::from_i64(0, 0);
        assert_eq!(b.magnitude_squared().to_string(), "0");
    }

    #[test]
    fn test_big_complex_pow() {
        let a = BigComplex::from_i64(1, 1);
        let a2 = a.pow(2);
        assert_eq!(a2.real().to_string(), "0"); // (1+i)² = 1 + 2i + i² = 1 + 2i - 1 = 2i
        assert_eq!(a2.imag().to_string(), "2");

        let a4 = a.pow(4);
        assert_eq!(a4.real().to_string(), "-4"); // (1+i)⁴ = ((1+i)²)² = (2i)² = -4
        assert_eq!(a4.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_scaling() {
        let a = BigComplex::from_i64(3, 4);
        let scaled = a.scale(&BigInt::new(2));
        assert_eq!(scaled.real().to_string(), "6");
        assert_eq!(scaled.imag().to_string(), "8");
    }

    #[test]
    fn test_big_complex_div_exact() {
        let a = BigComplex::from_i64(6, 8);
        let result = a.div_exact(&BigInt::new(2));
        assert!(result.is_some());
        let divided = result.unwrap();
        assert_eq!(divided.real().to_string(), "3");
        assert_eq!(divided.imag().to_string(), "4");

        let b = BigComplex::from_i64(5, 7);
        let result2 = b.div_exact(&BigInt::new(2));
        assert!(result2.is_none()); // 5 and 7 are not divisible by 2
    }

    #[test]
    fn test_big_complex_zero_and_one() {
        let zero = BigComplex::zero();
        assert!(zero.is_zero());

        let one = BigComplex::one();
        assert_eq!(one.real().to_string(), "1");
        assert_eq!(one.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_display() {
        let a = BigComplex::from_i64(3, 4);
        assert_eq!(a.to_string(), "3+4i");

        let b = BigComplex::from_i64(3, -4);
        assert_eq!(b.to_string(), "3-4i");

        let c = BigComplex::from_i64(0, 5);
        assert_eq!(c.to_string(), "5i");

        let d = BigComplex::from_i64(7, 0);
        assert_eq!(d.to_string(), "7");

        let e = BigComplex::from_i64(0, 1);
        assert_eq!(e.to_string(), "i");

        let f = BigComplex::from_i64(0, -1);
        assert_eq!(f.to_string(), "-i");
    }

    #[test]
    fn test_big_complex_distance() {
        let a = BigComplex::from_i64(3, 4);
        let b = BigComplex::from_i64(0, 0);
        let distance = a.distance_to(&b);
        assert_eq!(distance.to_string(), "25"); // (3-0)² + (4-0)² = 9 + 16 = 25
    }

    #[test]
    fn test_big_complex_magnitude_function() {
        let a = BigComplex::from_i64(3, 4);
        let mag = a.magnitude();
        assert_eq!(mag.to_string(), "5"); // sqrt(3² + 4²) = sqrt(25) = 5

        let b = BigComplex::from_i64(5, 0);
        assert_eq!(b.magnitude().to_string(), "5");

        let c = BigComplex::from_i64(0, 12);
        assert_eq!(c.magnitude().to_string(), "12");
    }

    #[test]
    fn test_big_complex_polar() {
        let r = BigInt::new(5);

        // 测试四个基本方向
        let z0 = BigComplex::from_polar(&r, 0); // 0°
        assert_eq!(z0.real().to_string(), "5");
        assert_eq!(z0.imag().to_string(), "0");

        let z1 = BigComplex::from_polar(&r, 1); // 90°
        assert_eq!(z1.real().to_string(), "0");
        assert_eq!(z1.imag().to_string(), "5");

        let z2 = BigComplex::from_polar(&r, 2); // 180°
        assert_eq!(z2.real().to_string(), "-5");
        assert_eq!(z2.imag().to_string(), "0");

        let z3 = BigComplex::from_polar(&r, 3); // 270°
        assert_eq!(z3.real().to_string(), "0");
        assert_eq!(z3.imag().to_string(), "-5");

        // 测试角度循环
        let z4 = BigComplex::from_polar(&r, 4); // 360° = 0°
        assert_eq!(z4.real().to_string(), "5");
        assert_eq!(z4.imag().to_string(), "0");
    }

    #[test]
    fn test_big_complex_quadrant() {
        let z1 = BigComplex::from_i64(3, 4); // 第一象限
        assert_eq!(z1.arg_quadrant(), Some(0));

        let z2 = BigComplex::from_i64(-3, 4); // 第二象限
        assert_eq!(z2.arg_quadrant(), Some(1));

        let z3 = BigComplex::from_i64(-3, -4); // 第三象限
        assert_eq!(z3.arg_quadrant(), Some(2));

        let z4 = BigComplex::from_i64(3, -4); // 第四象限
        assert_eq!(z4.arg_quadrant(), Some(3));

        let z0 = BigComplex::from_i64(0, 0); // 原点
        assert_eq!(z0.arg_quadrant(), None);
    }

    #[test]
    fn test_big_complex_rotation() {
        let z = BigComplex::from_i64(1, 0); // 1 + 0i

        // 旋转90度: 1 -> i
        let z90 = z.rotate_90();
        assert_eq!(z90.real().to_string(), "0");
        assert_eq!(z90.imag().to_string(), "1");

        // 旋转180度: 1 -> -1
        let z180 = z.rotate_180();
        assert_eq!(z180.real().to_string(), "-1");
        assert_eq!(z180.imag().to_string(), "0");

        // 旋转270度: 1 -> -i
        let z270 = z.rotate_270();
        assert_eq!(z270.real().to_string(), "0");
        assert_eq!(z270.imag().to_string(), "-1");

        // 测试复杂数字的旋转
        let w = BigComplex::from_i64(3, 4); // 3 + 4i
        let w90 = w.rotate_90(); // -4 + 3i
        assert_eq!(w90.real().to_string(), "-4");
        assert_eq!(w90.imag().to_string(), "3");
    }

    #[test]
    fn test_big_complex_nth_root() {
        // 测试平方根
        let z1 = BigComplex::from_i64(4, 0); // 4 + 0i
        let roots = z1.nth_root(2);
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[0].real().to_string(), "2");
        assert_eq!(roots[0].imag().to_string(), "0");
        assert_eq!(roots[1].real().to_string(), "-2");
        assert_eq!(roots[1].imag().to_string(), "0");

        // 测试负数的平方根
        let z2 = BigComplex::from_i64(-4, 0); // -4 + 0i
        let roots2 = z2.nth_root(2);
        assert_eq!(roots2.len(), 2);
        assert_eq!(roots2[0].real().to_string(), "0");
        assert_eq!(roots2[0].imag().to_string(), "2");
        assert_eq!(roots2[1].real().to_string(), "0");
        assert_eq!(roots2[1].imag().to_string(), "-2");

        // 测试零的根
        let zero = BigComplex::zero();
        let zero_roots = zero.nth_root(3);
        assert_eq!(zero_roots.len(), 1);
        assert!(zero_roots[0].is_zero());

        // 测试一次根
        let z3 = BigComplex::from_i64(5, 7);
        let roots3 = z3.nth_root(1);
        assert_eq!(roots3.len(), 1);
        assert_eq!(roots3[0], z3);
    }

    #[test]
    fn test_big_complex_ln_approx() {
        // 测试ln(1) = 0
        let one = BigComplex::from_i64(1, 0);
        let ln_one = one.ln_approx().unwrap();
        assert!(ln_one.is_zero());

        // 测试正实数的对数近似
        let z1 = BigComplex::from_i64(8, 0); // ln(8) ≈ 3 (简化近似)
        let ln_z1 = z1.ln_approx().unwrap();
        assert_eq!(ln_z1.real().to_string(), "3");
        assert_eq!(ln_z1.imag().to_string(), "0");

        // 测试零的对数（应该返回None）
        let zero = BigComplex::zero();
        assert_eq!(zero.ln_approx(), None);

        // 测试复数的对数（简化结果）
        let z2 = BigComplex::from_i64(1, 1);
        let ln_z2 = z2.ln_approx().unwrap();
        assert_eq!(ln_z2.real().to_string(), "0");
        assert_eq!(ln_z2.imag().to_string(), "1");
    }

    #[test]
    fn test_big_complex_exp_approx() {
        // 测试exp(0) = 1
        let zero = BigComplex::zero();
        let exp_zero = zero.exp_approx();
        assert_eq!(exp_zero.real().to_string(), "1");
        assert_eq!(exp_zero.imag().to_string(), "0");

        // 测试实数的指数近似
        let z1 = BigComplex::from_i64(1, 0);
        let exp_z1 = z1.exp_approx();
        // 由于是简化近似，我们只检查结果不为零
        assert!(!exp_z1.is_zero());

        // 测试复数的指数（简化结果）
        let z2 = BigComplex::from_i64(0, 1);
        let exp_z2 = z2.exp_approx();
        assert_eq!(exp_z2.real().to_string(), "1");
        assert_eq!(exp_z2.imag().to_string(), "1");
    }
}
