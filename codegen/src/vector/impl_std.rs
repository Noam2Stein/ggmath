use indoc::formatdoc;

use crate::constants::{BINARY_OPS, LENGTHS, UNARY_OPS};

pub fn impl_std(scalar_override_fns: &mut Vec<String>) -> String {
    let impl_base_traits = impl_base_traits(scalar_override_fns);
    let impl_eq_hash_default = impl_eq_hash_default(scalar_override_fns);
    let impl_fmt = impl_fmt(scalar_override_fns);
    let impl_unary_ops = impl_unary_ops(scalar_override_fns);
    let impl_binary_ops = impl_binary_ops(scalar_override_fns);
    let impl_assign_ops = impl_assign_ops(scalar_override_fns);

    formatdoc! {r#"
        {impl_base_traits}

        {impl_eq_hash_default}

        {impl_fmt}

        {impl_unary_ops}

        {impl_binary_ops}
        
        {impl_assign_ops}
    "#}
}

fn impl_base_traits(_scalar_override_fns: &mut Vec<String>) -> String {
    formatdoc! {r#"
        impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn clone(&self) -> Self {{
                *self
            }}
        }}

        impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {{}}

        impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            type Item = T;
            type IntoIter = <[T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {{
                self.as_array().into_iter()
            }}
        }}

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> Index<I> for Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {{
            type Output = I::Output;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {{
                &self.0[index]
            }}
        }}

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {{
                &mut self.0[index]
            }}
        }}
    "#}
}

fn impl_eq_hash_default(scalar_override_fns: &mut Vec<String>) -> String {
    for n in LENGTHS {
        scalar_override_fns.push(formatdoc! {r#"
            /// Overridable implementation of `Vector::eq` for aligned vec{n}s.
            #[inline(always)]
            fn vec{n}_eq<T2: Scalar>(vec: Vec{n}<Self>, other: Vec{n}<T2>) -> bool
            where
                Self: PartialEq<T2>,
            {{
                (0..{n}).all(|i| vec.index(i) == other.index(i))
            }}
    
            /// Overridable implementation of `Vector::ne` for aligned vec{n}s.
            #[inline(always)]
            fn vec{n}_ne<T2: Scalar>(vec: Vec{n}<Self>, other: Vec{n}<T2>) -> bool
            where
                Self: PartialEq<T2>,
            {{
                (0..{n}).any(|i| vec.index(i) != other.index(i))
            }}
        "#});
    }

    let eq_aligned_returns = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                return_for_types! {{
                    (
                        vec: Vector<N, T, A> => Vector<{n}, T, VecAligned>,
                        other: Vector<N, T2, A2> => Vector<{n}, T2, VecAligned>,
                    ) -> bool => bool {{
                        |vec, other| T::vec{n}_eq(vec, other)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t");

    let ne_aligned_returns = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                return_for_types! {{
                    (
                        vec: Vector<N, T, A> => Vector<{n}, T, VecAligned>,
                        other: Vector<N, T2, A2> => Vector<{n}, T2, VecAligned>,
                    ) -> bool => bool {{
                        |vec, other| T::vec{n}_ne(vec, other)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t");

    formatdoc! {r#"
        impl<const N: usize, T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
            PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn eq(&self, other: &Vector<N, T2, A2>) -> bool {{
                {eq_aligned_returns}

                (0..N).all(|i| self.index(i) == other.index(i))
            }}

            #[inline(always)]
            fn ne(&self, other: &Vector<N, T2, A2>) -> bool {{
                {ne_aligned_returns}

                (0..N).any(|i| self.index(i) != other.index(i))
            }}
        }}

        impl<const N: usize, T: Scalar + Eq, A: VecAlignment> Eq for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{}}

        impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {{
                self.as_array().hash(state)
            }}
        }}

        impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn default() -> Self {{
                Self::splat(T::default())
            }}
        }} 
    "#}
}

fn impl_fmt(_scalar_override_fns: &mut Vec<String>) -> String {
    formatdoc! {r#"
        impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                write!(f, "(")?;

                for i in 0..N {{
                    if i != 0 {{
                        write!(f, ", ")?;
                    }}

                    write!(f, "{{:?}}", self.index(i))?;
                }}

                write!(f, ")")?;

                Ok(())
            }}
        }}

        impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                write!(f, "(")?;

                for i in 0..N {{
                    if i != 0 {{
                        write!(f, ", ")?;
                    }}

                    write!(f, "{{}}", self.index(i))?;
                }}

                write!(f, ")")?;

                Ok(())
            }}
        }}
    "#}
}

