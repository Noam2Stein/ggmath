// Based on `https://github.com/Lokathor/wide/blob/main/tests/utils/random_iter.rs`.

extern crate std;

use std::iter::repeat_n;

use crate::{
    Affine, Alignment, FloatExt, Length, Mask, Matrix, Quaternion, Scalar, SupportedLength, Vector,
};

/// Returns an iterator over random values.
///
/// This is used for fuzz-testing.
pub fn random_iter<T>() -> impl Iterator<Item = T>
where
    T: Random,
{
    const SEED: u64 = 0x123456789abcdef0;
    const CATEGORIES: &[Category] = &[
        Category::Exponents(-1, 0),
        Category::Exponents(-1, 1),
        Category::Exponents(-2, 2),
        Category::Exponents(-3, 4),
        Category::Exponents(-5, 4),
        Category::Exponents(0, 8),
        Category::Exponents(-8, 0),
        Category::Exponents(-100, 100),
        Category::MostlyZeroOne,
        Category::MostlyNanInfinity,
    ];

    let mut state = SEED;
    T::categories(CATEGORIES.iter().copied())
        .flat_map(|category| repeat_n(category, 1000))
        .map(move |category| T::random(&mut state, category))
}

pub trait Random {
    type Category: Copy;

    fn random(state: &mut u64, category: Self::Category) -> Self;

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// An inclusive range of base-10 exponents.
    Exponents(i32, i32),
    MostlyZeroOne,
    MostlyNanInfinity,
}

/// Generates the next pseudo-random number.
///
/// Definitely non-cryptographic, just used to generate random test values.
fn update_state(state: &mut u64) {
    // Constants for the LCG
    const A: u64 = 6364136223846793005;
    const C: u64 = 1442695040888963407;

    // Update the state and calculate the next number (rotate to avoid lack of
    // randomness in low bits).
    *state = state.wrapping_mul(A).wrapping_add(C).rotate_left(31);
}

fn random_bits(state: &mut u64) -> u64 {
    update_state(state);
    *state
}

macro_rules! float_impl_random {
    ($T:ident, $UnsignedT:ident) => {
        impl Random for $T {
            type Category = Category;

            fn random(state: &mut u64, category: Self::Category) -> Self {
                let bits = random_bits(state);

                let (min_exponent, max_exponent) = match category {
                    Category::Exponents(min_exponent, max_exponent) => match bits % 51 {
                        0 => return 0.0,
                        1 => return -0.0,
                        2 => return 1.0,
                        3 => return -1.0,
                        _ => (min_exponent, max_exponent),
                    },
                    Category::MostlyZeroOne => match bits % 6 {
                        0 => return 0.0,
                        1 => return -0.0,
                        2 => return 1.0,
                        3 => return -1.0,
                        _ => (-1, 0),
                    },
                    Category::MostlyNanInfinity => match bits % 6 {
                        0 => return $T::NAN,
                        1 => return $T::INFINITY,
                        2 => return $T::NEG_INFINITY,
                        _ => (-2, 2),
                    },
                };
                let exponents_len = max_exponent - min_exponent + 1;

                let exponent = (bits % exponents_len as u64) as i32 + min_exponent;
                let min = (10.0 as $T).powi(exponent);
                let max = (10.0 as $T).powi(exponent + 1);

                const T_BITS: u32 = 53;
                let t = (bits >> (u64::BITS - T_BITS)) as $T / (1u64 << T_BITS) as $T;

                min.lerp(max, t)
            }

            fn categories(
                categories: impl Iterator<Item = Category>,
            ) -> impl Iterator<Item = Self::Category> {
                categories
            }
        }
    };
}
float_impl_random!(f32, u32);
float_impl_random!(f64, u64);

macro_rules! signed_impl_random {
    ($T:ident) => {
        impl Random for $T {
            type Category = Category;

            fn random(state: &mut u64, category: Self::Category) -> Self {
                let bits = random_bits(state);

                let (min_exponent, max_exponent) = match category {
                    Category::Exponents(min_exponent, max_exponent) => match bits % 51 {
                        0 => return 0,
                        1 => return 1,
                        2 => return -1,
                        _ => (min_exponent, max_exponent),
                    },
                    Category::MostlyZeroOne => match bits % 4 {
                        0 => return 0,
                        1 => return 1,
                        2 => return -1,
                        _ => (-1, 0),
                    },
                    Category::MostlyNanInfinity => (-100, 100),
                };

                let exponent =
                    (bits % (max_exponent - min_exponent + 1) as u64) as i32 + min_exponent;
                let exponent = (exponent.max(0) as u32) % (Self::BITS + 1);

                (bits as Self).unbounded_shr(Self::BITS - exponent)
            }

            fn categories(
                categories: impl Iterator<Item = Category>,
            ) -> impl Iterator<Item = Self::Category> {
                categories
            }
        }
    };
}
signed_impl_random!(i8);
signed_impl_random!(i16);
signed_impl_random!(i32);
signed_impl_random!(i64);
signed_impl_random!(i128);
signed_impl_random!(isize);

