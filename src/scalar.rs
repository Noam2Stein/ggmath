use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::{Aligned, Alignment, Length, SupportedLength, Unaligned, Vector};

/// A trait for types that can be stored in vectors.
///
/// All scalars must implement the [`Copy`] trait, and the
/// [`ScalarBackend<N, A>`] trait which controls the internal representation and
/// function implementations of the scalar's math types.
///
/// For simple implementations there is the [`ScalarDefault`] trait which
/// provides a default implementation for `ScalarBackend`.
///
/// # Example
///
/// ```
/// use ggmath::{Scalar, ScalarDefault, Vec2, Vec3, Vec4, vec2, vec3, vec4};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(f32);
///
/// impl Scalar for Foo {}
///
/// impl ScalarDefault for Foo {}
///
/// let v2: Vec2<Foo> = vec2!(Foo(1.0), Foo(2.0));
/// let v3: Vec3<Foo> = vec3!(Foo(1.0), Foo(2.0), Foo(3.0));
/// let v4: Vec4<Foo> = vec4!(Foo(1.0), Foo(2.0), Foo(3.0), Foo(4.0));
///
/// println!("{v2:?}, {v3:?}, {v4:?}");
/// ```
pub trait Scalar:
    Copy
    + ScalarBackend<2, Aligned>
    + ScalarBackend<3, Aligned>
    + ScalarBackend<4, Aligned>
    + ScalarBackend<2, Unaligned>
    + ScalarBackend<3, Unaligned>
    + ScalarBackend<4, Unaligned>
{
}

