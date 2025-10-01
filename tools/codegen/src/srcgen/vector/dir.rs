use genco::{quote, tokens::quoted};
use strum::IntoEnumIterator;

use crate::{backend::{SrcFile, TokensExt}, iter::{Axis, Direction, Length}};

pub fn srcmod() -> SrcFile {
    quote! {
        use core::mem::transmute_copy;

        use crate::{Usize, Scalar, Simdness, Simd, NonSimd, VecLen, Vector, $(for n in Length::iter() join(, ) => Vec$(n))};

        $("/// A trait for scalar types that have a `0` value.")
        $("///")
        $("/// This trait along with `ScalarOne` and `ScalarNegOne`")
        $("/// automatically enables direction constants like `RIGHT`, `UP`, and `FORWARD` if direction cargo features are enabled.")
        pub trait ScalarZero: Scalar {
            $("/// `0`")
            const ZERO: Self;

            $(
                for n in Length::iter() join($['\r']) =>

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
                for n in Length::iter() join($['\n']) =>

                $(format!("/// A vec{n} of all `1`s."))
                $("/// This only exists because Rust const traits aren't stable yet.")
                const VEC$(n)_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    $(format!("/// A vec{n} that points to the positive `{}` direction with magnitude `1`.", Axis(i).lowercase_str()))
                    $("/// This only exists because Rust const traits aren't stable yet.")
                    const VEC$(n)_$(Axis(i).uppercase_str()): Vec$(n)<Self>;
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
                for n in Length::iter() join($['\n']) =>

                $(format!("/// A vec{n} of all `-1`s."))
                $("/// This only exists because Rust const traits aren't stable yet.")
                const VEC$(n)_NEG_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    $(format!("/// A vec{n} that points to the negative `{}` direction with magnitude `1`.", Axis(i).lowercase_str()))
                    $("/// This only exists because Rust const traits aren't stable yet.")
                    const VEC$(n)_NEG_$(Axis(i).uppercase_str()): Vec$(n)<Self>;
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
                                for n in Length::iter() join($['\r']) =>

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
                                for n in Length::iter() join($['\r']) =>

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
                                for n in Length::iter() join($['\r']) =>

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
            for n in Length::iter() join($['\n']) =>

            impl<T: ScalarOne, S: Simdness> Vector<$n, T, S> {$(
                for i in 0..n.as_usize() join($['\n']) =>

                $(format!("/// A vector that points to the positive `{}` direction with magnitude `1`.", Axis(i).lowercase_str()))
                pub const $(Axis(i).uppercase_str()): Self = {
                    unsafe {
                        if S::IS_SIMD {
                            transmute_copy::<Vector<$n, T, Simd>, Vector<$n, T, S>>(&T::VEC$(n)_$(Axis(i).uppercase_str()))
                        } else {
                            transmute_copy::<Vector<$n, T, NonSimd>, Vector<$n, T, S>>(&Vector([$(for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { T::ONE } else { T::ZERO }))]))
                        }
                    }
                };
            )}
        )
        
        $(
            for n in Length::iter() join($['\n']) =>

            impl<T: ScalarNegOne, S: Simdness> Vector<$n, T, S> {$(
                for i in 0..n.as_usize() join($['\n']) =>

                $(format!("/// A vector that points to the negative `{}` direction with magnitude `1`.", Axis(i).lowercase_str()))
                pub const NEG_$(Axis(i).uppercase_str()): Self = {
                    unsafe {
                        if S::IS_SIMD {
                            transmute_copy::<Vector<$n, T, Simd>, Vector<$n, T, S>>(&T::VEC$(n)_NEG_$(Axis(i).uppercase_str()))
                        } else {
                            transmute_copy::<Vector<$n, T, NonSimd>, Vector<$n, T, S>>(&Vector([$(for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { T::NEG_ONE } else { T::ZERO }))]))
                        }
                    }
                };
            )}
        )

        $(
            for dir in Direction::iter() join($['\n']) =>

            $(format!("/// `{}` and `{} constants where {} is positive and {} is negative.", dir.uppercase_str(), dir.opposite_dir().uppercase_str(), dir.lowercase_str(), dir.opposite_dir().lowercase_str()))
            #[cfg(feature = $(quoted(dir.lowercase_str())))]
            pub mod $(dir.lowercase_str()) {
                use crate::{Construct, ScalarOne, ScalarNegOne, Simdness, Vector};

                $(format!("/// `{}` constant where {} is positive and {} is negative.", dir.uppercase_str(), dir.lowercase_str(), dir.opposite_dir().lowercase_str()))
                pub trait Positive$(dir.camelcase_str()): Construct {
                    $(format!("/// A value that points {} with a magnitude of one,", dir.lowercase_str()))
                    $(format!("/// where {} is positive and {} is negative.", dir.lowercase_str(), dir.opposite_dir().lowercase_str()))
                    const $(dir.uppercase_str()): Self;
                }

                $(format!("/// `{}` constant where {} is positive and {} is negative.", dir.opposite_dir().uppercase_str(), dir.lowercase_str(), dir.opposite_dir().lowercase_str()))
                pub trait Negative$(dir.opposite_dir().camelcase_str()): Construct {
                    $(format!("/// A value that points {} with a magnitude of one,", dir.opposite_dir().lowercase_str()))
                    $(format!("/// where {} is positive and {} is negative.", dir.lowercase_str(), dir.opposite_dir().lowercase_str()))
                    const $(dir.opposite_dir().uppercase_str()): Self;
                }

                impl<T: ScalarOne> Positive$(dir.camelcase_str()) for T {
                    const $(dir.uppercase_str()): Self = Self::ONE;
                }
                
                impl<T: ScalarNegOne> Negative$(dir.opposite_dir().camelcase_str()) for T {
                    const $(dir.opposite_dir().uppercase_str()): Self = Self::NEG_ONE;
                }

                $(
                    for n in Length::iter().filter(|n| dir.axis() < n.as_usize()) join($['\n']) =>

                    impl<T: ScalarOne, S: Simdness> Positive$(dir.camelcase_str()) for Vector<$n, T, S> {
                        const $(dir.uppercase_str()): Self = Self::$(Axis(dir.axis()).uppercase_str());
                    }

                    impl<T: ScalarNegOne, S: Simdness> Negative$(dir.opposite_dir().camelcase_str()) for Vector<$n, T, S> {
                        const $(dir.opposite_dir().uppercase_str()): Self = Self::NEG_$(Axis(dir.axis()).uppercase_str());
                    }
                )
            }
        )
    }
    .to_srcfile("dir")
}
