use gauss_int::GaussInt;
use num_traits::{One, Zero};

#[test]
fn test_arithmetic_chain() {
    let z1 = GaussInt::from_i64(1, 1);
    let z2 = GaussInt::from_i64(2, 3);
    let z3 = GaussInt::from_i64(4, -1);

    let z2z3 = &z2 * &z3; // (2+3i)(4-i) = 11+10i
    assert_eq!(z2z3, GaussInt::from_i64(11, 10));

    let result = &z1 + &z2z3; // (1+i) + (11+10i) = 12+11i
    assert_eq!(result, GaussInt::from_i64(12, 11));
}

#[test]
fn test_gcd_euclidean_property() {
    let a = GaussInt::from_i64(36, 48);
    let b = GaussInt::from_i64(12, 16);
    let g = a.gcd(&b);
    assert!(a.div_rem(&g).unwrap().1.is_zero());
    assert!(b.div_rem(&g).unwrap().1.is_zero());
}

#[test]
fn test_pow_large_exponent() {
    let z = GaussInt::from_i64(1, 1);
    assert_eq!(z.pow_u32(8), GaussInt::from_i64(16, 0)); // (1+i)^8 = 16
}

#[test]
fn test_field_properties() {
    let zero = GaussInt::zero();
    let one = GaussInt::one();
    let z = GaussInt::from_i64(3, 4);

    assert_eq!(&z + &zero, z);
    assert_eq!(&z * &one, z);
    assert_eq!(&z + &(-&z), zero);

    let z_conj = z.conjugate();
    let product = &z * &z_conj;
    assert!(product.is_real());
    assert_eq!(product.to_string(), "25");
}

#[test]
fn test_rotation_property() {
    // multiplying by i four times = identity
    let z = GaussInt::from_i64(3, 4);
    let i = GaussInt::from_i64(0, 1);
    assert_eq!(&z * &i * &i * &i * &i, z);
}

#[test]
fn test_div_rem_invariant() {
    let pairs = vec![
        (GaussInt::from_i64(100, 0), GaussInt::from_i64(7, 0)),
        (GaussInt::from_i64(0, 100), GaussInt::from_i64(0, 7)),
        (GaussInt::from_i64(-100, -100), GaussInt::from_i64(3, 4)),
        (GaussInt::from_i64(1, 1), GaussInt::from_i64(1, 1)),
    ];
    for (a, b) in pairs {
        let (q, r) = a.div_rem(&b).unwrap();
        assert!(
            r.norm() < b.norm(),
            "N(r)={} >= N(b)={}",
            r.norm(),
            b.norm()
        );
        assert_eq!(&q * &b + &r, a);
    }
}
