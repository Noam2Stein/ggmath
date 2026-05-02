use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::{
    Aligned, Alignment, Length, Mask, PrimitiveFloat, PrimitiveSigned, PrimitiveUnsigned, Scalar,
    SupportedLength, Unaligned, Vector,
    utils::{Repr2, Repr3, Repr4},
};

#[cfg(not(target_feature = "sse2"))]
mod fallback;
#[cfg(target_feature = "sse2")]
mod sse2;

/// A trait for optimizing the implementation of vectors.
///
/// More specifically, [`Backend<N, A>`] controls the internal representation
/// and function implementations of vectors and vector masks of length `N` and
/// alignment `A`.
///
/// This is automatically implemented for types implementing
/// [`DefaultBackend<N, A>`]. Manual implementations should only exist to make
/// optimizations.
///
/// All associated functions have default implementations, except for
/// [`mask_from_array`] and [`mask_to_array`], because masks do not have a
/// consistent internal representation.
///
/// # Safety
///
/// The associated types [`Vector`](Backend::Vector) and [`Mask`](Backend::Mask)
/// control internal representations. See their documentation for specific
/// guarantees that implementations must uphold.
///
/// # Example
///
/// ```
/// use core::ops::Add;
///
/// use ggmath::{
///     Affine, Aligned, Backend, DefaultBackend, Length, Mask, Matrix, Scalar, SupportedLength,
///     Unaligned, Vec3, Vector,
/// };
///
/// #[repr(transparent)]
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct Foo(f32);
///
/// impl Add for Foo {
///     type Output = Self;
///
///     #[inline]
///     fn add(self, rhs: Self) -> Self::Output {
///         Self(self.0 + rhs.0)
///     }
/// }
///
/// impl Scalar for Foo {}
///
/// impl<const N: usize> DefaultBackend<N, Unaligned> for Foo {}
///
/// // SAFETY: `Foo` is a transparent wrapper over `f32`. They both accept all
/// // bit-patterns. Any internal representation used for `f32` is also valid
/// // for `Foo`.
/// unsafe impl<const N: usize> Backend<N, Aligned> for Foo
/// where
///     Length<N>: SupportedLength,
/// {
///     // This uses whatever SIMD alignment `f32` types use.
///     type Vector = Vector<N, f32, Aligned>;
///     type Mask = Mask<N, f32, Aligned>;
///
///     #[inline]
///     fn vector_add(
///         vector: Vector<N, Self, Aligned>,
///         rhs: Vector<N, Self, Aligned>,
///     ) -> Vector<N, Self, Aligned> {
///         // This uses whatever SIMD implementation `f32` vectors use.
///         Vector::from_inner(vector.inner() + rhs.inner())
///     }
///
///     // This function does not have a default implementation.
///     #[inline]
///     fn mask_from_array(array: [bool; N]) -> Mask<N, Self, Aligned> {
///         Mask::from_inner(Mask::from_array(array))
///     }
///
///     // This function does not have a default implementation.
///     #[inline]
///     fn mask_to_array(mask: Mask<N, Self, Aligned>) -> [bool; N] {
///         mask.inner().to_array()
///     }
/// }
///
/// let vector = Vec3::new(Foo(1.0), Foo(2.0), Foo(3.0));
/// let rhs = Vec3::new(Foo(4.0), Foo(5.0), Foo(6.0));
/// assert_eq!(vector + rhs, Vec3::new(Foo(5.0), Foo(7.0), Foo(9.0)));
/// ```
///
/// [`mask_from_array`]: Self::mask_from_array
/// [`mask_to_array`]: Self::mask_to_array
#[diagnostic::on_unimplemented(
    message = "`{Self}` is missing an implementation for `ggmath::Backend`",
    note = "see the documentation for `ggmath::Scalar`"
)]
pub unsafe trait Backend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    /// Controls the internal representation of [`Vector<N, Self, A>`].
    ///
    /// # Safety
    ///
    /// References to this type must be transmutable to and from references to
    /// `[T; N]`, meaning any bit-patterns accepted by `T` must be accepted by
    /// this type, and any bit-patterns accepted by this type must be accepted
    /// by `T`.
    ///
    /// For `A = Unaligned` this type must have the size and alignment of
    /// `[T; N]`.
    ///
    /// For `N = 2` and `N = 4` this type must have the size of `[T; N]` and may
    /// have additional alignment.
    ///
    /// For `N = 3, A = Aligned` this type must have the size of either `[T; 3]`
    /// or `[T; 4]` and may have additional alignment. If this type has the size
    /// of `[T; 4]` the padding must be initialized memory accept all
    /// bit-patterns.
    type Vector: Copy;

    /// Controls the internal representation of [`Mask<N, Self, A>`].
    ///
    /// # Safety
    ///
    /// This type must only have initialized memory and must accept the zero
    /// bit-pattern.
    type Mask: Send + Sync + Copy;

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

    /// Controls the implementation of [`Mask::from_array`].
    fn mask_from_array(array: [bool; N]) -> Mask<N, Self, A>
    where
        Self: Scalar;

    /// Overridable implementation for [`Mask::splat`].
    #[inline]
    fn mask_splat(value: bool) -> Mask<N, Self, A>
    where
        Self: Scalar,
    {
        Self::mask_from_array([value; N])
    }

    /// Overridable implementation for [`Mask::from_fn`].
    #[inline]
    #[track_caller]
    fn mask_from_fn<F>(f: F) -> Mask<N, Self, A>
    where
        Self: Scalar,
        F: FnMut(usize) -> bool,
    {
        Self::mask_from_array(core::array::from_fn(f))
    }

    /// Controls the implementation of [`Mask::to_array`].
    fn mask_to_array(mask: Mask<N, Self, A>) -> [bool; N]
    where
        Self: Scalar;

    /// Overridable implementation for [`Mask::all`].
    #[inline]
    fn mask_all(mask: Mask<N, Self, A>) -> bool
    where
        Self: Scalar,
    {
        match N {
            2 => mask.get(0) && mask.get(1),
            3 => mask.get(0) && mask.get(1) && mask.get(2),
            4 => mask.get(0) && mask.get(1) && mask.get(2) && mask.get(3),
            _ => unreachable!(),
        }
    }

    /// Overridable implementation for [`Mask::any`].
    #[inline]
    fn mask_any(mask: Mask<N, Self, A>) -> bool
    where
        Self: Scalar,
    {
        match N {
            2 => mask.get(0) || mask.get(1),
            3 => mask.get(0) || mask.get(1) || mask.get(2),
            4 => mask.get(0) || mask.get(1) || mask.get(2) || mask.get(3),
            _ => unreachable!(),
        }
    }

    /// Overridable implementation for [`Mask::select`].
    #[inline]
    fn mask_select(
        mask: Mask<N, Self, A>,
        if_true: Vector<N, Self, A>,
        if_false: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Scalar,
    {
        Vector::from_fn(|i| if mask.get(i) { if_true[i] } else { if_false[i] })
    }

    /// Overridable implementation for [`Mask::get`].
    #[inline]
    #[track_caller]
    fn mask_get(mask: Mask<N, Self, A>, index: usize) -> bool
    where
        Self: Scalar,
    {
        mask.to_array()[index]
    }

    /// Overridable implementation for [`Mask::set`].
    #[inline]
    #[track_caller]
    fn mask_set(mask: &mut Mask<N, Self, A>, index: usize, value: bool)
    where
        Self: Scalar,
    {
        let mut array = mask.to_array();
        array[index] = value;
        *mask = Mask::from_array(array);
    }

    /// Overridable implementation for `mask == mask`.
    #[inline]
    fn mask_eq(mask: &Mask<N, Self, A>, other: &Mask<N, Self, A>) -> bool
    where
        Self: Scalar,
    {
        mask.to_array() == other.to_array()
    }

    /// Overridable implementation for `mask != mask`.
    #[inline]
    fn mask_ne(mask: &Mask<N, Self, A>, other: &Mask<N, Self, A>) -> bool
    where
        Self: Scalar,
    {
        !Self::mask_eq(mask, other)
    }

    /// Overridable implementation for `!mask`.
    #[inline]
    fn mask_not(mask: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar,
    {
        Mask::from_fn(|i| !mask.get(i))
    }

    /// Overridable implementation for `mask & mask`.
    #[inline]
    fn mask_bitand(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar,
    {
        Mask::from_fn(|i| mask.get(i) & rhs.get(i))
    }

    /// Overridable implementation for `mask | mask`.
    #[inline]
    fn mask_bitor(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar,
    {
        Mask::from_fn(|i| mask.get(i) | rhs.get(i))
    }

    /// Overridable implementation for `mask ^ mask`.
    #[inline]
    fn mask_bitxor(mask: Mask<N, Self, A>, rhs: Mask<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: Scalar,
    {
        Mask::from_fn(|i| mask.get(i) ^ rhs.get(i))
    }
}

/// A trait for implementing [`Scalar`] without boilerplate.
///
/// Due to type system limitations, [`Scalar`] requires the [`Backend`] trait
/// which controls the internal implementation of vectors. To simply implement
/// [`Scalar`], use the [`DefaultBackend`] trait:
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
pub trait DefaultBackend<const N: usize, A: Alignment>: Scalar {}

pub(crate) trait PrimitiveFloatBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vector_nan_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vector[i].is_nan())
    }

    #[inline]
    fn vector_finite_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vector[i].is_finite())
    }

    #[inline]
    fn vector_sign_positive_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vector[i].is_sign_positive())
    }

    #[inline]
    fn vector_sign_negative_mask(vector: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vector[i].is_sign_negative())
    }

    #[inline]
    fn vector_max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| {
            if vector[i] > other[i] {
                vector[i]
            } else {
                other[i]
            }
        })
    }

    #[inline]
    fn vector_min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| {
            if vector[i] < other[i] {
                vector[i]
            } else {
                other[i]
            }
        })
    }

    #[inline]
    fn vector_max_element(vector: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveFloat,
    {
        vector
            .iter()
            .reduce(|a, b| if a > b { a } else { b })
            .unwrap()
    }

    #[inline]
    fn vector_min_element(vector: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveFloat,
    {
        vector
            .iter()
            .reduce(|a, b| if a < b { a } else { b })
            .unwrap()
    }

    #[inline]
    fn vector_abs(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::abs)
    }

    #[inline]
    fn vector_signum(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::signum)
    }

    #[inline]
    fn vector_copysign(vector: Vector<N, Self, A>, sign: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vector[i].copysign(sign[i]))
    }

    #[inline]
    fn vector_floor(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::floor)
    }

    #[inline]
    fn vector_ceil(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::ceil)
    }

    #[inline]
    fn vector_round(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::round)
    }

    #[inline]
    fn vector_trunc(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::trunc)
    }

    #[inline]
    fn vector_mul_add(
        vector: Vector<N, Self, A>,
        a: Vector<N, Self, A>,
        b: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vector[i].mul_add(a[i], b[i]))
    }

    #[inline]
    fn vector_div_euclid(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vector[i].div_euclid(rhs[i]))
    }

    #[inline]
    fn vector_rem_euclid(vector: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vector[i].rem_euclid(rhs[i]))
    }

    #[inline]
    fn vector_sqrt(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::sqrt)
    }

    #[inline]
    fn vector_sin(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::sin)
    }

    #[inline]
    fn vector_cos(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::cos)
    }

    #[inline]
    fn vector_tan(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::tan)
    }

    #[inline]
    fn vector_asin(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::asin)
    }

    #[inline]
    fn vector_acos(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::acos)
    }

    #[inline]
    fn vector_atan(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vector.map(Self::atan)
    }

    #[inline]
    fn vector_sin_cos(vector: Vector<N, Self, A>) -> (Vector<N, Self, A>, Vector<N, Self, A>)
    where
        Self: PrimitiveFloat,
    {
        let array = vector.to_array().map(|x| x.sin_cos());

        (
            Vector::from_fn(|i| array[i].0),
            Vector::from_fn(|i| array[i].1),
        )
    }
}

