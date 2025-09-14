use genco::quote;

use crate::{
    constants::{COMPONENTS, DIRECTIONS_A, DIRECTIONS_B, LENGTHS},
    module::{ModFile, TokensExt},
};

pub fn mod_() -> ModFile {
    quote! {
        use core::mem::transmute_copy;

        use crate::{Usize, Scalar, VecAlignment, VecAligned, VecPacked, VecLen, Vector, $(for &n in LENGTHS join(, ) => Vec$(n))};

        /// A trait for scalar types that have a `0` value.
        ///
        /// This trait along with `ScalarOne` and `ScalarNegOne`
        /// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
        pub trait ScalarZero: Scalar {
            /// The zero value of the scalar type.
            const ZERO: Self;

            $(
                for &n in LENGTHS =>

                $(format!("/// A vec{n} of all `0`s."))
                const VEC$(n)_ZERO: Vec$(n)<Self>;
            )
        }

        /// A trait for scalar types that have a `1` value.
        ///
        /// This trait along with `ScalarZero` and `ScalarNegOne`
        /// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
        pub trait ScalarOne: ScalarZero {
            /// The one value of the scalar type.
            const ONE: Self;

            $(
                for &n in LENGTHS =>

                $(format!("/// A vec{n} of all `1`s."))
                const VEC$(n)_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n =>

                    $(format!("/// A vec{n} that points to the positive `{}` direction with magnitude `1`.", COMPONENTS[i]))
                    const VEC$(n)_$(COMPONENTS[i].to_uppercase()): Vec$(n)<Self>;
                )
            )
        }

        /// A trait for scalar types that have a `-1` value.
        ///
        /// This trait along with `ScalarZero` and `ScalarOne`
        /// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
        pub trait ScalarNegOne: ScalarZero {
            /// The negative one value of the scalar type.
            const NEG_ONE: Self;

            $(
                for &n in LENGTHS =>

                $(format!("/// A vec{n} of all `-1`s."))
                const VEC$(n)_NEG_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n =>

                    $(format!("/// A vec{n} that points to the negative `{}` direction with magnitude `1`.", COMPONENTS[i]))
                    const VEC$(n)_NEG_$(COMPONENTS[i].to_uppercase()): Vec$(n)<Self>;
                )
            )
        }

        impl<const N: usize, T: ScalarZero, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            /// A vector of all `0`s.
            pub const ZERO: Self = {
                unsafe {
                    if A::IS_ALIGNED {
                        match N {$(
                            for &n in LENGTHS =>

                            $n => transmute_copy::<Vector<$n, T, VecAligned>, Vector<N, T, A>>(T::VEC$(n)_ZERO),
                        )}
                    } else {
                        return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([T::ZERO; N]));
                    }
                }

                unreachable!("unusual vector type");
            };
        }

        impl<const N: usize, T: ScalarOne, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            /// A vector of all `1`s.
            pub const ONE: Self = {
                unsafe {
                    if A::IS_ALIGNED {
                        match N {$(
                            for &n in LENGTHS =>

                            $n => transmute_copy::<Vector<$n, T, VecAligned>, Vector<N, T, A>>(T::VEC$(n)_ONE),
                        )}
                    } else {
                        return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([T::ONE; N]));
                    }
                }

                unreachable!("unusual vector type");
            };
        }

        impl<const N: usize, T: ScalarNegOne, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            /// A vector of all `-1`s.
            pub const NEG_ONE: Self = {
                unsafe {
                    if A::IS_ALIGNED {
                        match N {$(
                            for &n in LENGTHS =>

                            $n => transmute_copy::<Vector<$n, T, VecAligned>, Vector<N, T, A>>(T::VEC$(n)_NEG_ONE),
                        )}
                    } else {
                        return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([T::NEG_ONE; N]));
                    }
                }

                unreachable!("unusual vector type");
            };
        }

        $(
            for &n in LENGTHS =>

            impl<T: ScalarOne, A: VecAlignment> Vector<$n, T, A> {$(
                for i in 0..n =>

                $(format!("/// A vector that points to the positive `{}` direction with magnitude `1`.", COMPONENTS[i]))
                pub const $(COMPONENTS[i].to_uppercase()): Self = {
                    unsafe {
                        if A::IS_ALIGNED {
                            match N {$(
                                for &n in LENGTHS =>
    
                                $n => transmute_copy::<Vector<$n, T, VecAligned>, Vector<N, T, A>>(T::VEC$(n)_$(COMPONENTS[i].to_uppercase())),
                            )}
                        } else {
                            return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([$(for i2 in 0..n join(, ) => $(if i2 == i { "T::ONE" } else { "T::ZERO" }))]));
                        }
                    }
    
                    unreachable!("unusual vector type");
                };
            )}
        )
        
        $(
            for &n in LENGTHS =>

            impl<T: ScalarNegOne, A: VecAlignment> Vector<$n, T, A> {$(
                for i in 0..n =>

                $(format!("/// A vector that points to the negative `{}` direction with magnitude `1`.", COMPONENTS[i]))
                pub const NEG_$(COMPONENTS[i].to_uppercase()): Self = {
                    unsafe {
                        if A::IS_ALIGNED {
                            match N {$(
                                for &n in LENGTHS =>
    
                                $n => transmute_copy::<Vector<$n, T, VecAligned>, Vector<N, T, A>>(T::VEC$(n)_NEG_$(COMPONENTS[i].to_uppercase())),
                            )}
                        } else {
                            return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([$(for i2 in 0..n join(, ) => $(if i2 == i { "T::NEG_ONE" } else { "T::ZERO" }))]));
                        }
                    }
    
                    unreachable!("unusual vector type");
                };
            )}
        )

        $(
            for ((&dir_a_camelcase, &dir_b_camelcase), &axis) in DIRECTIONS_A.iter().zip(DIRECTIONS_B.iter()).zip(COMPONENTS.iter()) =>

            $(let dir_a_lower = &dir_a_camelcase.to_lowercase())
            $(let dir_a_upper = &dir_a_camelcase.to_uppercase())
            $(let dir_b_lower = &dir_b_camelcase.to_lowercase())
            $(let dir_b_upper = &dir_b_camelcase.to_uppercase())

            $(format!("/// Traits with `{dir_a_upper}` and `{dir_b_upper} constants where {dir_a_lower} is the positive direction."))
            #[cfg(feature = $[str](dir_a_lower))]
            pub mod $dir_a_lower {
                use crate::{
                    Construct,
                    ScalarZero,
                    ScalarOne,
                    ScalarNegOne,
                    VecAlignment,
                    Vector,
                };

                $(format!("/// A trait for a `{dir_a_upper}` constant where {dir_a_lower} is the positive direction."))
                pub trait Positive$(dir_a_camelcase): Construct {
                    $(format!("/// A value that points {dir_a_lower} with magnitude `1` where {dir_a_lower} is the positive direction."))
                    const $dir_a_upper: Self;
                }

                $(format!("/// A trait for a `{dir_b_upper}` constant where {dir_a_lower} is the positive direction."))
                pub trait Negative$(dir_b_camelcase): Construct {
                    $(format!("/// A value that points {dir_b_lower} with magnitude `1` where {dir_a_lower} is the positive direction."))
                    const $dir_b_upper: Self;
                }

                impl<T: ScalarOne> Positive$(dir_a_camelcase) for T {
                    const $dir_a_upper: Self = Self::ONE;
                }
                
                impl<T: ScalarNegOne> Negative$(dir_b_camelcase) for T {
                    const $dir_b_upper: Self = Self::NEG_ONE;
                }

                $(
                    for &n in LENGTHS =>

                    impl<T: ScalarOne, A: VecAlignment> Positive$(dir_a_camelcase) for Vector<$n, T, A> {
                        const $dir_a_upper: Self = Self::$(axis.to_uppercase());
                    }

                    impl<T: ScalarNegOne, A: VecAlignment> Negative$(dir_b_camelcase) for Vector<$n, T, A> {
                        const $dir_b_upper: Self = Self::NEG_$(axis.to_uppercase());
                    }
                )
            }

            $(format!("/// Traits with `{dir_a_upper}` and `{dir_b_upper} constants where {dir_b_lower} is the positive direction."))
            #[cfg(feature = $[str](dir_a_lower))]
            pub mod $dir_b_lower {
                use crate::{
                    Construct,
                    ScalarZero,
                    ScalarOne,
                    ScalarNegOne,
                    VecAlignment,
                    Vector,
                };

                $(format!("/// A trait for a `{dir_a_upper}` constant where {dir_b_lower} is the positive direction."))
                pub trait Negative$(dir_a_camelcase): Construct {
                    $(format!("/// A value that points {dir_a_lower} with magnitude `1` where {dir_b_lower} is the positive direction."))
                    const $dir_a_upper: Self;
                }

                $(format!("/// A trait for a `{dir_b_upper}` constant where {dir_b_lower} is the positive direction."))
                pub trait Positive$(dir_b_camelcase): Construct {
                    $(format!("/// A value that points {dir_b_lower} with magnitude `1` where {dir_b_lower} is the positive direction."))
                    const $dir_b_upper: Self;
                }

                impl<T: ScalarNegOne> Negative$(dir_a_camelcase) for T {
                    const $dir_a_upper: Self = Self::NEG_ONE;
                }
                
                impl<T: ScalarOne> Positive$(dir_b_camelcase) for T {
                    const $dir_b_upper: Self = Self::ONE;
                }

                $(
                    for &n in LENGTHS =>

                    impl<T: ScalarNegOne, A: VecAlignment> Negative$(dir_a_camelcase) for Vector<$n, T, A> {
                        const $dir_a_upper: Self = Self::NEG_$(axis.to_uppercase());
                    }

                    impl<T: ScalarOne, A: VecAlignment> Positive$(dir_b_camelcase) for Vector<$n, T, A> {
                        const $dir_b_upper: Self = Self::$(axis.to_uppercase());
                    }
                )
            }
        )
    }
    .to_mod_file("dir")
}