/// A trait to control the implementation of math types.
///
/// More specifically, this trait controls the internal representation and
/// function implementations of math types with `N` as their length, `Self` as
/// their scalar type, and `A` as their alignment.
///
/// # Default Implementation
///
/// The [`ScalarDefault`] trait provides a default implementation for
/// `ScalarBackend`.
///
/// Manually implementing `ScalarBackend` is *unsafe* and should only be done to
/// make SIMD optimizations to a scalar's math types.
///
/// # Required Items
///
/// `ScalarBackend` has only one required item: [`ScalarBackend::VectorRepr`].
/// This associated type is used as the internal representation for
/// [`Vector<N, Self, A>`].
///
/// # Safety
///
/// Implementations must respect the requirements of the associated type
/// [`ScalarBackend::VectorRepr`].
///
/// # Example
///
/// Technically, SIMD optimizations can be made by implementing `ScalarBackend`
/// seperately for each length and alignment, using target architecture specific
/// SIMD intrinsics. This is what primitives like `f32` do.
///
/// An easier approach is to make our scalar type a wrapper of a primitive, then
/// use that primitive's math types as our internal representation and to
/// implement math functions.
///
/// Lets define scalar type `Foo` that is a wrapper around `f32`:
///
/// ```
/// use ggmath::Scalar;
///
/// #[repr(transparent)]
/// #[derive(Clone, Copy)]
/// struct Foo(f32);
///
/// impl Scalar for Foo {}
///
/// // We will replace this with a manual implementation.
/// impl ggmath::ScalarDefault for Foo {}
/// ```
///
/// Lets replace the default implementation with a manual one using
/// `Vector<N, f32, A>` as the internal representation for `Vector<N, Foo, A>`:
///
/// ```
/// # use ggmath::Scalar;
/// #
/// # #[repr(transparent)]
/// # #[derive(Clone, Copy)]
/// # struct Foo(f32);
/// #
/// # impl Scalar for Foo {}
/// #
/// use ggmath::{Alignment, Length, ScalarBackend, SupportedLength, Vector};
///
/// // SAFETY: Because `Foo` is a wrapper around `f32`, any internal
/// // representation that `Vector<N, f32, A>` may have is a valid
/// // representation for `Vector<N, Foo, A>`.
/// unsafe impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
///     type VectorRepr = Vector<N, f32, A>;
/// }
/// ```
///
/// Now whenever `f32` vectors have SIMD alignment, our vectors have the same
/// alignment too.
///
/// Lets implement addition for `Foo` by adding up the internal `f32`s:
///
/// ```
/// # #[repr(transparent)]
/// # #[derive(Clone, Copy)]
/// # struct Foo(f32);
/// #
/// use core::ops::Add;
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
/// An implementation of vector addition that is consistant with `Foo` addition
/// should add up the internal `f32` vectors just like `Foo` addition adds up
/// the internal `f32`s.
///
/// To implement SIMD optimized vector addition we need functions for converting
/// between `Foo` vectors and `f32` vectors. The builtin functions for
/// conversions are [`Vector::repr`] and [`Vector::from_repr`]. The latter is
/// unsafe because the internal representation of a vector might have
/// less memory safety guarantees than the outer vector.
///
/// Lets make an extension method for `f32` vectors that converts them to `Foo`
/// vectors:
///
/// ```
/// # use ggmath::Scalar;
/// #
/// # #[repr(transparent)]
/// # #[derive(Clone, Copy)]
/// # struct Foo(f32);
/// #
/// # impl Scalar for Foo {}
/// #
/// # use ggmath::{Alignment, Length, ScalarBackend, SupportedLength, Vector};
/// #
/// # unsafe impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// # where
/// #     Length<N>: SupportedLength,
/// # {
/// #     type VectorRepr = Vector<N, f32, A>;
/// # }
/// #
/// trait ToFoo {
///     type Output;
///
///     fn to_foo(self) -> Self::Output;
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
///         // SAFETY: Any value of `f32` is a valid value of `Foo`, so any
///         // value of an `f32` vector is a valid value of a `Foo` vector.
///         unsafe { Vector::from_repr(self) }
///     }
/// }
/// ```
///
/// Now that everything is ready lets implement [`ScalarBackend::vec_add`] which
/// controls the implementation for vector addition:
///
/// ```
/// # use ggmath::Scalar;
/// #
/// # #[repr(transparent)]
/// # #[derive(Clone, Copy)]
/// # struct Foo(f32);
/// #
/// # impl Scalar for Foo {}
/// #
/// # use ggmath::{Alignment, Length, ScalarBackend, SupportedLength, Vector};
/// #
/// # trait ToFoo {
/// #     type Output;
/// #
/// #     fn to_foo(self) -> Self::Output;
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
/// #         // SAFETY: Any value of `f32` is a valid value of `Foo`, so any
/// #         // value of an `f32` vector is a valid value of a `Foo` vector.
/// #         unsafe { Vector::from_repr(self) }
/// #     }
/// # }
/// #
/// // SAFETY: Because `Foo` is a wrapper around `f32`, any internal
/// // representation that `Vector<N, f32, A>` may have is a valid
/// // representation for `Vector<N, Foo, A>`.
/// unsafe impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
///     type VectorRepr = Vector<N, f32, A>;
///     
///     #[inline]
///     fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
///         (vec.repr() + rhs.repr()).to_foo()
///     }
/// }
/// ```
///
/// Now `Foo` vector addition has whatever SIMD optimizations `f32` vectors
/// have. This pattern can be expanded for all operators to have SIMD
/// optimizations.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is missing an implementation for `ScalarBackend`",
    note = "consider implementing `ScalarDefault` for `{Self}`"
)]
pub unsafe trait ScalarBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    /// The internal representation for [`Vector<N, Self, A>`].
    ///
    /// # Safety
    ///
    /// The specified type must meet the following requirements:
    ///
    /// `Vector<N, T, A>` must be [transmutable](core::mem::transmute) to
    /// `[T; N2]` where `N2` is `N` plus padding, meaning padding must be an
    /// initialized value of `T`.
    ///
    /// `Vector<N, T, Unaligned>` must have the memory layout of `[T; N]`.
    ///
    /// `Vector<2, T, Aligned>` and `Vector<4, T, Aligned>` must have the size
    /// of `[T; N]`, and may have additional alignment.
    ///
    /// `Vector<3, T, Aligned>` must have the size of either `[T; 3]` or
    /// `[T; 4]`, and may have additional alignment.
    ///
    /// Failure to meet these requirements results in undefined behaviour.
    type VectorRepr: Copy;

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

/// A default implementation for [`ScalarBackend`].
///
/// The `ScalarBackend` trait is required to implement [`Scalar`] but is
/// *unsafe* and hard to implement. `ScalarDefault` provides a safe default
/// implementation.
///
/// Don't use `ScalarDefault` as a generic bound because types that implement
/// `ScalarDefault` today might silently switch to a manual implementation of
/// `ScalarBackend` in the future.
///
/// # Example
///
/// ```
/// use ggmath::{Scalar, ScalarDefault};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(f32);
///
/// impl Scalar for Foo {}
///
/// impl ScalarDefault for Foo {}
/// ```
#[diagnostic::on_unimplemented(
    message = "`{Self}` is missing an implementation for `ScalarBackend`",
    note = "consider implementing `ScalarDefault` for `{Self}`"
)]
pub trait ScalarDefault {}

unsafe impl<const N: usize, T, A: Alignment> ScalarBackend<N, A> for T
where
    T: Scalar + ScalarDefault,
    Length<N>: SupportedLength,
{
    type VectorRepr = Vector<N, T, Unaligned>;
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
