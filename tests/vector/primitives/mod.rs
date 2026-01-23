use ggmath::{Aligned, Unaligned};

#[test]
fn f32_aligned() {
    f32::float_tests::<Aligned>();
}

#[test]
fn f32_unaligned() {
    f32::float_tests::<Unaligned>();
}

#[test]
fn f64_aligned() {
    f64::float_tests::<Aligned>();
}

#[test]
fn f64_unaligned() {
    f64::float_tests::<Unaligned>();
}

#[test]
fn i8_aligned() {
    i8::int_tests::<Aligned>();
}

#[test]
fn i8_unaligned() {
    i8::int_tests::<Unaligned>();
}

#[test]
fn i16_aligned() {
    i16::int_tests::<Aligned>();
}

#[test]
fn i16_unaligned() {
    i16::int_tests::<Unaligned>();
}

#[test]
fn i32_aligned() {
    i32::int_tests::<Aligned>();
}

#[test]
fn i32_unaligned() {
    i32::int_tests::<Unaligned>();
}

#[test]
fn i64_aligned() {
    i64::int_tests::<Aligned>();
}

#[test]
fn i64_unaligned() {
    i64::int_tests::<Unaligned>();
}

#[test]
fn i128_aligned() {
    i128::int_tests::<Aligned>();
}

#[test]
fn i128_unaligned() {
    i128::int_tests::<Unaligned>();
}

#[test]
fn isize_aligned() {
    isize::int_tests::<Aligned>();
}

#[test]
fn isize_unaligned() {
    isize::int_tests::<Unaligned>();
}

#[test]
fn u8_aligned() {
    u8::uint_tests::<Aligned>();
}

#[test]
fn u8_unaligned() {
    u8::uint_tests::<Unaligned>();
}

#[test]
fn u16_aligned() {
    u16::uint_tests::<Aligned>();
}

#[test]
fn u16_unaligned() {
    u16::uint_tests::<Unaligned>();
}

#[test]
fn u32_aligned() {
    u32::uint_tests::<Aligned>();
}

#[test]
fn u32_unaligned() {
    u32::uint_tests::<Unaligned>();
}

#[test]
fn u64_aligned() {
    u64::uint_tests::<Aligned>();
}

#[test]
fn u64_unaligned() {
    u64::uint_tests::<Unaligned>();
}

#[test]
fn u128_aligned() {
    u128::uint_tests::<Aligned>();
}

#[test]
fn u128_unaligned() {
    u128::uint_tests::<Unaligned>();
}

#[test]
fn usize_aligned() {
    usize::uint_tests::<Aligned>();
}

#[test]
fn usize_unaligned() {
    usize::uint_tests::<Unaligned>();
}

mod f32 {
    type T = f32;

    include!("float.rs");
}

mod f64 {
    type T = f64;

    include!("float.rs");
}

mod i8 {
    type T = i8;

    include!("int.rs");
}

mod i16 {
    type T = i16;

    include!("int.rs");
}

mod i32 {
    type T = i32;

    include!("int.rs");
}

mod i64 {
    type T = i64;

    include!("int.rs");
}

mod i128 {
    type T = i128;

    include!("int.rs");
}

mod isize {
    type T = isize;

    include!("int.rs");
}

mod u8 {
    type T = u8;

    include!("uint.rs");
}

mod u16 {
    type T = u16;

    include!("uint.rs");
}

mod u32 {
    type T = u32;

    include!("uint.rs");
}

mod u64 {
    type T = u64;

    include!("uint.rs");
}

mod u128 {
    type T = u128;

    include!("uint.rs");
}

mod usize {
    type T = usize;

    include!("uint.rs");
}
