use std::{array::from_fn, fmt::Debug};

use crate::{
    scalar::*,
    vector::{ScalarCount, VecLen},
};

use super::TestEq;

pub trait TestableScalar: Scalar + Debug + TestEq {
    /// 4 sets of 4 scalar values provided to test the scalar.
    ///
    /// These 4 sets should try to follow these rules:
    /// * 1st. Normal cases
    /// * 2nd. Normal cases
    /// * 3rd. Some edge case values
    /// * 4th. All edge case values (0, MIN, MAX, NAN, INFINITY...)
    const VALUES: [[Self; 4]; 4];

    fn get_4_n<const N: usize>() -> [[Self; N]; 4]
    where
        ScalarCount<N>: VecLen,
    {
        [
            Self::VALUES[0][0..N].try_into().unwrap(),
            Self::VALUES[1][0..N].try_into().unwrap(),
            Self::VALUES[2][0..N].try_into().unwrap(),
            Self::VALUES[3][0..N].try_into().unwrap(),
        ]
    }

    fn get_4_c_r<const C: usize, const R: usize>() -> [[[Self; C]; R]; 4]
    where
        ScalarCount<C>: VecLen,
        ScalarCount<R>: VecLen,
    {
        let rows = Self::VALUES.map(|row| <[Self; C]>::try_from(&row[0..C]).unwrap());
        [
            from_fn(|i| rows[i % 4]),
            from_fn(|i| rows[(i + 1) % 4]),
            from_fn(|i| rows[(i + 2) % 4]),
            from_fn(|i| rows[(i + 3) % 4]),
        ]
    }
}

impl TestableScalar for f32 {
    const VALUES: [[Self; 4]; 4] = [
        [3.0, 5.81, 1000.1, 6941.4],
        [837.175, 0.0, 1.0, 54.7657575],
        [Self::NAN, 0.23954389574, Self::MIN_POSITIVE, 0.000000001],
        [Self::INFINITY, Self::NEG_INFINITY, Self::MIN, Self::MAX],
    ];
}

impl TestableScalar for f64 {
    const VALUES: [[Self; 4]; 4] = [
        [3.0, 5.81, 1000.1, 6941.4],
        [837.175, 0.0, 1.0, 54.7657575],
        [Self::NAN, 0.23954389574, Self::MIN_POSITIVE, 0.000000001],
        [Self::INFINITY, Self::NEG_INFINITY, Self::MIN, Self::MAX],
    ];
}

impl TestableScalar for u8 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5, 27, 58],
        [109, 0, 94, 57],
        [109, Self::MAX, 94, 57],
        [0, Self::MAX, 94, 57],
    ];
}
impl TestableScalar for u16 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5, 27, 58],
        [109, 0, 9423, 5723],
        [109, Self::MAX, 94, 57],
        [0, Self::MAX, 94, 57],
    ];
}
impl TestableScalar for u32 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 534, 27, 5856],
        [1109, 0, 9346423, 586723],
        [109, Self::MAX, 94, 57111111],
        [0, Self::MAX, 9444444, 57],
    ];
}
impl TestableScalar for u64 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 534, 27, 5856],
        [114509, 0, 934645546423, 5863723],
        [10459, Self::MAX, 94, 57111111],
        [0, Self::MAX, 9444444, 5734],
    ];
}
impl TestableScalar for u128 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 534, 27, 5856],
        [114509, 0, 9346473456345675546423, 586374534573723],
        [10459, Self::MAX, 345745734, 571134571111],
        [0, Self::MAX, 944473457444, 57574734],
    ];
}
impl TestableScalar for usize {
    const VALUES: [[Self; 4]; 4] = [
        [1, 534, 27, 5856],
        [1109, 0, 9346423, 586723],
        [109, Self::MAX, 94, 57111111],
        [0, Self::MAX, 9444444, 57],
    ];
}

impl TestableScalar for i8 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5, 27, -58],
        [0, Self::MIN, -94, 57],
        [-109, Self::MAX, 95, 0],
        [Self::MIN, Self::MAX, 0, Self::MAX - 1],
    ];
}
impl TestableScalar for i16 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5, 27, -58],
        [0, Self::MIN, -9344, 5647],
        [-10349, Self::MAX, 9345, 0],
        [Self::MIN, Self::MAX, 0, Self::MAX - 1],
    ];
}
impl TestableScalar for i32 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5345, 23457, -58],
        [0, Self::MIN, -9334544, 5643457],
        [-10334549, Self::MAX, 9534345, 0],
        [Self::MIN, Self::MAX, 0, Self::MAX - 1],
    ];
}
impl TestableScalar for i64 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5345, 23457, -58],
        [0, Self::MIN, -9356834544, 5645683457],
        [-10583334549, Self::MAX, 9538664345, 0],
        [Self::MIN, Self::MAX, 0, Self::MAX - 1],
    ];
}
impl TestableScalar for i128 {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5367567345, 23434673456757, -58],
        [0, Self::MIN, -935374534676834544, 56456873453457],
        [-10583325727534549, Self::MAX, 9532572578664345, 0],
        [Self::MIN, Self::MAX, 0, Self::MAX - 1],
    ];
}
impl TestableScalar for isize {
    const VALUES: [[Self; 4]; 4] = [
        [1, 5345, 23457, -58],
        [0, Self::MIN, -9334544, 5643457],
        [-10334549, Self::MAX, 9534345, 0],
        [Self::MIN, Self::MAX, 0, Self::MAX - 1],
    ];
}

impl TestableScalar for bool {
    const VALUES: [[Self; 4]; 4] = [
        [false, true, true, false],
        [false, true, true, true],
        [false, true, true, false],
        [false, true, true, true],
    ];
}
