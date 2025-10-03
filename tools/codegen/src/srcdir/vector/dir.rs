use genco::{quote, tokens::quoted};
use strum::IntoEnumIterator;

use crate::{module::{SrcFile, TokensExt}, iter::{Axis, Direction, Length}};

pub fn srcmod() -> SrcFile {
    quote! {
        use core::mem::transmute_copy;

        use crate::{Usize, Scalar, Simdness, Simd, NonSimd, VecLen, Vector, $(for n in Length::iter() join(, ) => Vec$(n))};

        pub use crate::{scalar_zero_boilerplate, scalar_one_boilerplate, scalar_neg_one_boilerplate};

        $("/// A trait for scalar types that have a `0` value.")
        pub trait ScalarZero: Scalar {
            $("/// `0`")
            const ZERO: Self;

            $(
                for n in Length::iter() join($['\r']) =>

                $("/// This is boilerplate that is required because of stable-Rust limitations.")
                $("/// Use [`scalar_zero_boilerplate`] to automatically implement this.")
                const VEC$(n)_ZERO: Vec$(n)<Self>;
            )
        }

        $("/// A trait for scalar types that have a `1` value.")
        pub trait ScalarOne: ScalarZero {
            $("/// `1`")
            const ONE: Self;

            $(
                for n in Length::iter() join($['\n']) =>

                $("/// This is boilerplate that is required because of stable-Rust limitations.")
                $("/// Use [`scalar_one_boilerplate`] to automatically implement this.")
                const VEC$(n)_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    $("/// This is boilerplate that is required because of stable-Rust limitations.")
                    $("/// Use [`scalar_one_boilerplate`] to automatically implement this.")
                    const VEC$(n)_$(Axis(i).uppercase_str()): Vec$(n)<Self>;
                )
            )
        }

        $("/// A trait for scalar types that have a `-1` value.")
        pub trait ScalarNegOne: ScalarZero {
            $("/// `-1`")
            const NEG_ONE: Self;

            $(
                for n in Length::iter() join($['\n']) =>

                $("/// This is boilerplate that is required because of stable-Rust limitations.")
                $("/// Use [`scalar_neg_one_boilerplate`] to automatically implement this.")
                const VEC$(n)_NEG_ONE: Vec$(n)<Self>;

                $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    $("/// This is boilerplate that is required because of stable-Rust limitations.")
                    $("/// Use [`scalar_neg_one_boilerplate`] to automatically implement this.")
                    const VEC$(n)_NEG_$(Axis(i).uppercase_str()): Vec$(n)<Self>;
                )
            )
        }

        $("/// Automatically generates the boilerplate part of [`ScalarZero`] implementations.")
        $("/// This requires that `Vector<N, Self, S>` has a `const_from_array` function.")
        #[macro_export]
        macro_rules! scalar_zero_boilerplate {
            {} => {
                $(
                    for n in Length::iter() join($['\r']) =>
    
                    const VEC$(n)_ZERO: $$crate::Vec$(n)<Self> = $$crate::Vec$(n)::<Self>::const_from_array([Self::ZERO; $n]);
                )
            }
        }

        $("/// Automatically generates the boilerplate part of [`ScalarOne`] implementations.")
        $("/// This requires that `Vector<N, Self, S>` has a `const_from_array` function.")
        #[macro_export]
        macro_rules! scalar_one_boilerplate {
            {} => {
                $(
                    for n in Length::iter() join($['\n']) =>
    
                    const VEC$(n)_ONE: $$crate::Vec$(n)<Self> = $$crate::Vec$(n)::<Self>::const_from_array([Self::ONE; $n]);
    
                    $(
                        for i in 0..n.as_usize() join($['\r']) =>
    
                        const VEC$(n)_$(Axis(i).uppercase_str()): $$crate::Vec$(n)<Self> = $$crate::Vec$(n)::<Self>::const_from_array([$(
                            for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { Self::ONE } else { Self::ZERO })
                        )]);
                    )
                )
            }
        }

        $("/// Automatically generates the boilerplate part of [`ScalarNegOne`] implementations.")
        $("/// This requires that `Vector<N, Self, S>` has a `const_from_array` function.")
        #[macro_export]
        macro_rules! scalar_neg_one_boilerplate {
            {} => {
                $(
                    for n in Length::iter() join($['\n']) =>
    
                    const VEC$(n)_NEG_ONE: $$crate::Vec$(n)<Self> = $$crate::Vec$(n)::<Self>::const_from_array([Self::NEG_ONE; $n]);
    
                    $(
                        for i in 0..n.as_usize() join($['\r']) =>
    
                        const VEC$(n)_NEG_$(Axis(i).uppercase_str()): $$crate::Vec$(n)<Self> = $$crate::Vec$(n)::<Self>::const_from_array([$(
                            for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { Self::NEG_ONE } else { Self::ZERO })
                        )]);
                    )
                )
            }
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
