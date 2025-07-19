use std::ops::*;

use super::*;

macro_loop! {
    // Un Ops

    @for [op_trait, op_fn] in [
        [Neg, neg],
        [Not, not],
    ] {
        impl<const N: usize, T: Scalar + @op_trait<Output: Scalar>, A: VecAlignment> @op_trait
            for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            fn @op_fn(self) -> Vector<N, <T as @op_trait>::Output, A> {
                self.map(T::@op_fn)
            }
        }
    }

    // Bin Ops, Assign Ops

    @for [op_trait, op_fn] in [
        [Add, add],
        [Sub, sub],
        [Mul, mul],
        [Div, div],
        [Rem, rem],
        [BitAnd, bitand],
        [BitOr, bitor],
        [BitXor, bitxor],
        [Shl, shl],
        [Shr, shr],
    ] {
        // Bin Ops

        impl<
            const N: usize,
            T: Scalar + @op_trait<TRhs, Output: Scalar>,
            A: VecAlignment,
            TRhs: Scalar,
            ARhs: VecAlignment,
        > @op_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            fn @op_fn(self, rhs: Vector<N, TRhs, ARhs>) -> Vector<N, <T as @op_trait<TRhs>>::Output, A> {
                self.map_rhs(rhs, |self_, rhs|T::@op_fn(self_, rhs))
            }
        }

        // Assign Ops

        @let op_assign_trait = @op_trait + Assign;
        @let op_assign_fn = @op_fn + _assign;

        impl<const N: usize, T: Scalar + @op_assign_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        @op_assign_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            fn @op_assign_fn(&mut self, rhs: Vector<N, TRhs, ARhs>) {
                for i in 0..N {
                    self[i].@op_assign_fn(rhs[i]);
                }
            }
        }
    }

    // Scalar Ops

    @for [op_trait, op_fn] in [
        [Mul, mul],
        [Div, div],
        [Rem, rem],
    ] {
        impl<
            const N: usize,
            T: Scalar + @op_trait<Rhs, Output: Scalar>,
            A: VecAlignment,
            Rhs: Scalar,
        > @op_trait<Rhs> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            type Output = <Self as @op_trait<Vector<N, Rhs, A>>>::Output;

            fn @op_fn(self, rhs: Rhs) -> Self::Output {
                self.@op_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }

        @let op_assign_trait = @op_trait + Assign;
        @let op_assign_fn = @op_fn + _assign;

        impl<const N: usize, T: Scalar + @op_assign_trait<Rhs>, A: VecAlignment, Rhs: Scalar>
            @op_assign_trait<Rhs> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            fn @op_assign_fn(&mut self, rhs: Rhs) {
                self.@op_assign_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }
    }
}
