use crate::{
    Alignment, F64VectorBackend, I8VectorBackend, I16VectorBackend, I32VectorBackend,
    I64VectorBackend, I128VectorBackend, IsizeVectorBackend, Length, Mask, MaskBackend, Scalar,
    ScalarBackend, ScalarRepr, SupportedLength, U8VectorBackend, U16VectorBackend,
    U32VectorBackend, U64VectorBackend, U128VectorBackend, UsizeVectorBackend, Vector,
    utils::{Repr2, Repr3, Repr4},
};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for f64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for i128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for isize where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for u128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for usize where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for bool where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> F64VectorBackend<N, A> for f64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I8VectorBackend<N, A> for i8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I16VectorBackend<N, A> for i16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I32VectorBackend<N, A> for i32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I64VectorBackend<N, A> for i64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> I128VectorBackend<N, A> for i128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> IsizeVectorBackend<N, A> for isize where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> U8VectorBackend<N, A> for u8 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U16VectorBackend<N, A> for u16 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U32VectorBackend<N, A> for u32 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U64VectorBackend<N, A> for u64 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> U128VectorBackend<N, A> for u128 where Length<N>: SupportedLength {}

impl<const N: usize, A: Alignment> UsizeVectorBackend<N, A> for usize where
    Length<N>: SupportedLength
{
}

macro_rules! impl_scalar_repr {
    ($T:ty) => {
        // SAFETY: Look at the safety note for each associated type.
        unsafe impl ScalarRepr for $T {
            // SAFETY: Select chooses `ReprN` from `Repr2`, `Repr3`, and
            // `Repr4`. Each type is guaranteed to be a simple struct equivalent
            // to `[T; N]`. The vector is made out of consecutive values of `T`,
            // the vector is guaranteed to have the size and alignment of
            // `[T; N]`, and two scalars that share `$T` must have the same size
            // thus the vectors have the same size and element positions too.
            type VectorRepr<const N: usize, T, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>
            where
                Length<N>: SupportedLength,
                T: Scalar;

            // SAFETY: All types are equivalent to nested vectors making
            // `[T; N * N]`.
            type MatrixRepr<const N: usize, T, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr4<T>, Repr3<Repr3<T>>, Repr4<Repr4<T>>>
            where
                Length<N>: SupportedLength,
                T: Scalar;

            // SAFETY: Select chooses `ReprN` from `Repr2`, `Repr3`, and
            // `Repr4`. Each type is guaranteed to be a simple struct equivalent
            // to `[bool; N]`. `[bool; N]` has no uninitialized bytes, and is
            // zeroable. Masks of `$T` have the same representation no matter
            // their `T` type.
            type MaskRepr<const N: usize, A: Alignment>
                = <Length<N> as SupportedLength>::Select<Repr2<bool>, Repr3<bool>, Repr4<bool>>
            where
                Length<N>: SupportedLength;
        }
    };
}
impl_scalar_repr!(());
impl_scalar_repr!(i8);
impl_scalar_repr!(i16);
impl_scalar_repr!(i64);
impl_scalar_repr!(i128);

