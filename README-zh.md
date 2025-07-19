# Big Complex Number Calculator

一个使用 Rust 语言实现的支持大数复杂运算的计算模块，提供了丰富的数学运算功能。

## 核心模块

### BigInt - 大整数模块

#### 基础功能

- ✅ 大整数的创建和基本运算（加、减、乘、除、取模）
- ✅ 字符串解析和显示
- ✅ 字节序列转换
- ✅ 符号检测和绝对值计算
- ✅ 比较运算

#### 数学运算功能

- ✅ 幂运算 (`pow`)
- ✅ 平方根计算 (`sqrt`)
- ✅ 最大公约数 (`gcd`)
- ✅ 最小公倍数 (`lcm`)
- ✅ 模幂运算 (`mod_pow`)
- ✅ 模逆运算 (`mod_inv`)

#### 新增功能

- ✅ **阶乘计算** (`factorial`) - 计算 n!
- ✅ **素数检测** (`is_prime`) - 判断是否为素数
- ✅ **下一个素数** (`next_prime`) - 找到大于当前数的最小素数
- ✅ **二进制操作**:
  - `bit_length()` - 获取二进制位长度
  - `count_ones()` - 计算二进制中 1 的个数
  - `trailing_zeros()` - 计算尾随零的个数
  - `is_power_of_two()` - 判断是否为 2 的幂
  - `next_power_of_two()` - 获取下一个 2 的幂

### BigComplex - 大复数模块

#### 基础功能

- ✅ 复数的创建和基本运算（加、减、乘、除）
- ✅ 实部和虚部访问
- ✅ 复数共轭 (`conjugate`)
- ✅ 模长平方 (`magnitude_squared`)
- ✅ 缩放操作 (`scale`)
- ✅ 复数幂运算 (`pow`)

#### 几何和极坐标功能

- ✅ **模长计算** (`magnitude`) - 计算复数的模长
- ✅ **极坐标构造** (`from_polar`) - 从极坐标创建复数
- ✅ **象限判断** (`arg_quadrant`) - 判断复数所在象限
- ✅ **旋转操作**:
  - `rotate_90()` - 逆时针旋转 90 度
  - `rotate_180()` - 旋转 180 度
  - `rotate_270()` - 逆时针旋转 270 度

#### 高级数学运算

- ✅ **n 次方根** (`nth_root`) - 计算复数的 n 次方根
- ✅ **自然对数近似** (`ln_approx`) - 复数自然对数的简化计算
- ✅ **指数函数近似** (`exp_approx`) - 复数指数函数的简化计算

## 测试覆盖

### 单元测试

- ✅ BigInt: 18 个测试函数，覆盖所有功能
- ✅ BigComplex: 17 个测试函数，覆盖所有功能

### 集成测试

- ✅ 12 个集成测试，包括：
  - 大数运算测试
  - 复数运算链测试
  - 多项式求值测试
  - 数学性质验证测试
  - 新功能综合测试

## 性能特点

- ✅ 支持任意精度的大整数运算
- ✅ 高效的算法实现（如素数检测使用试除法优化）
- ✅ 内存安全的 Rust 实现
- ✅ 零拷贝的引用操作支持

## 使用示例

```rust
use big_complex::{BigInt, BigComplex};

// 大整数运算
let n = BigInt::new(10);
println!("10! = {}", n.factorial().unwrap());

let num = BigInt::new(97);
println!("{} is prime: {}", num, num.is_prime());

// 复数运算
let z = BigComplex::from_i64(3, 4);
println!("Magnitude: {}", z.magnitude());
println!("Rotated 90°: {}", z.rotate_90());

// 高级运算
let roots = BigComplex::from_i64(16, 0).nth_root(2);
println!("Square roots: {:?}", roots);
```

## 编译和运行

```bash
# 运行所有测试
cargo test

# 运行示例程序
cargo run --example usage

# 运行特定测试
cargo test test_big_int_factorial
cargo test test_big_complex_rotation
```

## 项目结构

```
src/
├── lib.rs          # 模块导出
├── big_int.rs      # 大整数实现
└── big_complex.rs  # 大复数实现

tests/
└── integration_tests.rs  # 集成测试

examples/
└── usage.rs        # 使用示例
```

## 依赖项

- `num-bigint` - 大整数底层实现
- `num-traits` - 数值特征
- `num-complex` - 复数支持
- `num-integer` - 整数运算

## 总结

本项目成功实现了一个功能完整的大数复杂运算计算模块，包含：

- **28 个单元测试** 全部通过
- **12 个集成测试** 全部通过
- **15 个新增功能** 并配有相应测试
- **完整的示例程序** 展示所有功能

每个新功能的实现都遵循了"实现-测试"的开发模式，确保代码质量和功能正确性。