pub(crate) trait PrimitiveSignedBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vector_max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].max(other[i]))
    }

    #[inline]
    fn vector_min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].min(other[i]))
    }

    #[inline]
    fn vector_max_element(vector: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveSigned,
    {
        vector.iter().reduce(Self::max).unwrap()
    }

    #[inline]
    fn vector_min_element(vector: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveSigned,
    {
        vector.iter().reduce(Self::min).unwrap()
    }

    #[inline]
    #[track_caller]
    fn vector_abs(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        vector.map(Self::abs)
    }

    #[inline]
    fn vector_signum(vector: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        vector.map(Self::signum)
    }

    #[inline]
    fn vector_checked_add(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveSigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_add(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_sub(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveSigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_sub(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_mul(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveSigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_mul(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_div(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveSigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_div(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_rem(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveSigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_rem(rhs[i])?;
        }
        Some(vector)
    }

    #[inline]
    fn vector_saturating_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].saturating_add(rhs[i]))
    }

    #[inline]
    fn vector_saturating_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].saturating_sub(rhs[i]))
    }

    #[inline]
    fn vector_saturating_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].saturating_mul(rhs[i]))
    }

    #[inline]
    #[track_caller]
    fn vector_saturating_div(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].saturating_div(rhs[i]))
    }

    #[inline]
    fn vector_wrapping_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_add(rhs[i]))
    }

    #[inline]
    fn vector_wrapping_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_sub(rhs[i]))
    }

    #[inline]
    fn vector_wrapping_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_mul(rhs[i]))
    }

    #[inline]
    #[track_caller]
    fn vector_wrapping_div(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_div(rhs[i]))
    }

    #[inline]
    #[track_caller]
    fn vector_wrapping_rem(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveSigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_rem(rhs[i]))
    }
}