impl<R, A: Alignment> MaskBackend<2, A> for R
where
    R: ScalarRepr<MaskRepr<2, A> = Repr2<bool>>,
{
    #[inline]
    fn mask_from_array<T>(array: [bool; 2]) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr2(array[0], array[1]))
    }

    #[inline]
    fn mask_splat<T>(value: bool) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr2(value, value))
    }

    #[inline]
    fn mask_from_fn<T, F>((mut f,): (F,)) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool,
    {
        Mask(Repr2(f(0), f(1)))
    }

    #[inline]
    fn mask_to_array<T>(mask: Mask<2, T, A>) -> [bool; 2]
    where
        T: Scalar<Repr = Self>,
    {
        [mask.0.0, mask.0.1]
    }

    #[inline]
    fn mask_all<T>(mask: Mask<2, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 && mask.0.1
    }

    #[inline]
    fn mask_any<T>(mask: Mask<2, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 || mask.0.1
    }

    #[inline]
    fn mask_select<T>(
        mask: Mask<2, T, A>,
        if_true: Vector<2, T, A>,
        if_false: Vector<2, T, A>,
    ) -> Vector<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Vector::<2, T, A>::new(
            if mask.0.0 { if_true.x } else { if_false.x },
            if mask.0.1 { if_true.y } else { if_false.y },
        )
    }

    #[inline]
    fn mask_get<T>(mask: Mask<2, T, A>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        match index {
            0 => mask.0.0,
            1 => mask.0.1,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set<T>(mask: &mut Mask<2, T, A>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>,
    {
        match index {
            0 => mask.0.0 = value,
            1 => mask.0.1 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq<T>(mask: &Mask<2, T, A>, other: &Mask<2, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 == other.0.0 && mask.0.1 == other.0.1
    }

    #[inline]
    fn mask_not<T>(mask: Mask<2, T, A>) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr2(!mask.0.0, !mask.0.1))
    }

    #[inline]
    fn mask_bitand<T>(mask: Mask<2, T, A>, rhs: Mask<2, T, A>) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr2(mask.0.0 && rhs.0.0, mask.0.1 && rhs.0.1))
    }

    #[inline]
    fn mask_bitor<T>(mask: Mask<2, T, A>, rhs: Mask<2, T, A>) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr2(mask.0.0 || rhs.0.0, mask.0.1 || rhs.0.1))
    }

    #[inline]
    fn mask_bitxor<T>(mask: Mask<2, T, A>, rhs: Mask<2, T, A>) -> Mask<2, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr2(mask.0.0 ^ rhs.0.0, mask.0.1 ^ rhs.0.1))
    }
}

impl<R, A: Alignment> MaskBackend<3, A> for R
where
    R: ScalarRepr<MaskRepr<3, A> = Repr3<bool>>,
{
    #[inline]
    fn mask_from_array<T>(array: [bool; 3]) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr3(array[0], array[1], array[2]))
    }

    #[inline]
    fn mask_splat<T>(value: bool) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr3(value, value, value))
    }

    #[inline]
    fn mask_from_fn<T, F>((mut f,): (F,)) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool,
    {
        Mask(Repr3(f(0), f(1), f(2)))
    }

    #[inline]
    fn mask_to_array<T>(mask: Mask<3, T, A>) -> [bool; 3]
    where
        T: Scalar<Repr = Self>,
    {
        [mask.0.0, mask.0.1, mask.0.2]
    }

    #[inline]
    fn mask_all<T>(mask: Mask<3, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 && mask.0.1 && mask.0.2
    }

    #[inline]
    fn mask_any<T>(mask: Mask<3, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 || mask.0.1 || mask.0.2
    }

    #[inline]
    fn mask_select<T>(
        mask: Mask<3, T, A>,
        if_true: Vector<3, T, A>,
        if_false: Vector<3, T, A>,
    ) -> Vector<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Vector::<3, T, A>::new(
            if mask.0.0 { if_true.x } else { if_false.x },
            if mask.0.1 { if_true.y } else { if_false.y },
            if mask.0.2 { if_true.z } else { if_false.z },
        )
    }

    #[inline]
    fn mask_get<T>(mask: Mask<3, T, A>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        match index {
            0 => mask.0.0,
            1 => mask.0.1,
            2 => mask.0.2,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set<T>(mask: &mut Mask<3, T, A>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>,
    {
        match index {
            0 => mask.0.0 = value,
            1 => mask.0.1 = value,
            2 => mask.0.2 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq<T>(mask: &Mask<3, T, A>, other: &Mask<3, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 == other.0.0 && mask.0.1 == other.0.1 && mask.0.2 == other.0.2
    }

    #[inline]
    fn mask_not<T>(mask: Mask<3, T, A>) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr3(!mask.0.0, !mask.0.1, !mask.0.2))
    }

    #[inline]
    fn mask_bitand<T>(mask: Mask<3, T, A>, rhs: Mask<3, T, A>) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr3(
            mask.0.0 && rhs.0.0,
            mask.0.1 && rhs.0.1,
            mask.0.2 && rhs.0.2,
        ))
    }

    #[inline]
    fn mask_bitor<T>(mask: Mask<3, T, A>, rhs: Mask<3, T, A>) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr3(
            mask.0.0 || rhs.0.0,
            mask.0.1 || rhs.0.1,
            mask.0.2 || rhs.0.2,
        ))
    }

    #[inline]
    fn mask_bitxor<T>(mask: Mask<3, T, A>, rhs: Mask<3, T, A>) -> Mask<3, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr3(
            mask.0.0 ^ rhs.0.0,
            mask.0.1 ^ rhs.0.1,
            mask.0.2 ^ rhs.0.2,
        ))
    }
}

