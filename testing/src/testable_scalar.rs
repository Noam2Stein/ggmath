use std::{array, fmt::Debug};

use ggmath::scalar::*;

pub trait TestableScalar: Scalar + PartialEq + Debug {
    const DEFAULT_VALUE: Self;
    const NORMAL_VALUES: &[Self];
    const SPECIAL_VALUES: &[Self];

    #[allow(dead_code)]
    fn n_normal_values<const N: usize>() -> [Self; N] {
        array::from_fn(Self::normal_value)
    }
    #[allow(dead_code)]
    fn n_special_values<const N: usize>() -> [Self; N] {
        array::from_fn(Self::special_value)
    }

    fn normal_value(index: usize) -> Self {
        *Self::NORMAL_VALUES
            .get(index)
            .unwrap_or(&Self::DEFAULT_VALUE)
    }
    fn special_value(index: usize) -> Self {
        Self::SPECIAL_VALUES.get(index).map_or_else(
            || Self::normal_value(index - Self::SPECIAL_VALUES.len()),
            |value| *value,
        )
    }
}

impl TestableScalar for f32 {
    const DEFAULT_VALUE: Self = 3.0;
    const NORMAL_VALUES: &[Self] = &[3.0, 5.81, 1000.1, 6941.4, 837.175];
    const SPECIAL_VALUES: &[Self] = &[
        0.0,
        1.0,
        Self::MIN_POSITIVE,
        Self::INFINITY,
        Self::NEG_INFINITY,
    ];
}
impl TestableScalar for f64 {
    const DEFAULT_VALUE: Self = 3.0;
    const NORMAL_VALUES: &[Self] = &[3.0, 5.81, 68034.168859, 6941.45499, 837.1754885];
    const SPECIAL_VALUES: &[Self] = &[
        0.0,
        1.0,
        Self::MIN_POSITIVE,
        Self::INFINITY,
        Self::NEG_INFINITY,
    ];
}

impl TestableScalar for u8 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 127, 67];
    const SPECIAL_VALUES: &[Self] = &[Self::MIN, Self::MAX];
}
impl TestableScalar for u16 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 1270, 67];
    const SPECIAL_VALUES: &[Self] = &[Self::MIN, Self::MAX];
}
impl TestableScalar for u32 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 12700, 67];
    const SPECIAL_VALUES: &[Self] = &[Self::MIN, Self::MAX];
}
impl TestableScalar for u64 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 127000, 67];
    const SPECIAL_VALUES: &[Self] = &[Self::MIN, Self::MAX];
}
impl TestableScalar for u128 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 1270000, 67];
    const SPECIAL_VALUES: &[Self] = &[Self::MIN, Self::MAX];
}
impl TestableScalar for usize {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 12700, 67];
    const SPECIAL_VALUES: &[Self] = &[Self::MIN, Self::MAX];
}

impl TestableScalar for i8 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 125, 67];
    const SPECIAL_VALUES: &[Self] = &[0, Self::MIN, Self::MAX];
}
impl TestableScalar for i16 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 1270, 67];
    const SPECIAL_VALUES: &[Self] = &[0, Self::MIN, Self::MAX];
}
impl TestableScalar for i32 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 12700, 67];
    const SPECIAL_VALUES: &[Self] = &[0, Self::MIN, Self::MAX];
}
impl TestableScalar for i64 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 127000, 67];
    const SPECIAL_VALUES: &[Self] = &[0, Self::MIN, Self::MAX];
}
impl TestableScalar for i128 {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 1270000, 67];
    const SPECIAL_VALUES: &[Self] = &[0, Self::MIN, Self::MAX];
}
impl TestableScalar for isize {
    const DEFAULT_VALUE: Self = 3;
    const NORMAL_VALUES: &[Self] = &[3, 7, 12700, 67];
    const SPECIAL_VALUES: &[Self] = &[0, Self::MIN, Self::MAX];
}

impl TestableScalar for bool {
    const DEFAULT_VALUE: Self = false;
    const NORMAL_VALUES: &[Self] = &[false, true];
    const SPECIAL_VALUES: &[Self] = &[];
}
