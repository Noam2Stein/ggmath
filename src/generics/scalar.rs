use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::{
    Affine, Aligned, Alignment, Length, Mask, MaskBackend, Matrix, SupportedLength, Unaligned,
    Vector,
};

/// A trait for elements of math types.
///
/// Types that implement `Scalar` can be used as the `T` in math types.
///
/// Implementations of `Scalar` can make SIMD optimizations for their math
/// types (see "Making SIMD optimizations"). Implementing `Scalar` without
/// optimizations still comes with boilerplate (see "Simple implementation").
///
/// # Safety
///
/// `Scalar` is only unsafe for SIMD implementations. The "Simple
/// implementation" below is safe.
///
/// If [`Self::Repr`] is a signed integer:
///
/// - `Self` and [`Self::Repr`] must have the same size.
/// - `Self` must never have uninitialized bytes.
///
/// # Simple implementation
///
/// This is the boilerplate required to implement `Scalar`:
///
/// ```
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
/// #
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
/// # Making SIMD optimizations
///
/// SIMD optimizations are made by wrapping a primitive type then reusing its
/// already SIMD-optimized functionality. This essentially derives all SIMD
/// optimizations of that primitive for our scalar type.
///
/// The associated marker type [`Repr`] controls SIMD alignment. It can either
/// be `()` for "disable SIMD alignment", or a signed integer for "enable SIMD
/// alignment appropriate for a specific size" where the size is the size of the
/// signed integer.
///
/// If [`Repr`] is a signed integer, `Self` must have the size of the signed
/// integer, and `Self` must never have uninitialized bytes. This is why
/// [`Scalar`] is unsafe to implement.
///
/// Let's copy the above "Simple implementation" example then replace [`Repr`]
/// with `i32`, because our type is a wrapper around `f32`:
///
/// ```
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
/// #
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
/// Because `Foo::Repr` is `i32`, math types of `Foo` will have SIMD alignment
/// appropriate for `Foo`'s size, 4 bytes.
///
/// Let's implement [`Add`] for `Foo`:
///
/// ```
/// # use core::ops::Add;
/// #
/// # #[repr(transparent)]
/// # #[derive(Debug, Clone, Copy, PartialEq)]
/// # struct Foo(f32);
/// #
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
/// # use core::ops::Add;
/// #
/// # use ggmath::{Alignment, Length, Scalar, ScalarBackend, SupportedLength, Vec2};
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
/// let a = Vec2::new(Foo(1.0), Foo(2.0));
/// let b = Vec2::new(Foo(3.0), Foo(4.0));
///
/// assert_eq!(a + b, Vec2::new(Foo(1.0 + 3.0), Foo(2.0 + 4.0)));
/// ```
///
/// Currently `Foo` vector addition doesn't use SIMD. To fix this let's override
/// the implementation for [`ScalarBackend::vector_add`] which controls vector
/// addition:
///
/// ```
/// # use core::ops::Add;
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
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
///     #[inline]
///     fn vector_add(
///         vector: Vector<N, Self, A>,
///         rhs: Vector<N, Self, A>,
///     ) -> Vector<N, Self, A> {
///         unimplemented!()
///     }
/// }
/// ```
///
/// To implement SIMD-backed addition, we need to use `f32` vector addition
/// which already has SIMD optimizations.
///
/// [`Vector::to_repr`] reinterprets the bits of compatible vector types, in our
/// case `f32` and `Foo` vectors. The function is unsafe because the input
/// vector might contain bit patterns that are not accepted by the output type.
/// In our case both `f32` and `Foo` accept all bit patterns so all conversions
/// are safe.
///
/// Let's implement extension methods for `Foo` and `f32` vectors to convert
/// between the two without repeating `unsafe`:
///
/// ```
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
/// # impl<const N: usize, A: Alignment> ScalarBackend<N, A> for Foo
/// # where
/// #     Length<N>: SupportedLength,
/// # {
/// # }
/// #
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
///         // SAFETY: `f32` accepts all bit patterns.
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
///         // SAFETY: `Foo` accepts all bit patterns.
///         unsafe { self.to_repr() }
///     }
/// }
/// ```
///
/// Finally let's implement `vector_add` using these methods:
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
///     fn vector_add(
///         vector: Vector<N, Self, A>,
///         rhs: Vector<N, Self, A>,
///     ) -> Vector<N, Self, A> {
///         (vector.to_f32() + rhs.to_f32()).to_foo()
///     }
/// }
/// ```
///
/// `Foo` vector addition now recycles `f32` vector addition, which has SIMD
/// optimizations. This pattern of recycling functionality of primitives can be
/// used to optimize additional functionality with SIMD.
///
/// [`Repr`]: Self::Repr
pub unsafe trait Scalar:
    Copy
    + ScalarBackend<2, Aligned>
    + ScalarBackend<3, Aligned>
    + ScalarBackend<4, Aligned>
    + ScalarBackend<2, Unaligned>
    + ScalarBackend<3, Unaligned>
    + ScalarBackend<4, Unaligned>
{
    /// Controls SIMD alignment.
    ///
    /// `Repr` can either be `()` for "disable SIMD alignment", or a signed
    /// integer for "enable SIMD alignment appropriate for a specific size"
    /// where the size is the size of the signed integer.
    ///
    /// If `Repr` is a signed integer, `Self` must have the size of the signed
    /// integer, and `Self` must never have uninitialized bytes. This is why
    /// [`Scalar`] is unsafe to implement.
    ///
    /// See [`Scalar`] "Making SIMD optimizations" for example usage.
    #[expect(private_bounds)]
    type Repr: ScalarRepr;
}

