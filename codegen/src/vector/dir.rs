use genco::{quote, tokens::quoted};

use crate::{
    constants::{COMPONENTS, DIRECTIONS_A, DIRECTIONS_B, LENGTHS},
    module::{SrcFile, TokensExt},
};

pub fn src_mod() -> SrcFile {
    quote! {
        use core::mem::transmute_copy;

        use crate::{Usize, Scalar, Simdness, Simd, NonSimd, VecLen, Vector, $(for &n in LENGTHS join(, ) => Vec$(n))};

        $("/// A trait for scalar types that have a `0` value.")
        $("///")
        $("/// This trait along with `ScalarOne` and `ScalarNegOne`")
        $("/// automatically enables direction constants like `RIGHT`, `UP`, and `FORWARD` if direction cargo features are enabled.")
        pub trait ScalarZero: Scalar {
            $("/// `0`")
            const ZERO: Self;

            $(
                for &n in LENGTHS join($['\r']) =>

                $(format!("/// A vec{n} of all `0`s."))
                $("/// This only exists because Rust const traits aren't stable yet.")
                const VEC$(n)_ZERO: Vec$(n)<Self>;
            )
        }

        $("/// A trait for scalar types that have a `1` value.")
        $("///")
        $("/// This trait along with `ScalarZero` and `ScalarNegOne`")
        $("/// automatically enables direction constants like `RIGHT`, `UP`, and `FORWARD` if direction cargo features are enabled.")
        pub trait ScalarOne: ScalarZero {
            $("/// `1`")
            const ONE: Self;

            $(
                for &n in LENGTHS join($['\n']) =>

                $(format!("/// A vec{n} of all `1`s."))
                $("/// This only exists because Rust const traits aren't stable yet.")
                const VEC$(n)_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n join($['\r']) =>

                    $(format!("/// A vec{n} that points to the positive `{}` direction with magnitude `1`.", COMPONENTS[i]))
                    $("/// This only exists because Rust const traits aren't stable yet.")
                    const VEC$(n)_$(COMPONENTS[i].to_uppercase()): Vec$(n)<Self>;
                )
            )
        }

        $("/// A trait for scalar types that have a `-1` value.")
        $("///")
        $("/// This trait along with `ScalarZero` and `ScalarOne`")
        $("/// automatically enables direction constants like `RIGHT`, `UP`, and `FORWARD` if direction cargo features are enabled.")
        pub trait ScalarNegOne: ScalarZero {
            $("/// `-1`")
            const NEG_ONE: Self;

            $(
                for &n in LENGTHS join($['\n']) =>

                $(format!("/// A vec{n} of all `-1`s."))
                $("/// This only exists because Rust const traits aren't stable yet.")
                const VEC$(n)_NEG_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n join($['\r']) =>

                    $(format!("/// A vec{n} that points to the negative `{}` direction with magnitude `1`.", COMPONENTS[i]))
                    $("/// This only exists because Rust const traits aren't stable yet.")
                    const VEC$(n)_NEG_$(COMPONENTS[i].to_uppercase()): Vec$(n)<Self>;
                )
            )
        }

        impl<const N: usize, T: ScalarZero, S: Simdness> Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            $("/// All `0`.")
            pub const ZERO: Self = {
                unsafe {
                    if S::IS_SIMD {
                        match N {
                            $(
                                for &n in LENGTHS join($['\r']) =>

                                $n => transmute_copy::<Vector<$n, T, Simd>, Vector<N, T, S>>(&T::VEC$(n)_ZERO),
                            )
                            _ => panic!("unusual vector type"),
                        }
                    } else {
                        transmute_copy::<Vector<N, T, NonSimd>, Vector<N, T, S>>(&Vector([T::ZERO; N]))
                    }
                }
            };
        }

        impl<const N: usize, T: ScalarOne, S: Simdness> Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            $("/// All `1`.")
            pub const ONE: Self = {
                unsafe {
                    if S::IS_SIMD {
                        match N {
                            $(
                                for &n in LENGTHS join($['\r']) =>

                                $n => transmute_copy::<Vector<$n, T, Simd>, Vector<N, T, S>>(&T::VEC$(n)_ONE),
                            )
                            _ => panic!("unusual vector type"),
                        }
                    } else {
                        transmute_copy::<Vector<N, T, NonSimd>, Vector<N, T, S>>(&Vector([T::ONE; N]))
                    }
                }
            };
        }

        impl<const N: usize, T: ScalarNegOne, S: Simdness> Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            $("/// All `-1`.")
            pub const NEG_ONE: Self = {
                unsafe {
                    if S::IS_SIMD {
                        match N {
                            $(
                                for &n in LENGTHS join($['\r']) =>

                                $n => transmute_copy::<Vector<$n, T, Simd>, Vector<N, T, S>>(&T::VEC$(n)_NEG_ONE),
                            )
                            _ => panic!("unusual vector type"),
                        }
                    } else {
                        transmute_copy::<Vector<N, T, NonSimd>, Vector<N, T, S>>(&Vector([T::NEG_ONE; N]))
                    }
                }
            };
        }

        $(
            for &n in LENGTHS join($['\n']) =>

            impl<T: ScalarOne, S: Simdness> Vector<$n, T, S> {$(
                for i in 0..n join($['\n']) =>

                $(format!("/// A vector that points to the positive `{}` direction with magnitude `1`.", COMPONENTS[i]))
                pub const $(COMPONENTS[i].to_uppercase()): Self = {
                    unsafe {
                        if S::IS_SIMD {
                            transmute_copy::<Vector<$n, T, Simd>, Vector<$n, T, S>>(&T::VEC$(n)_$(COMPONENTS[i].to_uppercase()))
                        } else {
                            transmute_copy::<Vector<$n, T, NonSimd>, Vector<$n, T, S>>(&Vector([$(for i2 in 0..n join(, ) => $(if i2 == i { T::ONE } else { T::ZERO }))]))
                        }
                    }
                };
            )}
        )
        
        $(
            for &n in LENGTHS join($['\n']) =>

            impl<T: ScalarNegOne, S: Simdness> Vector<$n, T, S> {$(
                for i in 0..n join($['\n']) =>

                $(format!("/// A vector that points to the negative `{}` direction with magnitude `1`.", COMPONENTS[i]))
                pub const NEG_$(COMPONENTS[i].to_uppercase()): Self = {
                    unsafe {
                        if S::IS_SIMD {
                            transmute_copy::<Vector<$n, T, Simd>, Vector<$n, T, S>>(&T::VEC$(n)_NEG_$(COMPONENTS[i].to_uppercase()))
                        } else {
                            transmute_copy::<Vector<$n, T, NonSimd>, Vector<$n, T, S>>(&Vector([$(for i2 in 0..n join(, ) => $(if i2 == i { T::NEG_ONE } else { T::ZERO }))]))
                        }
                    }
                };
            )}
        )

        $(
            for (((&dir_a_camelcase, &dir_b_camelcase), &axis), axis_index) in DIRECTIONS_A.iter().zip(DIRECTIONS_B.iter()).zip(COMPONENTS.iter()).zip(0..COMPONENTS.len()) join($['\n']) =>

            $(let dir_a_lower = &dir_a_camelcase.to_lowercase())
            $(let dir_a_upper = &dir_a_camelcase.to_uppercase())
            $(let dir_b_lower = &dir_b_camelcase.to_lowercase())
            $(let dir_b_upper = &dir_b_camelcase.to_uppercase())

            $(format!("/// `{dir_a_upper}` and `{dir_b_upper} constants where {dir_a_lower} is positive and {dir_b_lower} is negative."))
            #[cfg(feature = $(quoted(dir_a_lower)))]
            pub mod $dir_a_lower {
                use crate::{
                    Construct,
                    ScalarOne,
                    ScalarNegOne,
                    Simdness,
                    Vector,
                };

                $(format!("/// `{dir_a_upper}` constant where {dir_a_lower} is positive and {dir_b_lower} is negative."))
                pub trait Positive$(dir_a_camelcase): Construct {
                    $(format!("/// A value that points {dir_a_lower} with a magnitude of one,"))
                    $(format!("/// where {dir_a_lower} is positive and {dir_b_lower} is negative."))
                    const $dir_a_upper: Self;
                }

                $(format!("/// `{dir_b_upper}` constant where {dir_a_lower} is positive and {dir_b_lower} is negative."))
                pub trait Negative$(dir_b_camelcase): Construct {
                    $(format!("/// A value that points {dir_b_lower} with a magnitude of one,"))
                    $(format!("/// where {dir_a_lower} is positive and {dir_b_lower} is negative."))
                    const $dir_b_upper: Self;
                }

                impl<T: ScalarOne> Positive$(dir_a_camelcase) for T {
                    const $dir_a_upper: Self = Self::ONE;
                }
                
                impl<T: ScalarNegOne> Negative$(dir_b_camelcase) for T {
                    const $dir_b_upper: Self = Self::NEG_ONE;
                }

                $(
                    for &n in LENGTHS.iter().filter(|&&n| n > axis_index) join($['\n']) =>

                    impl<T: ScalarOne, S: Simdness> Positive$(dir_a_camelcase) for Vector<$n, T, S> {
                        const $dir_a_upper: Self = Self::$(axis.to_uppercase());
                    }

                    impl<T: ScalarNegOne, S: Simdness> Negative$(dir_b_camelcase) for Vector<$n, T, S> {
                        const $dir_b_upper: Self = Self::NEG_$(axis.to_uppercase());
                    }
                )
            }

            $(format!("/// `{dir_a_upper}` and `{dir_b_upper} constants where {dir_b_lower} is positive and {dir_a_lower} is negative."))
            #[cfg(feature = $(quoted(dir_b_lower)))]
            pub mod $dir_b_lower {
                use crate::{
                    Construct,
                    ScalarOne,
                    ScalarNegOne,
                    Simdness,
                    Vector,
                };

                $(format!("/// `{dir_a_upper}` constant where {dir_b_lower} is positive and {dir_a_lower} is negative."))
                pub trait Negative$(dir_a_camelcase): Construct {
                    $(format!("/// A value that points {dir_a_lower} with a magnitude of one,"))
                    $(format!("/// where {dir_b_lower} is positive and {dir_a_lower} is negative."))
                    const $dir_a_upper: Self;
                }

                $(format!("/// `{dir_b_upper}` constant where {dir_b_lower} is positive and {dir_a_lower} is negative."))
                pub trait Positive$(dir_b_camelcase): Construct {
                    $(format!("/// A value that points {dir_b_lower} with a magnitude of one,"))
                    $(format!("/// where {dir_b_lower} is positive and {dir_a_lower} is negative."))
                    const $dir_b_upper: Self;
                }

                impl<T: ScalarNegOne> Negative$(dir_a_camelcase) for T {
                    const $dir_a_upper: Self = Self::NEG_ONE;
                }
                
                impl<T: ScalarOne> Positive$(dir_b_camelcase) for T {
                    const $dir_b_upper: Self = Self::ONE;
                }

                $(
                    for &n in LENGTHS.iter().filter(|&&n| n > axis_index) join($['\n']) =>

                    impl<T: ScalarNegOne, S: Simdness> Negative$(dir_a_camelcase) for Vector<$n, T, S> {
                        const $dir_a_upper: Self = Self::NEG_$(axis.to_uppercase());
                    }

                    impl<T: ScalarOne, S: Simdness> Positive$(dir_b_camelcase) for Vector<$n, T, S> {
                        const $dir_b_upper: Self = Self::$(axis.to_uppercase());
                    }
                )
            }
        )
    }
    .to_src_file("dir")
}
