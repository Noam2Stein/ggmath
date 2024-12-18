use std::{array, fmt::Debug};

use ggmath::scalar::*;

pub trait TestableScalar: Scalar + PartialEq + Debug {
    const VALUES: [Self; 16];

    #[allow(dead_code)]
    fn n_values<const N: usize>(offset: usize) -> [Self; N] {
        array::from_fn(|index| Self::value(index + offset))
    }
    fn value(index: usize) -> Self {
        *Self::VALUES.get(index).unwrap_or(&Self::VALUES[0])
    }
}

impl TestableScalar for f32 {
    const VALUES: [Self; 16] = [
        3.0,
        5.81,
        1000.1,
        6941.4,
        837.175,
        0.0,
        1.0,
        54.7657575,
        0.23954389574,
        0.000000001,
        Self::MIN_POSITIVE,
        Self::INFINITY,
        Self::NEG_INFINITY,
        Self::MIN,
        Self::MAX,
        Self::NAN,
    ];
}
impl TestableScalar for f64 {
    const VALUES: [Self; 16] = [
        3.0,
        5.81,
        1000.1,
        6941.4,
        837.175,
        0.0,
        1.0,
        54.7657575,
        0.23954389574,
        0.000000001,
        Self::MIN_POSITIVE,
        Self::INFINITY,
        Self::NEG_INFINITY,
        Self::MIN,
        Self::MAX,
        Self::NAN,
    ];
}

impl TestableScalar for u8 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        5,
        7,
        13,
        64,
        23,
        53,
        65,
        14,
        78,
        65,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for u16 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        512,
        7,
        13,
        64,
        2312,
        53,
        6536,
        14,
        7846,
        65,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for u32 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        512346,
        7,
        13,
        64,
        2312346,
        53,
        6536,
        143333,
        7846,
        65,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for u64 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        58585885858588585,
        7,
        13,
        64,
        377373737373737377,
        53,
        868866668868886886,
        143333,
        7846,
        65,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for u128 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        512346585855585855855858558585,
        7,
        84884884884884884884884488848488484884,
        64,
        2312346188181818181818818,
        53,
        6536484888448884884884,
        143333,
        7846,
        65,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for usize {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        512346,
        7,
        13,
        64,
        2312346,
        53,
        6536,
        143333,
        7846,
        65,
        Self::MIN,
        Self::MAX,
    ];
}

impl TestableScalar for i8 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        -5,
        7,
        13,
        -64,
        23,
        53,
        -65,
        14,
        -78,
        -65,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for i16 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        -5,
        7,
        13,
        -64,
        23,
        5346,
        -65,
        1445,
        -78,
        -23511,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for i32 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2,
        3,
        -5,
        7,
        13,
        -6434577,
        23,
        535458734,
        -65,
        14453457,
        -78,
        -6523511,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for i64 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2235,
        3,
        -53463,
        7,
        13,
        -6434577457677456,
        23,
        535436587343475,
        -65,
        14434553457,
        -78,
        -6523589711,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for i128 {
    const VALUES: [Self; 16] = [
        0,
        1,
        2235,
        33465666666666,
        -53463,
        7,
        13,
        -6434577457677456,
        24443,
        53543658475,
        -65,
        14434553457,
        -78,
        -6523589711,
        Self::MIN,
        Self::MAX,
    ];
}
impl TestableScalar for isize {
    const VALUES: [Self; 16] = [
        0,
        1,
        2235,
        33465666666666,
        -53463,
        7,
        13,
        -6434577457677456,
        24443,
        53543658475,
        -65,
        14434553457,
        -78,
        -6523589711,
        Self::MIN,
        Self::MAX,
    ];
}

impl TestableScalar for bool {
    const VALUES: [Self; 16] = [
        true, false, true, true, false, true, false, true, true, true, true, false, false, true,
        false, true,
    ];
}
