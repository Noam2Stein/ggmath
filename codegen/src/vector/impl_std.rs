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
                        self: Vector<N, T, A> => Vector<{n}, T, VecAligned>,
                        other: Vector<N, T2, A2> => Vector<{n}, T2, VecAligned>,
                    ) -> bool => bool {{
                        |(vec, other)| T::vec{n}_eq(vec, other)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t\t");

    let ne_aligned_returns = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                return_for_types! {{
                    (
                        self: Vector<N, T, A> => Vector<{n}, T, VecAligned>,
                        other: Vector<N, T2, A2> => Vector<{n}, T2, VecAligned>,
                    ) -> bool => bool {{
                        |(vec, other)| T::vec{n}_ne(vec, other)
                    }}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t\t");

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
    UNARY_OPS.iter().map(|&op_camelcase| {
        let op_snakecase = op_camelcase.to_lowercase();

        for n in LENGTHS {
            scalar_override_fns.push(formatdoc! {r#"
                /// Overridable implementation of `Vector::{op_snakecase}` for aligned vec{n}s.
                #[inline(always)]
                fn vec{n}_{op_snakecase}(vec: Vec{n}<Self>) -> Vec{n}<<Self as {op_camelcase}>::Output>
                where
                    Self: {op_camelcase}<Output: Scalar>,
                {{
                    vec.map(|v| v.{op_snakecase}())
                }}
            "#});
        }

        let aligned_returns = LENGTHS
            .iter()
            .map(|&n| {
                formatdoc! {r#"
                    return_for_types! {{
                        (self: Vector<N, T, A> => Vector<{n}, T, VecAligned>) -> Vector<{n}, T::Output, VecAligned> => Vector<N, T::Output, A> {{
                            |(vec,)| T::vec{n}_{op_snakecase}(vec)
                        }}
                    }}
                "#}
            })
            .collect::<Vec<_>>()
            .join("\n")
            .replace("\n", "\n\t\t");

        formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_camelcase}<Output: Scalar>, A: VecAlignment> {op_camelcase} for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_snakecase}(self) -> Self::Output {{
                    {aligned_returns}

                    self.map(|v| v.{op_snakecase}())
                }}
            }}

            impl<const N: usize, T: Scalar + {op_camelcase}<Output: Scalar>, A: VecAlignment> {op_camelcase} for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_snakecase}(self) -> Self::Output {{
                    (*self).{op_snakecase}()
                }}
            }}
        "#}
    }).collect::<Vec<_>>().join("\n")
}

fn impl_binary_ops(scalar_override_fns: &mut Vec<String>) -> String {
    BINARY_OPS.iter().map(|&op_camelcase| {
        let op_snakecase = op_camelcase.to_lowercase();

        for n in LENGTHS {
            scalar_override_fns.push(formatdoc! {r#"
                /// Overridable implementation of `Vector::{op_snakecase}` for aligned vec{n}s.
                #[inline(always)]
                fn vec{n}_{op_snakecase}<T2: Scalar>(vec: Vec{n}<Self>, other: Vec{n}<T2>) -> Vec{n}<<Self as {op_camelcase}<T2>>::Output>
                where
                    Self: {op_camelcase}<T2, Output: Scalar>,
                {{
                    Vector::from_fn(|i| vec.index(i).{op_snakecase}(other.index(i)))
                }}
            "#});
        }

        let aligned_returns = LENGTHS
            .iter()
            .map(|&n| {
                formatdoc! {r#"
                    return_for_types! {{
                        (
                            self: Vector<N, T, A> => Vector<{n}, T, VecAligned>,
                            rhs: Vector<N, T2, A2> => Vector<{n}, T2, VecAligned>,
                        ) -> Vector<{n}, T::Output, VecAligned> => Vector<N, T::Output, A> {{
                            |(vec, rhs)| T::vec{n}_{op_snakecase}(vec, rhs)
                        }}
                    }}
                "#}
            })
            .collect::<Vec<_>>()
            .join("\n")
            .replace("\n", "\n\t\t");

        formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_camelcase}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op_camelcase}<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_snakecase}(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    {aligned_returns}

                    Vector::from_fn(|i| self.index(i).{op_snakecase}(rhs.index(i)))
                }}
            }}

            impl<const N: usize, T: Scalar + {op_camelcase}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op_camelcase}<Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_snakecase}(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    (*self).{op_snakecase}(rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_camelcase}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op_camelcase}<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_snakecase}(self, rhs: &Vector<N, T2, A2>) -> Self::Output {{
                    self.{op_snakecase}(*rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_camelcase}<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op_camelcase}<&Vector<N, T2, A2>> for &Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_snakecase}(self, rhs: &Vector<N, T2, A2>) -> Self::Output {{
                    (*self).{op_snakecase}(*rhs)
                }}
            }}
        "#}
    }).collect::<Vec<_>>().join("\n")
}

fn impl_assign_ops(_scalar_override_fns: &mut Vec<String>) -> String {
    BINARY_OPS.iter().map(|&op_camelcase| {
        let op_snakecase = op_camelcase.to_lowercase();

        formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_camelcase}<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op_camelcase}Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_snakecase}_assign(&mut self, rhs: Vector<N, T2, A2>) {{
                    *self = (*self).{op_snakecase}(rhs)
                }}
            }}

            impl<const N: usize, T: Scalar + {op_camelcase}<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
                {op_camelcase}Assign<&Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_snakecase}_assign(&mut self, rhs: &Vector<N, T2, A2>) {{
                    *self = (*self).{op_snakecase}(*rhs)
                }}
            }}
        "#}
    }).collect::<Vec<_>>().join("\n")
}
