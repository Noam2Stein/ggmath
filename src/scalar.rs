use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::{Aligned, Alignment, Length, SupportedLength, Unaligned, Vector};

/// A trait for elements of math types.
///
/// Types that implement `Scalar` can be used as the `T` in math types like
/// [`Vector`].
///
/// All scalars must implement [`Copy`].
///
/// In order to make SIMD optimizations possible, implementing `Scalar` comes
/// with boilerplate. See the example below for a simple implementation.
///
/// # Safety
///
/// `Scalar` is only unsafe for SIMD implementations. Implementations where
/// `Repr = ()` (like the example below) are safe.
///
/// If `Scalar::Repr` is a signed integer:
///
/// - `Self` and `Self::Repr` must have the same size.
/// - `Self` must have no uninitialized bytes and no padding.
///
/// # Example
///
/// Implementing without SIMD optimizations:
///
/// ```
/// use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
///
/// #[derive(Clone, Copy)]
/// struct Foo(f32);
///
/// // SAFETY: `Scalar` is only unsafe for SIMD implementations. Implementations
/// // where `Repr = ()` are safe.
/// unsafe impl Scalar for Foo {
///     type Repr = ();
/// }
///
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
/// }
/// ```
///
/// # Making SIMD Optimizations
///
/// `Scalar` implementations can enable SIMD optimizations for their scalar's
/// math types.
///
/// This is done by:
///
/// - Enabling SIMD alignment via [`Scalar::Repr`].
/// - Overriding math function implementations via [`ScalarBackend`].
///
/// `Scalar::Repr` is a marker type that controls SIMD alignment. It can be either:
///
/// - `()`: There won't be any SIMD alignment.
/// - A signed integer: There will be SIMD alignment appropriate for that
///   integer's size, where the integer must have the size of the scalar type.
///
/// Let's make a scalar named `Foo` which is a wrapper around `f32`:
///
/// ```
/// use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
///
/// #[repr(transparent)]
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct Foo(f32);
///
/// // SAFETY: `Foo` and `i32` are both 4-bytes long, and `Foo` has no
/// // uninitialized bytes because its a simple wrapper around `f32`.
/// unsafe impl Scalar for Foo {
///     type Repr = i32;
/// }
///
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
/// }
/// ```
///
/// Because [`Scalar::Repr`] is `i32`, math types of `Foo` will have SIMD
/// alignment appropriate for `Foo`'s size, 4-bytes.
///
/// Lets implement [`Add`] for `Foo`:
///
/// ```
/// # #[repr(transparent)]
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct Foo(f32);
/// #
/// use std::ops::Add;
///
/// impl Add for Foo {
///     type Output = Self;
///
///     #[inline]
///     fn add(self, rhs: Self) -> Self::Output {
///         Self(self.0 + rhs.0)
///     }
/// }
/// ```
///
/// Our implementation simply adds the internal `f32` values and returns the
/// result. Implementing `Add` for `Foo` automatically implements `Add` for
/// `Foo` vectors:
///
/// ```
/// # use std::ops::Add;
/// #
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
/// #
/// # #[repr(transparent)]
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct Foo(f32);
/// #
/// # // SAFETY: `Foo` and `i32` are both 4-bytes long, and `Foo` has no
/// # // uninitialized bytes because its a simple wrapper around `f32`.
/// # unsafe impl Scalar for Foo {
/// #     type Repr = i32;
/// # }
/// #
/// # impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// # where
/// #     Length<N>: SupportedLength,
/// # {
/// # }
/// #
/// # impl Add for Foo {
/// #     type Output = Self;
/// #
/// #     #[inline]
/// #     fn add(self, rhs: Self) -> Self::Output {
/// #         Self(self.0 + rhs.0)
/// #     }
/// # }
/// #
/// use ggmath::{Vec2, vec2};
///
/// let result: Vec2<Foo> = vec2!(Foo(1.0), Foo(2.0)) + vec2!(Foo(3.0), Foo(4.0));
///
/// assert_eq!(result, vec2!(Foo(1.0 + 3.0), Foo(2.0 + 4.0)));
/// ```
///
/// But currently the implementation for `Foo` vectors doesn't use SIMD. To fix
/// this, we can override the default implementation for
/// [`ScalarBackend::vec_add`] which controls vector addition:
///
/// ```compile_fail
/// # use std::ops::Add;
/// #
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
/// #
/// # #[repr(transparent)]
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct Foo(f32);
/// #
/// # // SAFETY: `Foo` and `i32` are both 4-bytes long, and `Foo` has no
/// # // uninitialized bytes because its a simple wrapper around `f32`.
/// # unsafe impl Scalar for Foo {
/// #     type Repr = i32;
/// # }
/// #
/// # impl Add for Foo {
/// #     type Output = Self;
/// #
/// #     #[inline]
/// #     fn add(self, rhs: Self) -> Self::Output {
/// #         Self(self.0 + rhs.0)
/// #     }
/// # }
/// #
/// use ggmath::Vector;
///
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
///     #[inline]
///     fn vec_add(
///         vec: Vector<N, Self, A>,
///         rhs: Vector<N, Self, A>,
///     ) -> Vector<N, Self, A> {
///         // TODO: implement this function.
///     }
/// }
/// ```
///
/// To make a SIMD implementation we need to know the underlying representation
/// of `Foo` vectors.
///
/// Its guaranteed that vectors of scalars with the same `Repr` have the same
/// memory layout. [`Vector::to_repr`] can be used to reinterpret the bits of
/// vectors to different scalar types where appropriate.
///
/// Lets implement extension methods for `Foo` and `f32` vectors to convert
/// between the two:
///
/// ```
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
/// #
/// # #[repr(transparent)]
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct Foo(f32);
/// #
/// # // SAFETY: `Foo` and `i32` are both 4-bytes long, and `Foo` has no
/// # // uninitialized bytes because its a simple wrapper around `f32`.
/// # unsafe impl Scalar for Foo {
/// #     type Repr = i32;
/// # }
/// #
/// # impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// # where
/// #     Length<N>: SupportedLength,
/// # {
/// # }
/// #
/// use ggmath::Vector;
///
/// trait ToF32 {
///     type Output;
///
///     fn to_f32(self) -> Self::Output;
/// }
///
/// trait ToFoo {
///     type Output;
///
///     fn to_foo(self) -> Self::Output;
/// }
///
/// impl<const N: usize, A: Alignment> ToF32 for Vector<N, Foo, A>
/// where
///     Length<N>: SupportedLength,
/// {
///     type Output = Vector<N, f32, A>;
///
///     #[inline]
///     fn to_f32(self) -> Self::Output {
///         // SAFETY: `f32` accepts all bit-patterns.
///         unsafe { self.to_repr() }
///     }
/// }
///
/// impl<const N: usize, A: Alignment> ToFoo for Vector<N, f32, A>
/// where
///     Length<N>: SupportedLength,
/// {
///     type Output = Vector<N, Foo, A>;
///
///     #[inline]
///     fn to_foo(self) -> Self::Output {
///         // SAFETY: `Foo` accepts all bit-patterns.
///         unsafe { self.to_repr() }
///     }
/// }
/// ```
///
/// `Vector::to_repr` is unsafe because the input arguments must be valid for
/// the output type. In our case both `Foo` and `f32` accept all bit-patterns
/// and so any conversion is safe.
///
/// Lets implement `vec_add` using these methods:
///
/// ```
/// # use std::ops::Add;
/// #
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength, Vector};
/// #
/// # #[repr(transparent)]
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct Foo(f32);
/// #
/// # // SAFETY: `Foo` and `i32` are both 4-bytes long, and `Foo` has no
/// # // uninitialized bytes because its a simple wrapper around `f32`.
/// # unsafe impl Scalar for Foo {
/// #     type Repr = i32;
/// # }
/// #
/// # impl Add for Foo {
/// #     type Output = Self;
/// #
/// #     #[inline]
/// #     fn add(self, rhs: Self) -> Self::Output {
/// #         Self(self.0 + rhs.0)
/// #     }
/// # }
/// #
/// # trait ToF32 {
/// #     type Output;
/// #
/// #     fn to_f32(self) -> Self::Output;
/// # }
/// #
/// # trait ToFoo {
/// #     type Output;
/// #
/// #     fn to_foo(self) -> Self::Output;
/// # }
/// #
/// # impl<const N: usize, A: Alignment> ToF32 for Vector<N, Foo, A>
/// # where
/// #     Length<N>: SupportedLength,
/// # {
/// #     type Output = Vector<N, f32, A>;
/// #
/// #     #[inline]
/// #     fn to_f32(self) -> Self::Output {
/// #         // SAFETY: `f32` accepts all bit-patterns.
/// #         unsafe { self.to_repr() }
/// #     }
/// # }
/// #
/// # impl<const N: usize, A: Alignment> ToFoo for Vector<N, f32, A>
/// # where
/// #     Length<N>: SupportedLength,
/// # {
/// #     type Output = Vector<N, Foo, A>;
/// #
/// #     #[inline]
/// #     fn to_foo(self) -> Self::Output {
/// #         // SAFETY: `Foo` accepts all bit-patterns.
/// #         unsafe { self.to_repr() }
/// #     }
/// # }
/// #
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
///     #[inline]
///     fn vec_add(
///         vec: Vector<N, Self, A>,
///         rhs: Vector<N, Self, A>,
///     ) -> Vector<N, Self, A> {
///         (vec.to_f32() + rhs.to_f32()).to_foo()
///     }
/// }
/// ```
///
/// `Foo` vector addition now uses the same implementation as `f32` vector
/// addition. Because of this, any SIMD optimization the `f32` implementation
/// has, the `Foo` implementation will have as well.
///
/// This pattern can then be expanded for all vector functions to make a fully optimized scalar.
///
/// SIMD optimizations can only be made using math types of primitives.
/// Implementations that directly use intrinsics aren't supported because the
/// exact representation of math types isn't stable.
pub unsafe trait Scalar:
    Copy
    + ScalarBackend<2, Aligned>
    + ScalarBackend<3, Aligned>
    + ScalarBackend<4, Aligned>
    + ScalarBackend<2, Unaligned>
    + ScalarBackend<3, Unaligned>
    + ScalarBackend<4, Unaligned>
{
    /// Controls the representation of math types.
    ///
    /// `Scalar::Repr` is a marker type that controls what SIMD alignment math
    /// types have.
    ///
    /// If `Repr` is `()`, math types won't have any SIMD alignment.
    ///
    /// If `Repr` is the signed integer with the size of `Self`, math types will
    /// have SIMD alignment appropriate for your scalar's size.
    ///
    /// For more information, see the documentation for [`Scalar`].
    ///
    /// # Safety
    ///
    /// `Scalar` implementations where `Repr = ()` are safe.
    ///
    /// If `Repr` is a signed integer:
    ///
    /// - `Self` and `Self::Repr` must have the same size.
    /// - `Self` must have no uninitialized bytes and no padding.
    #[expect(private_bounds)]
    type Repr: ScalarRepr;
}

