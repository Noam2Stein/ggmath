use crate::{Aligned, Unaligned, backend::Backend};

/// A trait for elements of vectors.
///
/// This requires [`Copy`], and due to type system limitations, the [`Backend`]
/// trait which controls the internal implementation of vectors. To simply
/// implement [`Scalar`], use the [`DefaultBackend`] trait:
///
/// ```
/// use ggmath::{Alignment, DefaultBackend, Scalar, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl Scalar for Foo {}
///
/// impl<const N: usize, A: Alignment> DefaultBackend<N, A> for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
///
/// Manual implementations of [`Backend`] should only exist to make
/// optimizations.
///
/// [`DefaultBackend`]: crate::DefaultBackend
pub trait Scalar:
    Copy
    + Backend<2, Aligned>
    + Backend<3, Aligned>
    + Backend<4, Aligned>
    + Backend<2, Unaligned>
    + Backend<3, Unaligned>
    + Backend<4, Unaligned>
{
}

impl Scalar for f32 {}

impl Scalar for f64 {}

impl Scalar for i8 {}

impl Scalar for i16 {}

impl Scalar for i32 {}

impl Scalar for i64 {}

impl Scalar for i128 {}

impl Scalar for isize {}

impl Scalar for u8 {}

impl Scalar for u16 {}

impl Scalar for u32 {}

impl Scalar for u64 {}

impl Scalar for u128 {}

impl Scalar for usize {}

impl Scalar for bool {}