pub(crate) trait PrimitiveUnsignedBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vector_max(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].max(other[i]))
    }

    #[inline]
    fn vector_min(vector: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].min(other[i]))
    }

    #[inline]
    fn vector_max_element(vector: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveUnsigned,
    {
        vector.iter().reduce(Self::max).unwrap()
    }

    #[inline]
    fn vector_min_element(vector: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveUnsigned,
    {
        vector.iter().reduce(Self::min).unwrap()
    }

    #[inline]
    fn vector_checked_add(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveUnsigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_add(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_sub(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveUnsigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_sub(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_mul(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveUnsigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_mul(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_div(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveUnsigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_div(rhs[i])?;
        }

        Some(vector)
    }

    #[inline]
    fn vector_checked_rem(
        mut vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Option<Vector<N, Self, A>>
    where
        Self: PrimitiveUnsigned,
    {
        for i in 0..N {
            vector[i] = vector[i].checked_rem(rhs[i])?;
        }
        Some(vector)
    }

    #[inline]
    fn vector_saturating_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].saturating_add(rhs[i]))
    }

    #[inline]
    fn vector_saturating_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].saturating_sub(rhs[i]))
    }

    #[inline]
    fn vector_saturating_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].saturating_mul(rhs[i]))
    }

    #[inline]
    fn vector_wrapping_add(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_add(rhs[i]))
    }

    #[inline]
    fn vector_wrapping_sub(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_sub(rhs[i]))
    }

    #[inline]
    fn vector_wrapping_mul(
        vector: Vector<N, Self, A>,
        rhs: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveUnsigned,
    {
        Vector::from_fn(|i| vector[i].wrapping_mul(rhs[i]))
    }
}

