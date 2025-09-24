use genco::quote;

use crate::{
    constants::{BINARY_OPS, LENGTHS, UNARY_OPS},
    module::{SrcFile, TokensExt},
};

pub fn src_mod() -> SrcFile {
    quote! {
        use std::ops::*;

        use crate::{specialize, Scalar, Usize, Simdness, Simd, VecLen, Vector};

        $(
            for &op_camelcase in UNARY_OPS =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: Scalar + $op_camelcase<Output: Scalar>, S: Simdness> $op_camelcase for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, S>) -> Vector<N, T::Output, S>:

                        $(
                            for &n in LENGTHS join($['\r']) =>

                            for (Vector<$n, T, Simd>) -> Vector<$n, T::Output, Simd> {
                                |vec| T::vec$(n)_$(op_snakecase)(vec)
                            }
                        )
                        else {
                            self.map(|v| v.$op_snakecase())
                        }
                    }                    
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<Output: Scalar>, S: Simdness> $op_camelcase for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {
                    (*self).$op_snakecase()
                }
            }
        )

        $(
            for &op_camelcase in BINARY_OPS =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $op_camelcase<Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Vector<N, T2, S>) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, S>, rhs: Vector<N, T2, S>) -> Vector<N, T::Output, S>:
                        
                        $(
                            for &n in LENGTHS join($['\r']) =>
                            
                            for (Vector<$n, T, Simd>, Vector<$n, T2, Simd>) -> Vector<$n, T::Output, Simd> {
                                |vec, rhs| T::vec$(n)_$(op_snakecase)(vec, rhs)
                            }
                        )
                        else {
                            Vector::from_fn(|i| self.index(i).$op_snakecase(rhs.index(i)))
                        }
                    }
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $op_camelcase<Vector<N, T2, S>> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Vector<N, T2, S>) -> Self::Output {
                    (*self).$op_snakecase(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $op_camelcase<&Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: &Vector<N, T2, S>) -> Self::Output {
                    self.$op_snakecase(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $op_camelcase<&Vector<N, T2, S>> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: &Vector<N, T2, S>) -> Self::Output {
                    (*self).$op_snakecase(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output = T>, S: Simdness, T2: Scalar>
                $(op_camelcase)Assign<Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: Vector<N, T2, S>) {
                    *self = (*self).$op_snakecase(rhs);
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output = T>, S: Simdness, T2: Scalar>
                $(op_camelcase)Assign<&Vector<N, T2, S>> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: &Vector<N, T2, S>) {
                    self.$(op_snakecase)_assign(*rhs);
                }
            }
            
            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $op_camelcase<T2> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: T2) -> Self::Output {
                    self.$op_snakecase(Vector::<N, T2, S>::splat(rhs))
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, S: Simdness, T2: Scalar>
                $op_camelcase<T2> for &Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, S>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: T2) -> Self::Output {
                    (*self).$op_snakecase(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output = T>, S: Simdness, T2: Scalar>
                $(op_camelcase)Assign<T2> for Vector<N, T, S>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: T2) {
                    *self = (*self).$op_snakecase(rhs);
                }
            }
        )
    }.to_src_file("ops")
}
