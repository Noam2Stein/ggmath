extern crate std;

use std::{
    panic::{UnwindSafe, catch_unwind, resume_unwind},
    println,
};

// The implementation of this macro is quite unmaintainable, but it is the only
// way to support all parameter combinations without using macro recursion,
// which destroys compile times.

/// Calls the given expression multiple times based on the given type
/// parameters.
///
/// Accepted type parameters:
///
/// - `N`
/// - `T: PrimitiveNumber`
/// - `T: PrimitiveFloat`
/// - `T: PrimitiveInteger`
/// - `T: PrimitiveSigned`
/// - `T: PrimitiveUnsigned`
/// - `Wide`
/// - `Wide: WideFloat`
/// - `Wide: WideInteger`
/// - `Wide: WideSigned`
/// - `Wide: WideUnsigned`
/// - `A`
///
/// # Examples
///
/// ```
/// for_types!(|T: PrimitiveFloat| {
///     assert_eq!((1.0 as T) + (1.0 as T), (2.0 as T));
/// });
/// ```
macro_rules! for_types {
    (|N| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            crate::utils::call_with_panic_message(|| $expr, Some(N), None, None, None);
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|T: PrimitiveNumber| $expr:expr) => {{
        fn for_t<T>(t: &'static str)
        where
            T: crate::utils::Number,
        {
            crate::utils::call_with_panic_message(|| $expr, None, Some(t), None, None);
        }

        for_t::<f32>("f32");
        for_t::<f64>("f64");
        for_t::<i8>("i8");
        for_t::<i16>("i16");
        for_t::<i32>("i32");
        for_t::<i64>("i64");
        for_t::<i128>("i128");
        for_t::<isize>("isize");
        for_t::<u8>("u8");
        for_t::<u16>("u16");
        for_t::<u32>("u32");
        for_t::<u64>("u64");
        for_t::<u128>("u128");
        for_t::<usize>("usize");
    };};
    (|T: PrimitiveFloat| $expr:expr) => {{
        type T = f32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), None, None);
    };
    {
        type T = f64;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), None, None);
    };};
    (|T: PrimitiveInteger| $expr:expr) => {{
        type T = i8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), None, None);
    };
    {
        type T = i16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), None, None);
    };
    {
        type T = i32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), None, None);
    };
    {
        type T = i64;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), None, None);
    };
    {
        type T = i128;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i128"), None, None);
    };
    {
        type T = isize;
        crate::utils::call_with_panic_message(|| $expr, None, Some("isize"), None, None);
    };
    {
        type T = u8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), None, None);
    };
    {
        type T = u16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), None, None);
    };
    {
        type T = u32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), None, None);
    };
    {
        type T = u64;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), None, None);
    };
    {
        type T = u128;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u128"), None, None);
    };
    {
        type T = usize;
        crate::utils::call_with_panic_message(|| $expr, None, Some("usize"), None, None);
    };};
    (|T: PrimitiveSigned| $expr:expr) => {{
        type T = i8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), None, None);
    };
    {
        type T = i16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), None, None);
    };
    {
        type T = i32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), None, None);
    };
    {
        type T = i64;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), None, None);
    };
    {
        type T = i128;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i128"), None, None);
    };
    {
        type T = isize;
        crate::utils::call_with_panic_message(|| $expr, None, Some("isize"), None, None);
    };};
    (|T: PrimitiveUnsigned| $expr:expr) => {{
        type T = u8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), None, None);
    };
    {
        type T = u16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), None, None);
    };
    {
        type T = u32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), None, None);
    };
    {
        type T = u64;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), None, None);
    };
    {
        type T = u128;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u128"), None, None);
    };
    {
        type T = usize;
        crate::utils::call_with_panic_message(|| $expr, None, Some("usize"), None, None);
    };};
    (|Wide| $expr:expr) => {{
        type Wide = wide::f32x4;
        #[allow(dead_code)]
        type T = f32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), Some(LANES), None);
    };
    {
        type Wide = wide::f32x8;
        #[allow(dead_code)]
        type T = f32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), Some(LANES), None);
    };
    {
        type Wide = wide::f32x16;
        #[allow(dead_code)]
        type T = f32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), Some(LANES), None);
    };
    {
        type Wide = wide::f64x2;
        #[allow(dead_code)]
        type T = f64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), Some(LANES), None);
    };
    {
        type Wide = wide::f64x4;
        #[allow(dead_code)]
        type T = f64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), Some(LANES), None);
    };
    {
        type Wide = wide::f64x8;
        #[allow(dead_code)]
        type T = f64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), Some(LANES), None);
    };
    {
        type Wide = wide::i8x16;
        #[allow(dead_code)]
        type T = i8;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), Some(LANES), None);
    };
    {
        type Wide = wide::i8x32;
        #[allow(dead_code)]
        type T = i8;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x8;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x16;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x32;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x4;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x8;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x16;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x2;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x4;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x8;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::u8x16;
        #[allow(dead_code)]
        type T = u8;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), Some(LANES), None);
    };
    {
        type Wide = wide::u8x32;
        #[allow(dead_code)]
        type T = u8;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x8;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x16;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x32;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x4;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x8;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x16;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x2;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x4;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x8;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };};
    (|Wide: WideFloat| $expr:expr) => {{
        type Wide = wide::f32x4;
        #[allow(dead_code)]
        type T = f32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), Some(LANES), None);
    };
    {
        type Wide = wide::f32x8;
        #[allow(dead_code)]
        type T = f32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), Some(LANES), None);
    };
    {
        type Wide = wide::f32x16;
        #[allow(dead_code)]
        type T = f32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f32"), Some(LANES), None);
    };
    {
        type Wide = wide::f64x2;
        #[allow(dead_code)]
        type T = f64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), Some(LANES), None);
    };
    {
        type Wide = wide::f64x4;
        #[allow(dead_code)]
        type T = f64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), Some(LANES), None);
    };
    {
        type Wide = wide::f64x8;
        #[allow(dead_code)]
        type T = f64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("f64"), Some(LANES), None);
    };};
    (|Wide: WideInteger| $expr:expr) => {{
        type Wide = wide::i8x16;
        #[allow(dead_code)]
        type T = i8;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), Some(LANES), None);
    };
    {
        type Wide = wide::i8x32;
        #[allow(dead_code)]
        type T = i8;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x8;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x16;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x32;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x4;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x8;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x16;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x2;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x4;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x8;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::u8x16;
        #[allow(dead_code)]
        type T = u8;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), Some(LANES), None);
    };
    {
        type Wide = wide::u8x32;
        #[allow(dead_code)]
        type T = u8;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x8;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x16;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x32;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x4;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x8;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x16;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x2;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x4;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x8;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };};
    (|Wide: WideSigned| $expr:expr) => {{
        type Wide = wide::i8x16;
        #[allow(dead_code)]
        type T = i8;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), Some(LANES), None);
    };
    {
        type Wide = wide::i8x32;
        #[allow(dead_code)]
        type T = i8;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i8"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x8;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x16;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i16x32;
        #[allow(dead_code)]
        type T = i16;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i16"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x4;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x8;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i32x16;
        #[allow(dead_code)]
        type T = i32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i32"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x2;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x4;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };
    {
        type Wide = wide::i64x8;
        #[allow(dead_code)]
        type T = i64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("i64"), Some(LANES), None);
    };};
    (|Wide: WideUnsigned| $expr:expr) => {{
        type Wide = wide::u8x16;
        #[allow(dead_code)]
        type T = u8;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), Some(LANES), None);
    };
    {
        type Wide = wide::u8x32;
        #[allow(dead_code)]
        type T = u8;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u8"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x8;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x16;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u16x32;
        #[allow(dead_code)]
        type T = u16;
        #[allow(dead_code)]
        const LANES: usize = 32;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u16"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x4;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x8;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u32x16;
        #[allow(dead_code)]
        type T = u32;
        #[allow(dead_code)]
        const LANES: usize = 16;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u32"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x2;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 2;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x4;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 4;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };
    {
        type Wide = wide::u64x8;
        #[allow(dead_code)]
        type T = u64;
        #[allow(dead_code)]
        const LANES: usize = 8;
        crate::utils::call_with_panic_message(|| $expr, None, Some("u64"), Some(LANES), None);
    };};
    (|N, T: PrimitiveNumber| $expr:expr) => {{
        fn for_nt<const N: usize, T>(t: &'static str)
        where
            crate::Length<N>: crate::SupportedLength,
            T: crate::utils::Number,
        {
            crate::utils::call_with_panic_message(|| $expr, Some(N), Some(t), None, None);
        }

        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            for_nt::<N, f32>("f32");
            for_nt::<N, f64>("f64");
            for_nt::<N, i8>("i8");
            for_nt::<N, i16>("i16");
            for_nt::<N, i32>("i32");
            for_nt::<N, i64>("i64");
            for_nt::<N, i128>("i128");
            for_nt::<N, isize>("isize");
            for_nt::<N, u8>("u8");
            for_nt::<N, u16>("u16");
            for_nt::<N, u32>("u32");
            for_nt::<N, u64>("u64");
            for_nt::<N, u128>("u128");
            for_nt::<N, usize>("usize");
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, T: PrimitiveFloat| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = f32;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("f32"), None, None);
            };
            {
                type T = f64;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("f64"), None, None);
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, T: PrimitiveInteger| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = i8;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i8"), None, None);
            };
            {
                type T = i16;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i16"), None, None);
            };
            {
                type T = i32;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i32"), None, None);
            };
            {
                type T = i64;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i64"), None, None);
            };
            {
                type T = i128;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i128"), None, None);
            };
            {
                type T = isize;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("isize"), None, None);
            };
            {
                type T = u8;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u8"), None, None);
            };
            {
                type T = u16;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u16"), None, None);
            };
            {
                type T = u32;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u32"), None, None);
            };
            {
                type T = u64;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u64"), None, None);
            };
            {
                type T = u128;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u128"), None, None);
            };
            {
                type T = usize;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("usize"), None, None);
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, T: PrimitiveSigned| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = i8;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i8"), None, None);
            };
            {
                type T = i16;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i16"), None, None);
            };
            {
                type T = i32;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i32"), None, None);
            };
            {
                type T = i64;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i64"), None, None);
            };
            {
                type T = i128;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("i128"), None, None);
            };
            {
                type T = isize;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("isize"), None, None);
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, T: PrimitiveUnsigned| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = u8;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u8"), None, None);
            };
            {
                type T = u16;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u16"), None, None);
            };
            {
                type T = u32;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u32"), None, None);
            };
            {
                type T = u64;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u64"), None, None);
            };
            {
                type T = u128;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("u128"), None, None);
            };
            {
                type T = usize;
                crate::utils::call_with_panic_message(|| $expr, Some(N), Some("usize"), None, None);
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, Wide| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::f32x4;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f32x8;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f32x16;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f64x2;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f64x4;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f64x8;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, Wide: WideFloat| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::f32x4;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f32x8;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f32x16;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f64x2;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f64x4;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::f64x8;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    None,
                );
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, Wide: WideInteger| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, Wide: WideSigned| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    None,
                );
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|N, Wide: WideUnsigned| $expr:expr) => {{
        fn for_n<const N: usize>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    None,
                );
            };
        }

        for_n::<2>();
        for_n::<3>();
        for_n::<4>();
    };};
    (|A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            crate::utils::call_with_panic_message(|| $expr, None, None, None, Some(A::IS_ALIGNED));
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|N, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            crate::utils::call_with_panic_message(
                || $expr,
                Some(N),
                None,
                None,
                Some(A::IS_ALIGNED),
            );
        }

        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
    };};
    (|T: PrimitiveNumber, A| $expr:expr) => {{
        fn for_ta<T, A: crate::Alignment>(t: &'static str)
        where
            T: crate::utils::Number,
        {
            crate::utils::call_with_panic_message(
                || $expr,
                None,
                Some(t),
                None,
                Some(A::IS_ALIGNED),
            );
        }

        fn for_a<A: crate::Alignment>() {
            for_ta::<f32, A>("f32");
            for_ta::<f64, A>("f64");
            for_ta::<i8, A>("i8");
            for_ta::<i16, A>("i16");
            for_ta::<i32, A>("i32");
            for_ta::<i64, A>("i64");
            for_ta::<i128, A>("i128");
            for_ta::<isize, A>("isize");
            for_ta::<u8, A>("u8");
            for_ta::<u16, A>("u16");
            for_ta::<u32, A>("u32");
            for_ta::<u64, A>("u64");
            for_ta::<u128, A>("u128");
            for_ta::<usize, A>("usize");
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|T: PrimitiveFloat, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type T = f32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = f64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|T: PrimitiveInteger, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type T = i8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = isize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("isize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = usize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("usize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|T: PrimitiveSigned, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type T = i8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = isize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("isize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|T: PrimitiveUnsigned, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type T = u8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = usize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("usize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|Wide, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type Wide = wide::f32x4;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x8;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x16;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x2;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x4;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x8;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|Wide: WideFloat, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type Wide = wide::f32x4;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x8;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x16;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x2;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x4;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x8;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|Wide: WideInteger, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|Wide: WideSigned, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|Wide: WideUnsigned, A| $expr:expr) => {{
        fn for_a<A: crate::Alignment>() {
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    None,
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_a::<crate::Aligned>();
        for_a::<crate::Unaligned>();
    };};
    (|N, T: PrimitiveNumber, A| $expr:expr) => {{
        fn for_nta<const N: usize, T, A: crate::Alignment>(t: &'static str)
        where
            crate::Length<N>: crate::SupportedLength,
            T: crate::utils::Number,
            crate::Vector<N, T, A>: crate::utils::Random,
            crate::Matrix<N, T, A>: crate::utils::Random,
            crate::Affine<N, T, A>: crate::utils::Random,
        {
            crate::utils::call_with_panic_message(
                || $expr,
                Some(N),
                Some(t),
                None,
                Some(A::IS_ALIGNED),
            );
        }

        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            for_nta::<N, f32, A>("f32");
            for_nta::<N, f64, A>("f64");
            for_nta::<N, i8, A>("i8");
            for_nta::<N, i16, A>("i16");
            for_nta::<N, i32, A>("i32");
            for_nta::<N, i64, A>("i64");
            for_nta::<N, i128, A>("i128");
            for_nta::<N, isize, A>("isize");
            for_nta::<N, u8, A>("u8");
            for_nta::<N, u16, A>("u16");
            for_nta::<N, u32, A>("u32");
            for_nta::<N, u64, A>("u64");
            for_nta::<N, u128, A>("u128");
            for_nta::<N, usize, A>("usize");
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, T: PrimitiveFloat, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = f32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = f64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, T: PrimitiveInteger, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = i8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = isize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("isize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = usize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("usize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, T: PrimitiveSigned, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = i8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = i128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = isize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("isize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, T: PrimitiveUnsigned, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type T = u8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u64;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = u128;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u128"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type T = usize;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("usize"),
                    None,
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, Wide, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::f32x4;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x8;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x16;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x2;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x4;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x8;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, Wide: WideFloat, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::f32x4;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x8;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f32x16;
                #[allow(dead_code)]
                type T = f32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x2;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x4;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::f64x8;
                #[allow(dead_code)]
                type T = f64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("f64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, Wide: WideInteger, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, Wide: WideSigned, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::i8x16;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i8x32;
                #[allow(dead_code)]
                type T = i8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x8;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x16;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i16x32;
                #[allow(dead_code)]
                type T = i16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x4;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x8;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i32x16;
                #[allow(dead_code)]
                type T = i32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x2;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x4;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::i64x8;
                #[allow(dead_code)]
                type T = i64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("i64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
    (|N, Wide: WideUnsigned, A| $expr:expr) => {{
        fn for_na<const N: usize, A: crate::Alignment>()
        where
            crate::Length<N>: crate::SupportedLength,
        {
            {
                type Wide = wide::u8x16;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u8x32;
                #[allow(dead_code)]
                type T = u8;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u8"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x8;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x16;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u16x32;
                #[allow(dead_code)]
                type T = u16;
                #[allow(dead_code)]
                const LANES: usize = 32;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u16"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x4;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x8;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u32x16;
                #[allow(dead_code)]
                type T = u32;
                #[allow(dead_code)]
                const LANES: usize = 16;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u32"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x2;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 2;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x4;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 4;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
            {
                type Wide = wide::u64x8;
                #[allow(dead_code)]
                type T = u64;
                #[allow(dead_code)]
                const LANES: usize = 8;
                crate::utils::call_with_panic_message(
                    || $expr,
                    Some(N),
                    Some("u64"),
                    Some(LANES),
                    Some(A::IS_ALIGNED),
                );
            };
        }

        for_na::<2, crate::Aligned>();
        for_na::<3, crate::Aligned>();
        for_na::<4, crate::Aligned>();
        for_na::<2, crate::Unaligned>();
        for_na::<3, crate::Unaligned>();
        for_na::<4, crate::Unaligned>();
    };};
}
pub(crate) use for_types;

use crate::{
    Scalar,
    constants::{One, Zero},
};

#[doc(hidden)]
pub trait Number: Scalar + Zero + One + num_primitive::PrimitiveNumber {}

impl Number for f32 {}
impl Number for f64 {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}

#[doc(hidden)]
pub fn call_with_panic_message(
    f: impl FnOnce() + UnwindSafe,
    n: Option<usize>,
    t: Option<&'static str>,
    lanes: Option<usize>,
    a: Option<bool>,
) {
    match catch_unwind(f) {
        Ok(_) => {}
        Err(payload) => {
            if let Some(n) = n {
                println!("N: {n}");
            };
            if let Some(t) = t {
                println!("T: {t}");
            };
            if let Some(lanes) = lanes {
                println!("LANES: {lanes}");
            };
            if let Some(a) = a {
                println!("A: {}", if a { "Aligned" } else { "Unaligned" });
            };

            resume_unwind(payload);
        }
    }
}