impl DefaultBackend<2, Aligned> for f32 {}

impl<const N: usize> DefaultBackend<N, Unaligned> for f32 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for f64 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i8 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i16 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i32 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i64 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for i128 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for isize {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u8 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u16 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u32 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u64 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for u128 {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for usize {}

impl<const N: usize, A: Alignment> DefaultBackend<N, A> for bool {}

// SAFETY: All associated types uphold requirements.
unsafe impl<T, A: Alignment> Backend<2, A> for T
where
    T: DefaultBackend<2, A>,
{
    type Vector = Repr2<T>;
    type Mask = Repr2<bool>;

    #[inline]
    fn mask_from_array(array: [bool; 2]) -> Mask<2, Self, A> {
        Mask::from_inner(Repr2(array[0], array[1]))
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask<2, Self, A> {
        Mask::from_inner(Repr2(value, value))
    }

    #[inline]
    fn mask_from_fn<F>(mut f: F) -> Mask<2, Self, A>
    where
        F: FnMut(usize) -> bool,
    {
        Mask::from_inner(Repr2(f(0), f(1)))
    }

    #[inline]
    fn mask_to_array(mask: Mask<2, Self, A>) -> [bool; 2] {
        [mask.inner().0, mask.inner().1]
    }

    #[inline]
    fn mask_all(mask: Mask<2, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1
    }

    #[inline]
    fn mask_any(mask: Mask<2, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1
    }

    #[inline]
    fn mask_get(mask: Mask<2, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask<2, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask<2, Self, A>, other: &Mask<2, Self, A>) -> bool {
        mask.inner() == other.inner()
    }
}

// SAFETY: All associated types uphold requirements.
unsafe impl<T, A: Alignment> Backend<3, A> for T
where
    T: DefaultBackend<3, A>,
{
    type Vector = Repr3<T>;
    type Mask = Repr3<bool>;

    #[inline]
    fn mask_from_array(array: [bool; 3]) -> Mask<3, Self, A> {
        Mask::from_inner(Repr3(array[0], array[1], array[2]))
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask<3, Self, A> {
        Mask::from_inner(Repr3(value, value, value))
    }

    #[inline]
    fn mask_from_fn<F>(mut f: F) -> Mask<3, Self, A>
    where
        F: FnMut(usize) -> bool,
    {
        Mask::from_inner(Repr3(f(0), f(1), f(2)))
    }

    #[inline]
    fn mask_to_array(mask: Mask<3, Self, A>) -> [bool; 3] {
        [mask.inner().0, mask.inner().1, mask.inner().2]
    }

    #[inline]
    fn mask_all(mask: Mask<3, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1 && mask.inner().2
    }

    #[inline]
    fn mask_any(mask: Mask<3, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1 || mask.inner().2
    }

    #[inline]
    fn mask_get(mask: Mask<3, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            2 => mask.inner().2,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask<3, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            2 => mask.inner_mut().2 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask<3, Self, A>, other: &Mask<3, Self, A>) -> bool {
        mask.inner() == other.inner()
    }
}

// SAFETY: All associated types uphold requirements.
unsafe impl<T, A: Alignment> Backend<4, A> for T
where
    T: DefaultBackend<4, A>,
{
    type Vector = Repr4<T>;
    type Mask = Repr4<bool>;

    #[inline]
    fn mask_from_array(array: [bool; 4]) -> Mask<4, Self, A> {
        Mask::from_inner(Repr4(array[0], array[1], array[2], array[3]))
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask<4, Self, A> {
        Mask::from_inner(Repr4(value, value, value, value))
    }

    #[inline]
    fn mask_from_fn<F>(mut f: F) -> Mask<4, Self, A>
    where
        F: FnMut(usize) -> bool,
    {
        Mask::from_inner(Repr4(f(0), f(1), f(2), f(3)))
    }

    #[inline]
    fn mask_to_array(mask: Mask<4, Self, A>) -> [bool; 4] {
        [
            mask.inner().0,
            mask.inner().1,
            mask.inner().2,
            mask.inner().3,
        ]
    }

    #[inline]
    fn mask_all(mask: Mask<4, Self, A>) -> bool {
        mask.inner().0 && mask.inner().1 && mask.inner().2 && mask.inner().3
    }

    #[inline]
    fn mask_any(mask: Mask<4, Self, A>) -> bool {
        mask.inner().0 || mask.inner().1 || mask.inner().2 || mask.inner().3
    }

    #[inline]
    fn mask_get(mask: Mask<4, Self, A>, index: usize) -> bool {
        match index {
            0 => mask.inner().0,
            1 => mask.inner().1,
            2 => mask.inner().2,
            3 => mask.inner().3,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask<4, Self, A>, index: usize, value: bool) {
        match index {
            0 => mask.inner_mut().0 = value,
            1 => mask.inner_mut().1 = value,
            2 => mask.inner_mut().2 = value,
            3 => mask.inner_mut().3 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask<4, Self, A>, other: &Mask<4, Self, A>) -> bool {
        mask.inner() == other.inner()
    }
}

impl<const N: usize, T, A: Alignment> PrimitiveFloatBackend<N, A> for T
where
    Length<N>: SupportedLength,
    T: PrimitiveFloat + DefaultBackend<N, A>,
{
}

impl<const N: usize, T, A: Alignment> PrimitiveSignedBackend<N, A> for T
where
    Length<N>: SupportedLength,
    T: PrimitiveSigned + DefaultBackend<N, A>,
{
}

impl<const N: usize, T, A: Alignment> PrimitiveUnsignedBackend<N, A> for T
where
    Length<N>: SupportedLength,
    T: PrimitiveUnsigned + DefaultBackend<N, A>,
{
}
