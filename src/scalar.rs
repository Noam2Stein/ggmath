use crate::{
    Aligned, Alignment, Unaligned,
    backend::{DefaultBackend, MaskBackend, QuaternionBackend, RotorBackend, VectorBackend},
};

/// A trait for elements of vectors.
///
/// This requires [`Copy`].
///
/// Due to type system limitations, this trait cannot be implemented directly.
/// Instead implement the [`CustomScalar`] trait:
///
/// ```
/// use ggmath::{Alignment, CustomScalar, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl CustomScalar for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
#[expect(private_bounds)]
pub trait Scalar:
    Copy
    + VectorBackend<2, Aligned>
    + VectorBackend<3, Aligned>
    + VectorBackend<4, Aligned>
    + VectorBackend<2, Unaligned>
    + VectorBackend<3, Unaligned>
    + VectorBackend<4, Unaligned>
    + RotorBackend<3, Aligned>
    + RotorBackend<3, Unaligned>
    + QuaternionBackend<Aligned>
    + QuaternionBackend<Unaligned>
    + MaskBackend<2, Aligned>
    + MaskBackend<3, Aligned>
    + MaskBackend<4, Aligned>
    + MaskBackend<2, Unaligned>
    + MaskBackend<3, Unaligned>
    + MaskBackend<4, Unaligned>
{
}

/// A trait to implement [`Scalar`] for downstream types.
///
/// Due to type system limitations, the [`Scalar`] trait cannot be implemented
/// directly. Instead implement this trait:
///
/// ```
/// use ggmath::{Alignment, CustomScalar, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl CustomScalar for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
pub trait CustomScalar: Copy {}

#[diagnostic::do_not_recommend]
impl<T> Scalar for T where T: CustomScalar {}

#[diagnostic::do_not_recommend]
impl<T, const N: usize, A: Alignment> DefaultBackend<N, A> for T where T: CustomScalar {}

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
