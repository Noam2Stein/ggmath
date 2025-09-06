use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::module::*;

pub fn write_mod(mut module: Mod) {
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
                    self.map(|x| x.{op_fn}())
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<Output: Scalar>, A: VecAlignment> {op_trait} for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self) -> Vector<N, T::Output, A> {{
                    self.map(|x| x.{op_fn}())
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
                    Vector::from_fn(|i| self[i].{op_fn}(rhs[i]))
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    Vector::from_fn(|i| self[i].{op_fn}(rhs[i]))
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    self.{op_fn}(*rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}<&Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {{
                    (*self).{op_fn}(*rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_trait}Assign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: Vector<N, T2, A2>) {{
                    for i in 0..N {{
                        self[i].{op_fn}_assign(rhs[i]);
                    }}
                }}
            }}
            
            impl<const N: usize, T: Scalar + {op_trait}Assign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> {op_trait}Assign<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: &Vector<N, T2, A2>) {{
                    self.{op_fn}_assign(*rhs)
                }}
            }}
        "#});
    }

    let impls = impls.join("\n");

    writedoc!(
        module,
        r#"
        use std::ops::*;

        use crate::{{Scalar, Usize, VecAlignment, VecLen, Vector}};

        {impls}
        "#
    )
    .unwrap();
}
