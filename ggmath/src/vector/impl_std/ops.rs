use std::ops::*;

use super::*;

macro_rules! impl_self_op {
    ($trait:ident $fn:ident) => {
        impl<const N: usize, T: Scalar + $trait<Output: Scalar>, A: VecAlignment> $trait
            for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            fn $fn(self) -> Vector<N, <T as $trait>::Output, A> {
                self.map(T::$fn)
            }
        }
    };
}
impl_self_op! { Not not }
impl_self_op! { Neg neg }

macro_rules! impl_rhs_ops {
    ($trait:ident $fn:ident) => {
        impl<
            const N: usize,
            T: Scalar + $trait<TRhs, Output: Scalar>,
            A: VecAlignment,
            TRhs: Scalar,
            ARhs: VecAlignment,
        > $trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            fn $fn(self, rhs: Vector<N, TRhs, ARhs>) -> Vector<N, T::Output, A> {
                Vector::from_fn(|i| self[i].$fn(rhs[i]))
            }
        }

        paste! {
            impl<const N: usize, T: Scalar + [<$trait Assign>]<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
                [<$trait Assign>]<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
            where
                MaybeVecLen<N>: VecLen,
            {
                fn [<$fn _assign>](&mut self, rhs: Vector<N, TRhs, ARhs>) {
                    for i in 0..N {
                        self[i].[<$fn _assign>](rhs[i]);
                    }
                }
            }
        }
    };
}
impl_rhs_ops! { Add add }
impl_rhs_ops! { Sub sub }
impl_rhs_ops! { Mul mul }
impl_rhs_ops! { Div div }
impl_rhs_ops! { Rem rem }
impl_rhs_ops! { Shl shl }
impl_rhs_ops! { Shr shr }
impl_rhs_ops! { BitAnd bitand }
impl_rhs_ops! { BitOr bitor }
impl_rhs_ops! { BitXor bitxor }

macro_rules! impl_scalar_ops {
    (
        $std_trait:ident $std_fn:ident,
        $assign_std_trait:ident $assign_std_fn:ident
    ) => {
        impl<
            const N: usize,
            T: Scalar + $std_trait<Rhs, Output: Scalar>,
            A: VecAlignment,
            Rhs: Scalar,
        > $std_trait<Rhs> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            type Output = <Self as $std_trait<Vector<N, Rhs, A>>>::Output;

            fn $std_fn(self, rhs: Rhs) -> Self::Output {
                self.$std_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }

        impl<const N: usize, T: Scalar + $assign_std_trait<Rhs>, A: VecAlignment, Rhs: Scalar>
            $assign_std_trait<Rhs> for Vector<N, T, A>
        where
            MaybeVecLen<N>: VecLen,
        {
            fn $assign_std_fn(&mut self, rhs: Rhs) {
                self.$assign_std_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }
    };
}
impl_scalar_ops!(Mul mul, MulAssign mul_assign);
impl_scalar_ops!(Div div, DivAssign div_assign);
impl_scalar_ops!(Rem rem, RemAssign rem_assign);
