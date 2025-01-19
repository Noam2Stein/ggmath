use std::ops::*;

use super::*;

use ggmath_proc_macros::{for_assign_ops, for_rhs_ops, for_self_ops};

for_self_ops!($(
    impl<const N: usize, T: Scalar + $std_trait<Output: Scalar>, A: VecAlignment>
        $std_trait for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen
    {
        type Output = Vector<N, T::Output, A>;

        fn $std_fn(self) -> Vector<N, <T as $std_trait>::Output, A> {
            T::$scalar_fn(self)
        }
    }
)*);

for_rhs_ops!($(
    impl<const N: usize, T: Scalar + $std_trait<TRhs, Output: Scalar>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        $std_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen,
    {
        type Output = Vector<N, T::Output, A>;

        fn $std_fn(self, rhs: Vector<N, TRhs, ARhs>) -> Vector<N, T::Output, A> {
            T::$scalar_fn(self, rhs)
        }
    }
)*);

for_assign_ops!($(
    impl<const N: usize, T: Scalar + $std_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        $std_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen,
    {
        fn $std_fn(&mut self, rhs: Vector<N, TRhs, ARhs>) {
            T::$scalar_fn(self, rhs)
        }
    }
)*);

scalar_defaults_macro! {
    scalar_defaults_vector_ops:

    // Vector: Self Ops

    fn vector_neg<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        vec.map(|c| c.neg())
    }

    fn vector_not<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        vec.map(|c| c.not())
    }

    // Vector: Rhs Ops

    fn vector_add<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Add<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].add(rhs[i]))
    }

    fn vector_bitand<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitAnd<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitand(rhs[i]))
    }

    fn vector_bitor<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitOr<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitor(rhs[i]))
    }

    fn vector_bitxor<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitXor<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitxor(rhs[i]))
    }

    fn vector_div<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Div<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].div(rhs[i]))
    }

    fn vector_mul<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Mul<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].mul(rhs[i]))
    }

    fn vector_rem<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Rem<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].rem(rhs[i]))
    }

    fn vector_shl<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Shl<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shl(rhs[i]))
    }

    fn vector_shr<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Shr<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shr(rhs[i]))
    }

    fn vector_sub<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Sub<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].sub(rhs[i]))
    }

    // Vector: Assign Ops

    fn vector_add_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: AddAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].add_assign(rhs[i]);
        }
    }

    fn vector_bitand_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitAndAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitand_assign(rhs[i]);
        }
    }

    fn vector_bitor_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitOrAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitor_assign(rhs[i]);
        }
    }

    fn vector_bitxor_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitXorAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitxor_assign(rhs[i]);
        }
    }

    fn vector_div_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: DivAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].div_assign(rhs[i]);
        }
    }

    fn vector_mul_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: MulAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].mul_assign(rhs[i]);
        }
    }

    fn vector_rem_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: RemAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].rem_assign(rhs[i]);
        }
    }

    fn vector_shl_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: ShlAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].shl_assign(rhs[i]);
        }
    }

    fn vector_shr_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: ShrAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].shr_assign(rhs[i]);
        }
    }

    fn vector_sub_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: SubAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].sub_assign(rhs[i]);
        }
    }
}
