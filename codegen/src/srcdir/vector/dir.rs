use genco::{quote, tokens::quoted};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::util::TokensExt;

pub fn generate() {
    quote! {
        $(let common_lengths = [2, 3, 4])
        $(let axes_lowercase = ["x", "y", "z", "w"])
        $(let axes_uppercase = ["X", "Y", "Z", "W"])
        $(let axes_ordinals = ["1st", "2nd", "3rd", "4th"])

        use crate::{Construct, Vector, Simdness, ElementOfVector};

        $("/// Trait for element types that have a `ZERO` value.")
        pub trait ElementZero: Construct {
            $("/// `0` constant.")
            const ZERO: Self;
        }

        $("/// Trait for element types that have a `ONE` value.")
        pub trait ElementOne: Construct {
            $("/// `1` constant.")
            const ONE: Self;
        }

        $("/// Trait for element types that have a `NEG_ONE` value.")
        pub trait ElementNegOne: Construct {
            $("/// `-1` constant.")
            const NEG_ONE: Self;
        }

        macro_rules! impl_for_float {
            ($$T:ty) => {
                impl ElementZero for $$T {
                    const ZERO: Self = 0.0;
                }
        
                impl ElementOne for $$T {
                    const ONE: Self = 1.0;
                }
        
                impl ElementNegOne for $$T {
                    const NEG_ONE: Self = -1.0;
                }
            };
        }
        impl_for_float!(f32);
        impl_for_float!(f64);
        
        macro_rules! impl_for_sint {
            ($$T:ty) => {
                impl ElementZero for $$T {
                    const ZERO: Self = 0;
                }
        
                impl ElementOne for $$T {
                    const ONE: Self = 1;
                }
        
                impl ElementNegOne for $$T {
                    const NEG_ONE: Self = -1;
                }
            };
        }
        impl_for_sint!(i8);
        impl_for_sint!(i16);
        impl_for_sint!(i32);
        impl_for_sint!(i64);
        impl_for_sint!(i128);
        impl_for_sint!(isize);
        
        macro_rules! impl_for_uint {
            ($$T:ty) => {
                impl ElementZero for $$T {
                    const ZERO: Self = 0;
                }
        
                impl ElementOne for $$T {
                    const ONE: Self = 1;
                }
            };
        }
        impl_for_uint!(u8);
        impl_for_uint!(u16);
        impl_for_uint!(u32);
        impl_for_uint!(u64);
        impl_for_uint!(u128);
        impl_for_uint!(usize);

        impl<const N: usize, T: ElementZero + ElementOfVector<N, S>, S: Simdness> Vector<N, T, S> {
            $("/// `0` vector.")
            pub const ZERO: Self = Self::const_from_array([T::ZERO; N]);
        }

        impl<const N: usize, T: ElementOne + ElementOfVector<N, S>, S: Simdness> Vector<N, T, S> {
            $("/// Vector with all elements set to `1`.")
            pub const ONE: Self = Self::const_from_array([T::ONE; N]);
        }

        impl<const N: usize, T: ElementNegOne + ElementOfVector<N, S>, S: Simdness> Vector<N, T, S> {
            $("/// Vector with all elements set to `-1`.")
            pub const NEG_ONE: Self = Self::const_from_array([T::NEG_ONE; N]);
        }

        $(
            for n in common_lengths join($['\n']) =>

            impl<T: ElementZero + ElementOne + ElementOfVector<$n, S>, S: Simdness> Vector<$n, T, S> {
                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Vector pointing to the positive direction of the `{}` ({}) axis with magnitude `1`.", axes_lowercase[i], axes_ordinals[i]))
                    pub const $(axes_uppercase[i]): Self = Self::const_from_array([$(
                        for i2 in 0..n join(, ) =>

                        $(if i2 == i { T::ONE } else { T::ZERO })
                    )]);
                )
            }
        )

        $(
            for n in common_lengths join($['\n']) =>

            impl<T: ElementZero + ElementNegOne + ElementOfVector<$n, S>, S: Simdness> Vector<$n, T, S> {
                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Vector pointing to the negative direction of the `{}` ({}) axis with magnitude `1`.", axes_lowercase[i], axes_ordinals[i]))
                    pub const NEG_$(axes_uppercase[i]): Self = Self::const_from_array([$(
                        for i2 in 0..n join(, ) =>

                        $(if i2 == i { T::NEG_ONE } else { T::ZERO })
                    )]);
                )
            }
        )

        $(
            for dir in Direction::iter() join($['\n']) =>

            $(let neg_dir = dir.opposite())

            $(format!("/// Module with `{}` and `{}` constants where {} is positive.", dir.uppercase(), neg_dir.uppercase(), dir.snakecase()))
            #[cfg(feature = $(quoted(dir.snakecase())))]
            pub mod $(dir.snakecase()) {
                use crate::{Construct, Vector, Simdness, ElementOfVector, ElementNegOne, ElementOne, ElementZero};

                $(format!("/// Trait with a `{}` constant where {} is positive and {} is negative.", dir.uppercase(), dir.snakecase(), neg_dir.snakecase()))
                $("/// This trait is automatically implemented for vectors, and types that are [`ElementOne`].")
                pub trait Positive$(dir.camelcase()): Construct {
                    $(format!("/// Points {} with magnitude `1` ({} is positive and {} is negative).", dir.snakecase(), dir.snakecase(), neg_dir.snakecase()))
                    const $(dir.uppercase()): Self;
                }

                $(format!("/// Trait with a `{}` constant where {} is positive and {} is negative.", neg_dir.uppercase(), dir.snakecase(), neg_dir.snakecase()))
                $("/// This trait is automatically implemented for vectors, and types that are [`ElementNegOne`].")
                pub trait Negative$(neg_dir.camelcase()): Construct {
                    $(format!("/// Points {} with magnitude `1` ({} is positive and {} is negative).", neg_dir.snakecase(), dir.snakecase(), neg_dir.snakecase()))
                    const $(neg_dir.uppercase()): Self;
                }

                impl<T: ElementOne> Positive$(dir.camelcase()) for T {
                    const $(dir.uppercase()): Self = Self::ONE;
                }

                impl<T: ElementNegOne> Negative$(neg_dir.camelcase()) for T {
                    const $(neg_dir.uppercase()): Self = Self::NEG_ONE;
                }

                $(
                    for n in common_lengths.into_iter().filter(|&n| dir.axis() < n) join($['\n']) =>

                    impl<T: ElementZero + ElementOne + ElementOfVector<$n, S>, S: Simdness> Positive$(dir.camelcase()) for Vector<$n, T, S> {
                        const $(dir.uppercase()): Self = Self::$(axes_uppercase[dir.axis()]);
                    }
                )

                $(
                    for n in common_lengths.into_iter().filter(|&n| dir.axis() < n) join($['\n']) =>

                    impl<T: ElementZero + ElementNegOne + ElementOfVector<$n, S>, S: Simdness> Negative$(neg_dir.camelcase()) for Vector<$n, T, S> {
                        const $(neg_dir.uppercase()): Self = Self::NEG_$(axes_uppercase[dir.axis()]);
                    }
                )
            }
        )
    }
    .write_in_src("vector/dir.rs");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    Forwards,
    Backwards,
}

impl Direction {
    fn snakecase(&self) -> &str {
        match self {
            Direction::Right => "right",
            Direction::Left => "left",
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Forwards => "forwards",
            Direction::Backwards => "backwards",
        }
    }

    fn camelcase(&self) -> &str {
        match self {
            Direction::Right => "Right",
            Direction::Left => "Left",
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Forwards => "Forwards",
            Direction::Backwards => "Backwards",
        }
    }

    fn uppercase(&self) -> &str {
        match self {
            Direction::Right => "RIGHT",
            Direction::Left => "LEFT",
            Direction::Up => "UP",
            Direction::Down => "DOWN",
            Direction::Forwards => "FORWARDS",
            Direction::Backwards => "BACKWARDS",
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Forwards => Direction::Backwards,
            Direction::Backwards => Direction::Forwards,
        }
    }

    fn axis(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Left => 0,
            Direction::Up => 1,
            Direction::Down => 1,
            Direction::Forwards => 2,
            Direction::Backwards => 2,
        }
    }
}