/// Controls the implementation of math functions.
///
/// `ScalarBackend<N, A>` controls the function implementations of math types
/// with length `N`, scalar type `Self`, and alignment `A`.
///
/// All `ScalarBackend` functions have a default implementation that can be
/// overriden to make SIMD optimizations.
///
/// You should only override implementations to make optimizations, and
/// generally you should be consistent with the behaviour of default
/// implementations.
///
/// For more information, see the documentation for [`Scalar`].
#[diagnostic::on_unimplemented(
    message = "`{Self}` is missing an implementation for `ScalarBackend`",
    note = "see the documentation for `Scalar`"
)]
pub trait ScalarBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    /// Overridable implementation of the `==` operator for vectors.
    #[inline]
    fn vec_eq(vec: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        (0..N).all(|i| vec[i] == other[i])
    }

    /// Overridable implementation of the `!=` operator for vectors.
    #[inline]
    fn vec_ne(vec: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        !Self::vec_eq(vec, other)
    }

    /// Overridable implementation of the `-` operator for vectors.
    #[inline]
    fn vec_neg(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Neg<Output = Self>,
    {
        vec.map(Self::neg)
    }

    /// Overridable implementation of the `!` operator for vectors.
    #[inline]
    fn vec_not(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Not<Output = Self>,
    {
        vec.map(Self::not)
    }

    /// Overridable implementation of the `+` operator for vectors.
    #[inline]
    fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Add<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] + rhs[i])
    }

    /// Overridable implementation of the `-` operator for vectors.
    #[inline]
    fn vec_sub(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Sub<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] - rhs[i])
    }

    /// Overridable implementation of the `*` operator for vectors.
    #[inline]
    fn vec_mul(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Mul<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] * rhs[i])
    }

    /// Overridable implementation of the `/` operator for vectors.
    #[inline]
    fn vec_div(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Div<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] / rhs[i])
    }

    /// Overridable implementation of the `%` operator for vectors.    
    #[inline]
    fn vec_rem(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Rem<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] % rhs[i])
    }

    /// Overridable implementation of the `<<` operator for vectors.
    #[inline]
    fn vec_shl(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shl<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] << rhs[i])
    }

    /// Overridable implementation of the `>>` operator for vectors.
    #[inline]
    fn vec_shr(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] >> rhs[i])
    }

    /// Overridable implementation of the `&` operator for vectors.
    #[inline]
    fn vec_bitand(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] & rhs[i])
    }

    /// Overridable implementation of the `|` operator for vectors.
    #[inline]
    fn vec_bitor(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] | rhs[i])
    }

    /// Overridable implementation of the `^` operator for vectors.
    #[inline]
    fn vec_bitxor(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] ^ rhs[i])
    }
}

