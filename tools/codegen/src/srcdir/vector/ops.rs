use genco::quote;
use strum::IntoEnumIterator;

use crate::{module::{SrcFile, TokensExt}, iter::{BinOp, UnOp}};

pub fn srcmod() -> SrcFile {
    quote! {
        use std::ops::*;

        use crate::{specialize, Scalar, Usize, Simdness, Simd, VecLen, Vector};

        $(
            for op in UnOp::iter() =>

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness> $(op.camelcase_str()) for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Self;

                #[inline(always)]
                fn $(op.lowercase_str())(self) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, S>) -> Vector<N, T, S>:

                        for (Vector<N, T, Simd>) -> Vector<N, T, Simd> {
                            |vec| T::vec_$(op.lowercase_str())(vec)
                        } else {
                            self.map(|v| v.$(op.lowercase_str())())
                        }
                    }                    
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness> $(op.camelcase_str()) for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self) -> Self::Output {
                    (*self).$(op.lowercase_str())()
                }
            }
        )

        $(
            for op in BinOp::iter() =>

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str()) for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Self;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: Self) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, S>, rhs: Vector<N, T, S>) -> Vector<N, T, S>:
                        
                        for (Vector<N, T, Simd>, Vector<N, T, Simd>) -> Vector<N, T, Simd> {
                            |vec, rhs| T::vec_$(op.lowercase_str())(vec, rhs)
                        } else {
                            Vector::from_fn(|i| self.index(i).$(op.lowercase_str())(rhs.index(i)))
                        }
                    }
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())<Vector<N, T, S>> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: Vector<N, T, S>) -> Self::Output {
                    (*self).$(op.lowercase_str())(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())<&Self> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Self;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: &Self) -> Self::Output {
                    self.$(op.lowercase_str())(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str()) for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: Self) -> Self::Output {
                    (*self).$(op.lowercase_str())(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())Assign for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op.lowercase_str())_assign(&mut self, rhs: Self) {
                    *self = (*self).$(op.lowercase_str())(rhs);
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())Assign<&Self> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op.lowercase_str())_assign(&mut self, rhs: &Self) {
                    self.$(op.lowercase_str())_assign(*rhs);
                }
            }
            
            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())<T> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Self;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: T) -> Self::Output {
                    self.$(op.lowercase_str())(Self::splat(rhs))
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())<T> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T, S>;

                #[inline(always)]
                fn $(op.lowercase_str())(self, rhs: T) -> Self::Output {
                    (*self).$(op.lowercase_str())(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $(op.camelcase_str())<Output = T>, S: Simdness>
                $(op.camelcase_str())Assign<T> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op.lowercase_str())_assign(&mut self, rhs: T) {
                    *self = (*self).$(op.lowercase_str())(rhs);
                }
            }
        )
    }.to_srcfile("ops")
}
