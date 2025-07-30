use std::ops::*;

use super::*;

/// Trait required to put a type inside a `Aabb`.
pub trait AabbScalar:
    Scalar + Add<Output = Self> + Sub<Output = Self> + PartialEq + PartialOrd
{
    /// Returns `self * 2`.
    /// Used by `Rectangle` functions.
    fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    /// Returns `self / 2`.
    /// Used by `Rectangle` functions.
    ///
    /// For ints this should floor the output.
    fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    fn aabb_vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    fn aabb_vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;

    fn aabb_vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen;
}

macro_loop! {
    // int impl
    @for prim in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize] {
        impl AabbScalar for @prim {
            #[inline(always)]
            fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec * 2
            }

            #[inline(always)]
            fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec / 2
            }

            #[inline(always)]
            fn aabb_vector_abs_diff<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                rhs: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.abs_diff(rhs)
            }

            #[inline(always)]
            fn aabb_vector_min<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.min(other)
            }

            #[inline(always)]
            fn aabb_vector_max<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.max(other)
            }
        }
    }

    // float impl
    @for prim in [f32, f64] {
        impl AabbScalar for @prim {
            #[inline(always)]
            fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec * 2.0
            }

            #[inline(always)]
            fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec / 2.0
            }

            #[inline(always)]
            fn aabb_vector_abs_diff<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                rhs: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.abs_diff(rhs)
            }

            #[inline(always)]
            fn aabb_vector_min<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.min(other)
            }

            #[inline(always)]
            fn aabb_vector_max<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.max(other)
            }
        }
    }
}