/// Controls the implementation of math functions.
///
/// `ScalarBackend<N, A>` controls the function implementations of math types
/// with length `N`, scalar type `Self` and alignment `A`.
///
/// All `ScalarBackend<N, A>` functions have a default implementation that can
/// be overriden to make SIMD optimizations.
///
/// You should only override implementations to make optimizations. Generally
/// implementations should be consistent with the behaviour of default
/// implementations.
///
/// See [`Scalar`] "Making SIMD optimizations" for example usage.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is missing an implementation for `ScalarBackend`",
    note = "see the documentation for `Scalar`"
)]
pub trait ScalarBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    /// Overridable implementation for the `vector == vector` operation.
    #[inline]
    fn vector_eq(vector: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        (0..N).all(|i| vector[i] == other[i])
    }

    /// Overridable implementation for the `vector != vector` operation.
    #[inline]
    fn vector_ne(vector: &Vector<N, Self, A>, other: &Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        !Self::vector_eq(vector, other)
    }

    /// Overridable implementation for the unary `-vector` operation.
    #[inline]
    #[track_caller]
    fn vector_neg(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Neg<Output = Self>,
    {
        vector.map(Self::neg)
    }

    /// Overridable implementation for the unary `!vector` operation.
    #[inline]
    #[track_caller]
    fn vector_not(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Not<Output = Self>,
    {
        vector.map(Self::not)
    }

    /// Overridable implementation for the `vector + vector` operation.
    #[inline]
    #[track_caller]
    fn vector_add(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Add<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] + rhs[i])
    }

    /// Overridable implementation for the `vector - vector` operation.
    #[inline]
    #[track_caller]
    fn vector_sub(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Sub<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] - rhs[i])
    }

    /// Overridable implementation for the `vector * vector` operation.
    #[inline]
    #[track_caller]
    fn vector_mul(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Mul<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] * rhs[i])
    }

    /// Overridable implementation for the `vector / vector` operation.
    #[inline]
    #[track_caller]
    fn vector_div(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Div<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] / rhs[i])
    }

    /// Overridable implementation for the `vector % vector` operation.
    #[inline]
    #[track_caller]
    fn vector_rem(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Rem<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] % rhs[i])
    }

    /// Overridable implementation for the `vector << vector` operation.
    #[inline]
    #[track_caller]
    fn vector_shl(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shl<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] << rhs[i])
    }

    /// Overridable implementation for the `vector >> vector` operation.
    #[inline]
    #[track_caller]
    fn vector_shr(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shr<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] >> rhs[i])
    }

    /// Overridable implementation for the `vector & vector` operation.
    #[inline]
    #[track_caller]
    fn vector_bitand(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] & rhs[i])
    }

    /// Overridable implementation for the `vector | vector` operation.
    #[inline]
    #[track_caller]
    fn vector_bitor(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] | rhs[i])
    }

    /// Overridable implementation for the `vector ^ vector` operation.
    #[inline]
    #[track_caller]
    fn vector_bitxor(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vector[i] ^ rhs[i])
    }

    /// Overridable implementation for [`Vector::element_sum`].
    #[inline]
    #[track_caller]
    fn vector_element_sum(vector: Vector<N, Self, A>) -> Self
    where
        Self: Scalar + Add<Output = Self>,
    {
        match N {
            2 => vector[0] + vector[1],
            3 => vector[0] + vector[1] + vector[2],
            4 => vector[0] + vector[1] + (vector[2] + vector[3]),
            _ => unreachable!(),
        }
    }

    /// Overridable implementation for [`Vector::element_product`].
    #[inline]
    #[track_caller]
    fn vector_element_product(vector: Vector<N, Self, A>) -> Self
    where
        Self: Scalar + Mul<Output = Self>,
    {
        match N {
            2 => vector[0] * vector[1],
            3 => vector[0] * vector[1] * vector[2],
            4 => vector[0] * vector[1] * (vector[2] * vector[3]),
            _ => unreachable!(),
        }
    }

    /// Overridable implementation for [`Vector::eq_mask`].
    #[inline]
    fn vector_eq_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialEq,
    {
        Mask::from_fn(|i| vector[i] == other[i])
    }

    /// Overridable implementation for [`Vector::ne_mask`].
    #[inline]
    fn vector_ne_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialEq,
    {
        Mask::from_fn(|i| vector[i] != other[i])
    }

    /// Overridable implementation for [`Vector::lt_mask`].
    #[inline]
    fn vector_lt_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd,
    {
        Mask::from_fn(|i| vector[i] < other[i])
    }

    /// Overridable implementation for [`Vector::gt_mask`].
    #[inline]
    fn vector_gt_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd,
    {
        Mask::from_fn(|i| vector[i] > other[i])
    }

    /// Overridable implementation for [`Vector::le_mask`].
    #[inline]
    fn vector_le_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd,
    {
        Mask::from_fn(|i| vector[i] <= other[i])
    }

    /// Overridable implementation for [`Vector::ge_mask`].
    #[inline]
    fn vector_ge_mask(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar + PartialOrd,
    {
        Mask::from_fn(|i| vector[i] >= other[i])
    }

    /// Overridable implementation for the `matrix == matrix` operation.
    #[inline]
    fn matrix_eq(matrix: &Matrix<N, Self, A>, other: &Matrix<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        (0..N).all(|i| matrix.column(i) == other.column(i))
    }

    /// Overridable implementation for the `matrix != matrix` operation.
    #[inline]
    fn matrix_ne(matrix: &Matrix<N, Self, A>, other: &Matrix<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        !Self::matrix_eq(matrix, other)
    }

    /// Overridable implementation for the unary `-matrix` operation.
    #[inline]
    #[track_caller]
    fn matrix_neg(matrix: &Matrix<N, Self, A>) -> Matrix<N, Self, A>
    where
        Self: Scalar + Neg<Output = Self>,
    {
        Matrix::from_column_fn(|i| -matrix.column(i))
    }

    /// Overridable implementation for the `matrix + matrix` operation.
    #[inline]
    #[track_caller]
    fn matrix_add(matrix: &Matrix<N, Self, A>, rhs: &Matrix<N, Self, A>) -> Matrix<N, Self, A>
    where
        Self: Scalar + Add<Output = Self>,
    {
        Matrix::from_column_fn(|i| matrix.column(i) + rhs.column(i))
    }

    /// Overridable implementation for the `matrix - matrix` operation.
    #[inline]
    #[track_caller]
    fn matrix_sub(matrix: &Matrix<N, Self, A>, rhs: &Matrix<N, Self, A>) -> Matrix<N, Self, A>
    where
        Self: Scalar + Sub<Output = Self>,
    {
        Matrix::from_column_fn(|i| matrix.column(i) - rhs.column(i))
    }

    /// Overridable implementation for the `matrix * scalar` operation.
    #[inline]
    #[track_caller]
    fn matrix_mul_scalar(matrix: &Matrix<N, Self, A>, rhs: Self) -> Matrix<N, Self, A>
    where
        Self: Scalar + Mul<Output = Self>,
    {
        Matrix::from_column_fn(|i| matrix.column(i) * rhs)
    }

    /// Overridable implementation for the `matrix * vector` operation.
    #[inline]
    #[track_caller]
    fn matrix_mul_vector(matrix: &Matrix<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Add<Output = Self> + Mul<Output = Self>,
    {
        match N {
            2 => matrix.column(0) * rhs[0] + matrix.column(1) * rhs[1],
            3 => matrix.column(0) * rhs[0] + matrix.column(1) * rhs[1] + matrix.column(2) * rhs[2],
            4 => {
                matrix.column(0) * rhs[0]
                    + matrix.column(1) * rhs[1]
                    + matrix.column(2) * rhs[2]
                    + matrix.column(3) * rhs[3]
            }
            _ => unreachable!(),
        }
    }

    /// Overridable implementation for the `matrix * matrix` operation.
    #[inline]
    #[track_caller]
    fn matrix_mul(matrix: &Matrix<N, Self, A>, rhs: &Matrix<N, Self, A>) -> Matrix<N, Self, A>
    where
        Self: Scalar + Add<Output = Self> + Mul<Output = Self>,
    {
        Matrix::from_column_fn(|i| matrix * rhs.column(i))
    }

    /// Overridable implementation for the `matrix / scalar` operation.
    #[inline]
    #[track_caller]
    fn matrix_div_scalar(matrix: &Matrix<N, Self, A>, rhs: Self) -> Matrix<N, Self, A>
    where
        Self: Scalar + Div<Output = Self>,
    {
        Matrix::from_column_fn(|i| matrix.column(i) / rhs)
    }

    /// Overridable implementation for the `affine == affine` operation.
    #[inline]
    fn affine_eq(affine: &Affine<N, Self, A>, rhs: &Affine<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        affine.submatrix == rhs.submatrix && affine.translation == rhs.translation
    }

    /// Overridable implementation for the `affine != affine` operation.
    #[inline]
    fn affine_ne(affine: &Affine<N, Self, A>, rhs: &Affine<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        !Self::affine_eq(affine, rhs)
    }
}

/// A trait for types accepted by [`Scalar::Repr`].
///
/// # Safety
///
/// All associated types must uphold the guarantees of their math types.
///
/// `MaskRepr` must be either equivalent to `[bool; N]` or be a simple intrinsic
/// type.
pub(crate) unsafe trait ScalarRepr:
    MaskBackend<2, Aligned>
    + MaskBackend<3, Aligned>
    + MaskBackend<4, Aligned>
    + MaskBackend<2, Unaligned>
    + MaskBackend<3, Unaligned>
    + MaskBackend<4, Unaligned>
{
    type VectorRepr<const N: usize, T, A: Alignment>: Copy
    where
        Length<N>: SupportedLength,
        T: Scalar;

    type MatrixRepr<const N: usize, T, A: Alignment>: Copy
    where
        Length<N>: SupportedLength,
        T: Scalar;

    type AffineRepr<const N: usize, T, A: Alignment>: Copy
    where
        Length<N>: SupportedLength,
        T: Scalar;

    type MaskRepr<const N: usize, A: Alignment>: Copy
    where
        Length<N>: SupportedLength;
}

// SAFETY: `f32` and `i32` are both 4-bytes long, and `f32` does not contain
// uninitialized bytes.
unsafe impl Scalar for f32 {
    type Repr = i32;
}

// SAFETY: `f64` and `i64` are both 8-bytes long, and `f64` does not contain
// uninitialized bytes.
unsafe impl Scalar for f64 {
    type Repr = i64;
}

// SAFETY: `i8` has the same size as itself, and `i8` does not contain
// uninitialized bytes.
unsafe impl Scalar for i8 {
    type Repr = i8;
}

// SAFETY: `i16` has the same size as itself, and `i16` does not contain
// uninitialized bytes.
unsafe impl Scalar for i16 {
    type Repr = i16;
}

// SAFETY: `i32` has the same size as itself, and `i32` does not contain
// uninitialized bytes.
unsafe impl Scalar for i32 {
    type Repr = i32;
}

// SAFETY: `i64` has the same size as itself, and `i64` does not contain
// uninitialized bytes.
unsafe impl Scalar for i64 {
    type Repr = i64;
}

// SAFETY: `i128` has the same size as itself, and `i128` does not contain
// uninitialized bytes.
unsafe impl Scalar for i128 {
    type Repr = i128;
}

// SAFETY: Each representation matches the target pointer width, and `isize`
// does not contain uninitialized bytes.
unsafe impl Scalar for isize {
    #[cfg(target_pointer_width = "16")]
    type Repr = i16;

    #[cfg(target_pointer_width = "32")]
    type Repr = i32;

    #[cfg(target_pointer_width = "64")]
    type Repr = i64;
}

// SAFETY: `u8` and `i8` are both 1-byte long, and `u8` does not contain
// uninitialized bytes.
unsafe impl Scalar for u8 {
    type Repr = i8;
}

// SAFETY: `u16` and `i16` are both 2-bytes long, and `u16` does not contain
// uninitialized bytes.
unsafe impl Scalar for u16 {
    type Repr = i16;
}

// SAFETY: `u32` and `i32` are both 4-bytes long, and `u32` does not contain
// uninitialized bytes.
unsafe impl Scalar for u32 {
    type Repr = i32;
}

// SAFETY: `u64` and `i64` are both 8-bytes long, and `u64` does not contain
// uninitialized bytes.
unsafe impl Scalar for u64 {
    type Repr = i64;
}

// SAFETY: `u128` and `i128` are both 16-bytes long, and `u128` does not contain
// uninitialized bytes.
unsafe impl Scalar for u128 {
    type Repr = i128;
}

// SAFETY: Each representation matches the target pointer width, and `usize`
// does not contain uninitialized bytes.
unsafe impl Scalar for usize {
    #[cfg(target_pointer_width = "16")]
    type Repr = i16;

    #[cfg(target_pointer_width = "32")]
    type Repr = i32;

    #[cfg(target_pointer_width = "64")]
    type Repr = i64;
}

// SAFETY: `bool` and `i8` are both 1-byte long, and `bool` does not contain
// uninitialized bytes.
unsafe impl Scalar for bool {
    type Repr = i8;
}
