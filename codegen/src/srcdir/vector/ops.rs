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

        use crate::{Vector, ElementOfVector, Simdness};

        $(
            for op_camelcase in unary_ops_camelcase =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: $op_camelcase<Output = T> + ElementOfVector<N, S>, S: Simdness> $op_camelcase for Vector<N, T, S> {
                type Output = Self;

                #[inline(always)]
                fn $op_snakecase(self) -> Self::Output {
                    T::vec_$op_snakecase(self)
                }
            }
        )

        $(
            for op_camelcase in binary_ops_camelcase =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: $op_camelcase<Output = T> + ElementOfVector<N, S>, S: Simdness> $op_camelcase for Vector<N, T, S> {
                type Output = Self;

                #[inline(always)]
                fn $op_snakecase(self, rhs: Self) -> Self::Output {
                    T::vec_$op_snakecase(self, rhs)
                }
            }
        )

        $(
            for op_camelcase in binary_ops_camelcase =>

            $(let op_snakecase = &op_camelcase.to_lowercase())

            impl<const N: usize, T: $op_camelcase<Output = T> + ElementOfVector<N, S>, S: Simdness> $(op_camelcase)Assign for Vector<N, T, S> {
                #[inline(always)]
                fn $(op_snakecase)_assign(&mut self, rhs: Self) {
                    *self = T::vec_$op_snakecase(*self, rhs);
                }
            }
        )
    }
    .write_in_src("vector/ops.rs");
}