/// Types accepted by [`Scalar::Repr`].
///
/// # Safety
///
/// All associated types must uphold the guarantees of their math types.
pub(crate) unsafe trait ScalarRepr {
    type VectorRepr<const N: usize, T, A: Alignment>: Copy
    where
        Length<N>: SupportedLength,
        T: Scalar;
}

/// Used by [`Vector::to_repr`] to reject transmuting between vectors of `Repr = ()`.
///
/// # Safety
///
/// This trait must not be implemented for types that are not signed integer
/// primitives.
pub(crate) unsafe trait SignedInteger {}

unsafe impl Scalar for f32 {
    type Repr = i32;
}

unsafe impl Scalar for f64 {
    type Repr = i64;
}

unsafe impl Scalar for i8 {
    type Repr = i8;
}

unsafe impl Scalar for i16 {
    type Repr = i16;
}

unsafe impl Scalar for i32 {
    type Repr = i32;
}

unsafe impl Scalar for i64 {
    type Repr = i64;
}

unsafe impl Scalar for i128 {
    type Repr = i128;
}

unsafe impl Scalar for isize {
    #[cfg(target_pointer_width = "16")]
    type Repr = i16;

    #[cfg(target_pointer_width = "32")]
    type Repr = i32;

    #[cfg(target_pointer_width = "64")]
    type Repr = i64;
}

unsafe impl Scalar for u8 {
    type Repr = i8;
}

unsafe impl Scalar for u16 {
    type Repr = i16;
}

unsafe impl Scalar for u32 {
    type Repr = i32;
}

unsafe impl Scalar for u64 {
    type Repr = i64;
}

unsafe impl Scalar for u128 {
    type Repr = i128;
}

unsafe impl Scalar for usize {
    #[cfg(target_pointer_width = "16")]
    type Repr = i16;

    #[cfg(target_pointer_width = "32")]
    type Repr = i32;

    #[cfg(target_pointer_width = "64")]
    type Repr = i64;
}

unsafe impl Scalar for bool {
    type Repr = i8;
}

unsafe impl SignedInteger for i8 {}
unsafe impl SignedInteger for i16 {}
unsafe impl SignedInteger for i32 {}
unsafe impl SignedInteger for i64 {}
unsafe impl SignedInteger for i128 {}
unsafe impl SignedInteger for isize {}