impl<R, A: Alignment> MaskBackend<4, A> for R
where
    R: ScalarRepr<MaskRepr<4, A> = Repr4<bool>>,
{
    #[inline]
    fn mask_from_array<T>(array: [bool; 4]) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr4(array[0], array[1], array[2], array[3]))
    }

    #[inline]
    fn mask_splat<T>(value: bool) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr4(value, value, value, value))
    }

    #[inline]
    fn mask_from_fn<T, F>((mut f,): (F,)) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool,
    {
        Mask(Repr4(f(0), f(1), f(2), f(3)))
    }

    #[inline]
    fn mask_to_array<T>(mask: Mask<4, T, A>) -> [bool; 4]
    where
        T: Scalar<Repr = Self>,
    {
        [mask.0.0, mask.0.1, mask.0.2, mask.0.3]
    }

    #[inline]
    fn mask_all<T>(mask: Mask<4, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 && mask.0.1 && mask.0.2 && mask.0.3
    }

    #[inline]
    fn mask_any<T>(mask: Mask<4, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 || mask.0.1 || mask.0.2 || mask.0.3
    }

    #[inline]
    fn mask_select<T>(
        mask: Mask<4, T, A>,
        if_true: Vector<4, T, A>,
        if_false: Vector<4, T, A>,
    ) -> Vector<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Vector::<4, T, A>::new(
            if mask.0.0 { if_true.x } else { if_false.x },
            if mask.0.1 { if_true.y } else { if_false.y },
            if mask.0.2 { if_true.z } else { if_false.z },
            if mask.0.3 { if_true.w } else { if_false.w },
        )
    }

    #[inline]
    fn mask_get<T>(mask: Mask<4, T, A>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        match index {
            0 => mask.0.0,
            1 => mask.0.1,
            2 => mask.0.2,
            3 => mask.0.3,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_set<T>(mask: &mut Mask<4, T, A>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>,
    {
        match index {
            0 => mask.0.0 = value,
            1 => mask.0.1 = value,
            2 => mask.0.2 = value,
            3 => mask.0.3 = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn mask_eq<T>(mask: &Mask<4, T, A>, other: &Mask<4, T, A>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        mask.0.0 == other.0.0
            && mask.0.1 == other.0.1
            && mask.0.2 == other.0.2
            && mask.0.3 == other.0.3
    }

    #[inline]
    fn mask_not<T>(mask: Mask<4, T, A>) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr4(!mask.0.0, !mask.0.1, !mask.0.2, !mask.0.3))
    }

    #[inline]
    fn mask_bitand<T>(mask: Mask<4, T, A>, rhs: Mask<4, T, A>) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr4(
            mask.0.0 && rhs.0.0,
            mask.0.1 && rhs.0.1,
            mask.0.2 && rhs.0.2,
            mask.0.3 && rhs.0.3,
        ))
    }

    #[inline]
    fn mask_bitor<T>(mask: Mask<4, T, A>, rhs: Mask<4, T, A>) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr4(
            mask.0.0 || rhs.0.0,
            mask.0.1 || rhs.0.1,
            mask.0.2 || rhs.0.2,
            mask.0.3 || rhs.0.3,
        ))
    }

    #[inline]
    fn mask_bitxor<T>(mask: Mask<4, T, A>, rhs: Mask<4, T, A>) -> Mask<4, T, A>
    where
        T: Scalar<Repr = Self>,
    {
        Mask(Repr4(
            mask.0.0 ^ rhs.0.0,
            mask.0.1 ^ rhs.0.1,
            mask.0.2 ^ rhs.0.2,
            mask.0.3 ^ rhs.0.3,
        ))
    }
}
