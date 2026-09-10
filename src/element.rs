use crate::{
    Aligned, Alignment, Unaligned,
    backend::{AffineBackend, DefaultBackend, MaskBackend, RotorBackend, VectorBackend},
};

/// A trait for elements of vectors.
///
/// This requires [`Copy`].
///
/// Due to type system limitations, this trait cannot be implemented directly.
/// Instead implement the [`CustomElement`] trait:
///
/// ```
/// use ggmath::{Alignment, CustomElement, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl CustomElement for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
#[expect(private_bounds)]
pub trait Element:
    Copy
    + VectorBackend<2, Aligned>
    + VectorBackend<3, Aligned>
    + VectorBackend<4, Aligned>
    + VectorBackend<2, Unaligned>
    + VectorBackend<3, Unaligned>
    + VectorBackend<4, Unaligned>
    + AffineBackend<2, Aligned>
    + AffineBackend<3, Aligned>
    + AffineBackend<4, Aligned>
    + AffineBackend<2, Unaligned>
    + AffineBackend<3, Unaligned>
    + AffineBackend<4, Unaligned>
    + RotorBackend<3, Aligned>
    + RotorBackend<3, Unaligned>
    + MaskBackend<2, Aligned>
    + MaskBackend<3, Aligned>
    + MaskBackend<4, Aligned>
    + MaskBackend<2, Unaligned>
    + MaskBackend<3, Unaligned>
    + MaskBackend<4, Unaligned>
{
}

/// A trait to implement [`Element`] for downstream types.
///
/// Due to type system limitations, the [`Element`] trait cannot be implemented
/// directly. Instead implement this trait:
///
/// ```
/// use ggmath::{Alignment, CustomElement, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl CustomElement for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
pub trait CustomElement: Copy {}

#[diagnostic::do_not_recommend]
impl<T> Element for T where T: CustomElement {}

#[diagnostic::do_not_recommend]
impl<T, const N: usize, A: Alignment> DefaultBackend<N, A> for T where T: CustomElement {}

impl Element for f32 {}

impl Element for f64 {}

impl Element for i8 {}

impl Element for i16 {}

impl Element for i32 {}

impl Element for i64 {}

impl Element for i128 {}

impl Element for isize {}

impl Element for u8 {}

impl Element for u16 {}

impl Element for u32 {}

impl Element for u64 {}

impl Element for u128 {}

impl Element for usize {}

impl Element for bool {}

#[cfg(feature = "half")]
mod half_impl {
    use half::{bf16, f16};

    use crate::CustomElement;

    impl CustomElement for f16 {}

    impl CustomElement for bf16 {}
}
