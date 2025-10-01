use genco::quote;
use strum::IntoEnumIterator;

use crate::{module::{SrcFile, TokensExt}, iter::{BinOp, Length, UnOp}};

pub fn srcmod() -> SrcFile {
    quote! {
        use std::ops::*;

        use crate::{specialize, Scalar, Usize, Simdness, Simd, VecLen, Vector};

        $(
            for op in UnOp::iter() =>

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output: Scalar>, S: Simdness> $(op.camelcase_str()) for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, S>) -> Vector<N, T::Output, S>:

                        $(
                            for n in Length::iter() join($['\r']) =>

                            for (Vector<$n, T, Simd>) -> Vector<$n, T::Output, Simd> {
                                |vec| T::vec$(n)_$(op.lowercase_str())(vec)
                            }
                        )
                        else {
                            self.map(|v| v.$(op.lowercase_str())())
                        }
                    }                    
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output: Scalar>, S: Simdness> $(op.camelcase_str()) for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self) -> Self::Output {
                    (*self).$(op.lowercase_str())()
                }
            }
        )

        $(
            for op in BinOp::iter() =>

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())<Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: Vector<N, T2, S>) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, S>, rhs: Vector<N, T2, S>) -> Vector<N, T::Output, S>:
                        
                        $(
                            for n in Length::iter() join($['\r']) =>
                            
                            for (Vector<$n, T, Simd>, Vector<$n, T2, Simd>) -> Vector<$n, T::Output, Simd> {
                                |vec, rhs| T::vec$(n)_$(op.lowercase_str())(vec, rhs)
                            }
                        )
                        else {
                            Vector::from_fn(|i| self.index(i).$(op.lowercase_str())(rhs.index(i)))
                        }
                    }
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())<Vector<N, T2, S>> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: Vector<N, T2, S>) -> Self::Output {
                    (*self).$(op.lowercase_str())(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())<&Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: &Vector<N, T2, S>) -> Self::Output {
                    self.$(op.lowercase_str())(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())<&Vector<N, T2, S>> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: &Vector<N, T2, S>) -> Self::Output {
                    (*self).$(op.lowercase_str())(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output = T>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())Assign<Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op.lowercase_str())_assign(&mut self, rhs: Vector<N, T2, S>) {
                    *self = (*self).$(op.lowercase_str())(rhs);
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output = T>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())Assign<&Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op.lowercase_str())_assign(&mut self, rhs: &Vector<N, T2, S>) {
                    self.$(op.lowercase_str())_assign(*rhs);
                }
            }
            
            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())<T2> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: T2) -> Self::Output {
                    self.$(op.lowercase_str())(Vector::<N, T2, S>::splat(rhs))
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())<T2> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: T2) -> Self::Output {
                    (*self).$(op.lowercase_str())(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<T2, Output = T>, S: Simdness, T2: Scalar>
                $(op.camelcase_str())Assign<T2> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op.lowercase_str())_assign(&mut self, rhs: T2) {
                    *self = (*self).$(op.lowercase_str())(rhs);
                }
            }
        )
    }.to_srcfile("ops")
}
