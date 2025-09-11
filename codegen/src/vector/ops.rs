use indoc::formatdoc;

use crate::gen_mod::*;

pub fn write_mod(module: GenModFile) {
    let mut impls = Vec::new();

    for op_trait in ["Neg", "Not"] {
        let op_fn = match op_trait {
            "Neg" => "neg",
            "Not" => "not",
            _ => unreachable!(),
        };

        impls.push(formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_trait}<Output: Scalar>, A: VecAlignment> {op_trait} for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(self)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<Output: Scalar>, A: VecAlignment> {op_trait} for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(*self)
                }}
            }}
        "#});
    }

    for op_trait in [
        "Add", "Sub", "Mul", "Div", "Rem", "Shl", "Shr", "BitAnd", "BitOr", "BitXor",
    ] {
        let op_fn = match op_trait {
            "Add" => "add",
            "Sub" => "sub",
            "Mul" => "mul",
            "Div" => "div",
            "Rem" => "rem",
            "Shl" => "shl",
            "Shr" => "shr",
            "BitAnd" => "bitand",
            "BitOr" => "bitor",
            "BitXor" => "bitxor",
            _ => unreachable!(),
        };

        impls.push(formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(self, rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(*self, rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(self, *rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<&Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(*self, *rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output = T>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: Vector<N, T2, A2>) {{
                    *self = <Self as {op_trait}<Vector<N, T2, A2>>>::{op_fn}(*self, rhs);
                }}
            }}
            
            impl<const N: usize, T: Scalar + {op_trait}<T2, Output = T>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}Assign<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: &Vector<N, T2, A2>) {{
                    *self = <Self as {op_trait}<Vector<N, T2, A2>>>::{op_fn}(*self, *rhs);
                }}
            }}
        "#});
    }

    for op_trait in ["Mul", "Div", "Rem", "Shl", "Shr"] {
        let op_fn = match op_trait {
            "Add" => "add",
            "Sub" => "sub",
            "Mul" => "mul",
            "Div" => "div",
            "Rem" => "rem",
            "Shl" => "shl",
            "Shr" => "shr",
            "BitAnd" => "bitand",
            "BitOr" => "bitor",
            "BitXor" => "bitxor",
            _ => unreachable!(),
        };

        impls.push(formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar> {op_trait}<T2> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: T2) -> Vector<N, T::Output, A> {{
                    T::vec_scalar_{op_fn}(self, rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar> {op_trait}<T2> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: T2) -> Vector<N, T::Output, A> {{
                    T::vec_scalar_{op_fn}(*self, rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output = T>, A: VecAlignment, T2: Scalar> {op_trait}Assign<T2> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: T2) {{
                    *self = <Self as {op_trait}<T2>>::{op_fn}(*self, rhs);
                }}
            }}
        "#});
    }

    let impls = impls.join("\n");

    module.finish(formatdoc! {r#"
        use core::ops::*;

        use crate::{{Scalar, Usize, VecAlignment, VecLen, Vector}};

        {impls}
    "#});
}
