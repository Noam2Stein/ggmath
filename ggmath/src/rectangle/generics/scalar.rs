use std::ops::*;

use crate::{scalar::*, vector::*};

/// Trait required to put a type inside a `Rectangle`.
pub trait RectScalar: Scalar + Add<Output = Self> + Sub<Output = Self> + PartialOrd {
    /// Returns `self * 2`.
    /// Used by `Rectangle` functions.
    fn rect_mul_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen;

    /// Returns `self / 2`.
    /// Used by `Rectangle` functions.
    ///
    /// For ints this should floor the output.
    fn rect_div_vector_by_two<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen;

    fn rect_vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen;

    fn rect_vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen;

    fn rect_vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen;
}

#[cfg(feature = "newnum")]
mod newnum_features {
    use newnum::*;

    use super::*;

    #[diagnostic::do_not_recommend]
    impl<T: Scalar + Num> RectScalar for T {
        fn rect_mul_vector_by_two<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec * num!(2: Self)
        }

        fn rect_div_vector_by_two<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec / num!(2: Self)
        }

        fn rect_vector_abs_diff<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            rhs: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.abs_diff(rhs.into_alignment())
        }

        fn rect_vector_min<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            other: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.min(other.into_alignment())
        }

        fn rect_vector_max<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            other: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.max(other.into_alignment())
        }
    }
}

#[cfg(not(feature = "newnum"))]
mod no_newnum_features {
    use super::*;

    macro_rules! int_impl {
        ($type:ty) => {
            impl RectScalar for $type {
                fn rect_mul_vector_by_two<const N: usize, A: VecAlignment>(
                    vec: Vector<N, Self, A>,
                ) -> Vector<N, Self, A>
                where
                    ScalarCount<N>: VecLen,
                {
                    vec * 2
                }

                fn rect_div_vector_by_two<const N: usize, A: VecAlignment>(
                    vec: Vector<N, Self, A>,
                ) -> Vector<N, Self, A>
                where
                    ScalarCount<N>: VecLen,
                {
                    vec / 2
                }

                fn rect_vector_abs_diff<const N: usize, A: VecAlignment>(
                    vec: Vector<N, Self, A>,
                    rhs: Vector<N, Self, impl VecAlignment>,
                ) -> Vector<N, Self, A>
                where
                    ScalarCount<N>: VecLen,
                {
                    vec.abs_diff(rhs)
                }

                fn rect_vector_min<const N: usize, A: VecAlignment>(
                    vec: Vector<N, Self, A>,
                    other: Vector<N, Self, impl VecAlignment>,
                ) -> Vector<N, Self, A>
                where
                    ScalarCount<N>: VecLen,
                {
                    vec.min(other)
                }

                fn rect_vector_max<const N: usize, A: VecAlignment>(
                    vec: Vector<N, Self, A>,
                    other: Vector<N, Self, impl VecAlignment>,
                ) -> Vector<N, Self, A>
                where
                    ScalarCount<N>: VecLen,
                {
                    vec.max(other)
                }
            }
        };
    }
    int_impl!(u8);
    int_impl!(u16);
    int_impl!(u32);
    int_impl!(u64);
    int_impl!(u128);
    int_impl!(usize);
    int_impl!(i8);
    int_impl!(i16);
    int_impl!(i32);
    int_impl!(i64);
    int_impl!(i128);
    int_impl!(isize);

    impl RectScalar for f32 {
        fn rect_mul_vector_by_two<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec * 2.0
        }

        fn rect_div_vector_by_two<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec / 2.0
        }

        fn rect_vector_abs_diff<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            rhs: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.abs_diff(rhs)
        }

        fn rect_vector_min<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            other: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.min(other)
        }

        fn rect_vector_max<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            other: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.max(other)
        }
    }
    impl RectScalar for f64 {
        fn rect_mul_vector_by_two<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec * 2.0
        }

        fn rect_div_vector_by_two<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec / 2.0
        }

        fn rect_vector_abs_diff<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            rhs: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.abs_diff(rhs)
        }

        fn rect_vector_min<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            other: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.min(other)
        }

        fn rect_vector_max<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            other: Vector<N, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.max(other)
        }
    }
}
