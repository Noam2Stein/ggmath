use genco::quote;

use crate::{
    constants::{BINARY_OPS, LENGTHS, UNARY_OPS},
    module::{ModFile, TokensExt},
};

pub fn mod_() -> ModFile {
    quote! {
        use std::ops::*;

        use crate::{specialize, Scalar, Usize, VecAligned, VecAlignment, VecLen, Vector};

        $(
            for &op_camelcase in UNARY_OPS =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: Scalar + $op_camelcase<Output: Scalar>, A: VecAlignment> $op_camelcase for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, A>) -> Vector<N, T::Output, A>:

                        $(
                            for &n in LENGTHS join($['\r']) =>

                            for (Vector<$n, T, VecAligned>) -> Vector<$n, T::Output, VecAligned> {
                                |vec| T::vec$(n)_$(op_snakecase)(vec)
                            }
                        )
                        else {
                            self.map(|v| v.$op_snakecase())
                        }
                    }                    
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<Output: Scalar>, A: VecAlignment> $op_camelcase for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {
                    (*self).$op_snakecase()
                }
            }
        )

        $(
            for &op_camelcase in BINARY_OPS =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Vector<N, T2, A2>) -> Self::Output {
                    specialize! {
                        (self: Vector<N, T, A>, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A>:
                        
                        $(
                            for &n in LENGTHS join($['\r']) =>
                            
                            for (Vector<$n, T, VecAligned>, Vector<$n, T2, VecAligned>) -> Vector<$n, T::Output, VecAligned> {
                                |vec, rhs| T::vec$(n)_$(op_snakecase)(vec, rhs)
                            }
                        )
                        else {
                            Vector::from_fn(|i| self.index(i).$op_snakecase(rhs.index(i)))
                        }
                    }
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Vector<N, T2, A2>) -> Self::Output {
                    (*self).$op_snakecase(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: &Vector<N, T2, A2>) -> Self::Output {
                    self.$op_snakecase(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $op_camelcase<&Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: &Vector<N, T2, A2>) -> Self::Output {
                    (*self).$op_snakecase(*rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output = T>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $(op_camelcase)Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: Vector<N, T2, A2>) {
                    *self = (*self).$op_snakecase(rhs);
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output = T>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                $(op_camelcase)Assign<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: &Vector<N, T2, A2>) {
                    self.$(op_snakecase)_assign(*rhs);
                }
            }
            
            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar>
                $op_camelcase<T2> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: T2) -> Self::Output {
                    self.$op_snakecase(Vector::<N, T2, A>::splat(rhs))
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output: Scalar>, A: VecAlignment, T2: Scalar>
                $op_camelcase<T2> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn $op_snakecase(self, rhs: T2) -> Self::Output {
                    (*self).$op_snakecase(rhs)
                }
            }

            impl<const N: usize, T: Scalar + $op_camelcase<T2, Output = T>, A: VecAlignment, T2: Scalar>
                $(op_camelcase)Assign<T2> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: T2) {
                    *self = (*self).$op_snakecase(rhs);
                }
            }
        )
    }.to_mod_file("ops")
}