fn impl_unary_ops(scalar_override_fns: &mut Vec<String>) -> String {
    UNARY_OPS.iter().map(|&op| {
        let op_lower = op.to_lowercase();

        for n in LENGTHS {
            scalar_override_fns.push(formatdoc! {r#"
                /// Overridable implementation of `Vector::{op_lower}` for aligned vec{n}s.
                #[inline(always)]
                fn vec{n}_{op_lower}(vec: Vec{n}<Self>) -> Vec{n}<<Self as {op}>::Output>
                where
                    Self: {op}<Output: Scalar>,
                {{
                    vec.map(|v| v.{op_lower}())
                }}
            "#});
        }

        let aligned_returns = LENGTHS
            .iter()
            .map(|&n| {
                formatdoc! {r#"
                    return_for_types! {{
                        (vec: Vector<N, T, A> => Vector<{n}, T, VecAligned>) -> Vector<{n}, T::Output, VecAligned> => Vector<N, T::Output, A> {{
                            |vec| T::vec{n}_{op_lower}(vec)
                        }}
                    }}
                "#}
            })
            .collect::<Vec<_>>()
            .join("\n")
            .replace("\n", "\n\t");

        formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op}<Output: Scalar>, A: VecAlignment> {op} for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op}(self) -> Self::Output {{
                    {aligned_returns}

                    self.map(|v| v.{op_lower}())
                }}
            }}

            impl<const N: usize, T: Scalar + {op}<Output: Scalar>, A: VecAlignment> {op} for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op}(self) -> Self::Output {{
                    (*self).{op_lower}()
                }}
            }}
        "#}
    }).collect::<Vec<_>>().join("\n")
}

fn impl_binary_ops(scalar_override_fns: &mut Vec<String>) -> String {
    BINARY_OPS.iter().map(|&op| {
        let op_lower = op.to_lowercase();

        for n in LENGTHS {
            scalar_override_fns.push(formatdoc! {r#"
                /// Overridable implementation of `Vector::{op_lower}` for aligned vec{n}s.
                #[inline(always)]
                fn vec{n}_{op_lower}<T2: Scalar>(vec: Vec{n}<Self>, other: Vec{n}<T2>) -> Vec{n}<<Self as {op}<T2>>::Output>
                where
                    Self: {op}<T2, Output: Scalar>,
                {{
                    Vector::from_fn(|i| vec.index(i).{op_lower}(other.index(i)))
                }}
            "#});
        }

        let aligned_returns = LENGTHS
            .iter()
            .map(|&n| {
                formatdoc! {r#"
                    return_for_types! {{
                        (
                            vec: Vector<N, T, A> => Vector<{n}, T, VecAligned>,
                            other: Vector<N, T2, A2> => Vector<{n}, T2, VecAligned>,
                        ) -> Vector<{n}, T::Output, VecAligned> => Vector<N, T::Output, A> {{
                            |vec, other| T::vec{n}_{op_lower}(vec, other)
                        }}
                    }}
                "#}
            })
            .collect::<Vec<_>>()
            .join("\n")
            .replace("\n", "\n\t");

        formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op}<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op}(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    {aligned_returns}

                    Vector::from_fn(|i| self.index(i).{op_lower}(rhs.index(i)))
                }}
            }}

            impl<const N: usize, T: Scalar + {op}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op}<Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op}(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    (*self).{op_lower}(rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op}<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op}(self, rhs: &Vector<N, T2, A2>) -> Self::Output {{
                    self.{op_lower}(*rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op}<&Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op}(self, rhs: &Vector<N, T2, A2>) -> Self::Output {{
                    (*self).{op_lower}(*rhs)
                }}
            }}
        "#}
    }).collect::<Vec<_>>().join("\n")
}

fn impl_assign_ops(_scalar_override_fns: &mut Vec<String>) -> String {
    BINARY_OPS.iter().map(|&op| {
        let op_lower = op.to_lowercase();

        formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op}<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op}Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op}_assign(&mut self, rhs: Vector<N, T2, A2>) {{
                    *self = (*self).{op_lower}(rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op}<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op}Assign<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op}_assign(&mut self, rhs: &Vector<N, T2, A2>) {{
                    *self = (*self).{op_lower}(*rhs)
                }}
            }}
        "#}
    }).collect::<Vec<_>>().join("\n")
}