macro_rules! unsigned_impl_random {
    ($T:ident) => {
        impl Random for $T {
            type Category = Category;

            fn random(state: &mut u64, category: Self::Category) -> Self {
                let bits = random_bits(state);
                let (min_exponent, max_exponent) = match category {
                    Category::Exponents(min_exponent, max_exponent) => match bits % 47 {
                        0 => return 0,
                        1 => return 1,
                        _ => (min_exponent, max_exponent),
                    },
                    Category::MostlyZeroOne => match bits % 3 {
                        0 => return 0,
                        1 => return 1,
                        _ => (-1, 0),
                    },
                    Category::MostlyNanInfinity => (-100, 100),
                };

                let exponent =
                    (bits % (max_exponent - min_exponent + 1) as u64) as i32 + min_exponent;
                let exponent = (exponent.max(0) as u32) % (Self::BITS + 1);

                (bits as Self).unbounded_shr(Self::BITS - exponent)
            }

            fn categories(
                categories: impl Iterator<Item = Category>,
            ) -> impl Iterator<Item = Self::Category> {
                categories
            }
        }
    };
}
unsigned_impl_random!(u8);
unsigned_impl_random!(u16);
unsigned_impl_random!(u32);
unsigned_impl_random!(u64);
unsigned_impl_random!(u128);
unsigned_impl_random!(usize);

impl Random for bool {
    type Category = Category;

    fn random(state: &mut u64, _category: Self::Category) -> Self {
        random_bits(state) > u64::MAX / 2
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        categories
    }
}

impl<T, const N: usize> Random for [T; N]
where
    T: Random,
{
    type Category = T::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        std::array::from_fn(|_| T::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T::categories(categories)
    }
}

impl<T0, T1> Random for (T0, T1)
where
    T0: Random,
    T1: Random<Category = T0::Category>,
{
    type Category = T0::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        (T0::random(state, category), T1::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T0::categories(categories)
    }
}

impl<T0, T1, T2> Random for (T0, T1, T2)
where
    T0: Random,
    T1: Random<Category = T0::Category>,
    T2: Random<Category = T0::Category>,
{
    type Category = T0::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        (
            T0::random(state, category),
            T1::random(state, category),
            T2::random(state, category),
        )
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T0::categories(categories)
    }
}

impl<const N: usize, T, A: Alignment> Random for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Random,
{
    type Category = T::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        Self::from_array(Random::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T::categories(categories)
    }
}

impl<const N: usize, T, A: Alignment> Random for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Random,
{
    type Category = T::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        Self::from_rows(&Random::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T::categories(categories)
    }
}

impl<T, A: Alignment> Random for Quaternion<T, A>
where
    T: Scalar + Random,
{
    type Category = T::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        Self::from_array(Random::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T::categories(categories)
    }
}

impl<const N: usize, T, A: Alignment> Random for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Random,
{
    type Category = T::Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        Self::from_row_fn(|_| Random::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        T::categories(categories)
    }
}

impl<const N: usize, T, A: Alignment> Random for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Category = Category;

    fn random(state: &mut u64, category: Self::Category) -> Self {
        Self::from_array(Random::random(state, category))
    }

    fn categories(
        categories: impl Iterator<Item = Category>,
    ) -> impl Iterator<Item = Self::Category> {
        categories
    }
}

#[cfg(feature = "wide")]
mod wide {
    extern crate std;

    use wide::{
        f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16, i16x32, i32x4,
        i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8,
        u32x16, u64x2, u64x4, u64x8,
    };

    use crate::utils::random_iter::{Category, Random};

    macro_rules! wide_impl_random {
        ($T:ident, $LANES:literal, $Wide:ident) => {
            impl Random for $Wide {
                type Category = [Category; $LANES];

                fn random(state: &mut u64, category: Self::Category) -> Self {
                    Self::new(std::array::from_fn(|lane| {
                        $T::random(state, category[lane])
                    }))
                }

                fn categories(
                    categories: impl Iterator<Item = Category>,
                ) -> impl Iterator<Item = Self::Category> {
                    let mut categories = categories.peekable();

                    std::iter::from_fn(move || {
                        if categories.peek().is_some() {
                            Some(std::array::from_fn(|_| {
                                categories.next().unwrap_or(Category::MostlyZeroOne)
                            }))
                        } else {
                            None
                        }
                    })
                }
            }
        };
    }
    wide_impl_random!(f32, 4, f32x4);
    wide_impl_random!(f32, 8, f32x8);
    wide_impl_random!(f32, 16, f32x16);
    wide_impl_random!(f64, 2, f64x2);
    wide_impl_random!(f64, 4, f64x4);
    wide_impl_random!(f64, 8, f64x8);
    wide_impl_random!(i8, 16, i8x16);
    wide_impl_random!(i8, 32, i8x32);
    wide_impl_random!(i16, 8, i16x8);
    wide_impl_random!(i16, 16, i16x16);
    wide_impl_random!(i16, 32, i16x32);
    wide_impl_random!(i32, 4, i32x4);
    wide_impl_random!(i32, 8, i32x8);
    wide_impl_random!(i32, 16, i32x16);
    wide_impl_random!(i64, 2, i64x2);
    wide_impl_random!(i64, 4, i64x4);
    wide_impl_random!(i64, 8, i64x8);
    wide_impl_random!(u8, 16, u8x16);
    wide_impl_random!(u8, 32, u8x32);
    wide_impl_random!(u16, 8, u16x8);
    wide_impl_random!(u16, 16, u16x16);
    wide_impl_random!(u16, 32, u16x32);
    wide_impl_random!(u32, 4, u32x4);
    wide_impl_random!(u32, 8, u32x8);
    wide_impl_random!(u32, 16, u32x16);
    wide_impl_random!(u64, 2, u64x2);
    wide_impl_random!(u64, 4, u64x4);
    wide_impl_random!(u64, 8, u64x8);
}
