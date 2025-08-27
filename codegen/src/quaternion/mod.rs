use super::*;

mod construct;
mod convert;

pub fn write_mod(mut module: ModDir) {
    construct::write_mod(module.submod("construct"));
    convert::write_mod(module.submod("convert"));

    writedoc!(
        module,
        r#"
        //! Module for the quaternion type.
        
        use std::{{
            fmt::{{Display, Debug}},
            hash::{{Hash, Hasher}},
        }};
        
        use super::*;

        mod construct;
        mod convert;

        /// The quaternion type.
        ///
        /// In most cases you can use this type's type aliases instead.
        /// See in [`crate::quaternion`].
        ///
        /// This type is generic over scalar type and alignment,
        /// which follows the generics of [`Vector`].
        #[repr(transparent)]
        pub struct Quaternion<T: Scalar, A: VecAlignment> {{
            inner: Vector<4, T, A>,
        }}

        /// Type alias to [`Quaternion<T, VecAligned>`].
        pub type Quat<T> = Quaternion<T, VecAligned>;

        /// Type alias to [`Quaternion<T, VecPacked>`].
        pub type QuatP<T> = Quaternion<T, VecPacked>;

        impl<T: Scalar, A: VecAlignment> Clone for Quaternion<T, A> {{
            fn clone(&self) -> Self {{
                *self
            }}
        }}
        impl<T: Scalar, A: VecAlignment> Copy for Quaternion<T, A> {{}}

        impl<T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
            PartialEq<Quaternion<T2, A2>> for Quaternion<T, A>
        {{
            #[inline(always)]
            fn eq(&self, other: &Quaternion<T2, A2>) -> bool {{
                self.inner == other.inner
            }}
        }}
        impl<T: Scalar + Eq, A: VecAlignment> Eq for Quaternion<T, A> {{}}


        impl<T: Scalar + Hash, A: VecAlignment> Hash for Quaternion<T, A> {{
            fn hash<H: Hasher>(&self, state: &mut H) {{
                self.inner.hash(state);
            }}
        }}

        impl<T: Scalar + Debug, A: VecAlignment> Debug for Quaternion<T, A> {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(f, "{{:?}}", self.inner)
            }}
        }}
        
        impl<T: Scalar + Display, A: VecAlignment> Display for Quaternion<T, A> {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(f, "{{}}", self.inner)
            }}
        }}
        "#
    )
    .unwrap();
}
