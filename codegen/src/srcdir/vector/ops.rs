use genco::quote;

use crate::util::TokensExt;

pub fn generate() {
    quote! {
        $(let unary_ops_camelcase = ["Neg", "Not"])
        $(let binary_ops_camelcase = ["Add", "Sub", "Mul", "Div", "Rem", "Shl", "Shr", "BitAnd", "BitOr", "BitXor"])

        use std::ops::{
            $(for op_camelcase in unary_ops_camelcase join(, ) => $op_camelcase),
            $(for op_camelcase in binary_ops_camelcase join(, ) => $op_camelcase),
            $(for op_camelcase in binary_ops_camelcase join(, ) => $(op_camelcase)Assign),
        };

        use crate::{Vector, Scalar, Simdness, match_simdness, Simd, NonSimd};

        $(
            for op_camelcase in unary_ops_camelcase =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: $op_camelcase<Output = T> + Scalar<N>, S: Simdness> $op_camelcase for Vector<N, T, S> {
                type Output = Self;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {
                    match_simdness! {
                        for self;

                        Simd => |vec: Vector<N, T, Simd>| -> Vector<N, T, Simd> {
                            T::vec_$op_snakecase(vec)
                        },
                        NonSimd => |vec: Vector<N, T, NonSimd>| -> Vector<N, T, NonSimd> {
                            vec.map(|x| x.$op_snakecase())
                        }
                    }
                }
            }
        )

        $(
            for op_camelcase in binary_ops_camelcase =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: $op_camelcase<Output = T> + Scalar<N>, S: Simdness> $op_camelcase for Vector<N, T, S> {
                type Output = Self;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Self) -> Self::Output {
                    match_simdness! {
                        for self, rhs;

                        Simd => |vec: Vector<N, T, Simd>, rhs: Vector<N, T, Simd>| -> Vector<N, T, Simd> {
                            T::vec_$op_snakecase(vec, rhs)
                        },
                        NonSimd => |vec: Vector<N, T, NonSimd>, rhs: Vector<N, T, NonSimd>| -> Vector<N, T, NonSimd> {
                            Vector::from_fn(|i| vec[i].$op_snakecase(rhs[i]))
                        }
                    }
                }
            }
        )

        $(
            for op_camelcase in binary_ops_camelcase =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: $op_camelcase<Output = T> + Scalar<N>, S: Simdness> $(op_camelcase)Assign for Vector<N, T, S> {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: Self) {
                    *self = self.$op_snakecase(rhs);
                }
            }
        )
    }
    .write_in_src("vector/ops.rs");
}
