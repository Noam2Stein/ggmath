// Based on `https://github.com/Lokathor/wide/blob/main/tests/utils/random_iter.rs`.

extern crate std;

use std::iter::repeat_n;

use crate::{
    Affine, Alignment, FloatExt, Length, Mask, Matrix, Projective, Quaternion, Rotor, Scalar,
    SupportedLength, Vector, length::TwoOrThree, utils::specialize_23,
};

/// Returns an iterator over random values.
///
/// This is used for fuzz-testing.
pub fn random_iter<T>() -> impl Iterator<Item = T>
where
    T: Random,
{
    const SEED: u64 = 0x123456789abcdef0;

    let mut state = SEED;
    T::Input::values().map(move |category| T::random(&mut state, category))
}

pub trait Random {
    type Input: Input;

    fn random(state: &mut u64, input: Self::Input) -> Self;
}

pub trait Input: Copy {
    fn values() -> impl Iterator<Item = Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// An inclusive range of base-10 exponents.
    Exponents(i32, i32),
    MostlyZeroOne,
    MostlyNanInfinity,
}

impl Input for Category {
    fn values() -> impl Iterator<Item = Self> {
        [
            (Category::Exponents(-1, 0), 1000),
            (Category::Exponents(-1, 1), 1000),
            (Category::Exponents(-2, 2), 1000),
            (Category::Exponents(-3, 4), 1000),
            (Category::Exponents(-5, 4), 500),
            (Category::Exponents(0, 8), 100),
            (Category::Exponents(-8, 0), 100),
            (Category::Exponents(-100, 100), 100),
            (Category::MostlyZeroOne, 200),
            (Category::MostlyNanInfinity, 100),
        ]
        .into_iter()
        .flat_map(|(element, count)| repeat_n(element, count))
    }
}

fn random_bits(state: &mut u64) -> u64 {
    fn update_state(state: &mut u64) {
        // Constants for the LCG
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;

        // Update the state and calculate the next number (rotate to avoid lack of
        // randomness in low bits).
        *state = state.wrapping_mul(A).wrapping_add(C).rotate_left(31);
    }

    update_state(state);
    *state
}

macro_rules! float_impl_random {
    ($T:ident, $UnsignedT:ident) => {
        impl Random for $T {
            type Input = Category;

            fn random(state: &mut u64, input: Self::Input) -> Self {
                let bits = random_bits(state);

                let (min_exponent, max_exponent) = match input {
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
        }
    };
}
float_impl_random!(f32, u32);
float_impl_random!(f64, u64);

macro_rules! signed_impl_random {
    ($T:ident) => {
        impl Random for $T {
            type Input = Category;

            fn random(state: &mut u64, input: Self::Input) -> Self {
                let bits = random_bits(state);

                let (min_exponent, max_exponent) = match input {
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
            type Input = Category;

            fn random(state: &mut u64, input: Self::Input) -> Self {
                let bits = random_bits(state);
                let (min_exponent, max_exponent) = match input {
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
    type Input = Category;

    fn random(state: &mut u64, _input: Self::Input) -> Self {
        random_bits(state) > u64::MAX / 2
    }
}

impl<T, const N: usize> Random for [T; N]
where
    T: Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        std::array::from_fn(|_| T::random(state, input))
    }
}

impl<T0, T1> Random for (T0, T1)
where
    T0: Random,
    T1: Random<Input = T0::Input>,
{
    type Input = T0::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        (T0::random(state, input), T1::random(state, input))
    }
}

impl<T0, T1, T2> Random for (T0, T1, T2)
where
    T0: Random,
    T1: Random<Input = T0::Input>,
    T2: Random<Input = T0::Input>,
{
    type Input = T0::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        (
            T0::random(state, input),
            T1::random(state, input),
            T2::random(state, input),
        )
    }
}

impl<const N: usize, T, A: Alignment> Random for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        Self::from_array(Random::random(state, input))
    }
}

impl<const N: usize, T, A: Alignment> Random for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        Self::from_rows(&Random::random(state, input))
    }
}

impl<const N: usize, T, A: Alignment> Random for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        Self::from_row_fn(|_| Random::random(state, input))
    }
}

impl<const N: usize, T, A: Alignment> Random for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        specialize_23!(Projective::<N, T, A>::random_backend((state,), (input,)))
    }
}

macro_rules! projective_backend {
    ($N:literal) => {
        impl<T, A: Alignment> Projective<$N, T, A>
        where
            T: Scalar + Random,
        {
            fn random_backend((state,): (&mut u64,), (input,): (T::Input,)) -> Self {
                Self::from_row_fn(|_| Random::random(state, input))
            }
        }
    };
}
projective_backend!(2);
projective_backend!(3);

impl<const N: usize, T, A: Alignment> Random for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        specialize_23!(Rotor::<N, T, A>::random_backend((state,), (input,)))
    }
}

macro_rules! rotor_backend {
    ($N:literal) => {
        impl<T, A: Alignment> Rotor<$N, T, A>
        where
            T: Scalar + Random,
        {
            fn random_backend((state,): (&mut u64,), (input,): (T::Input,)) -> Self {
                Self(Random::random(state, input))
            }
        }
    };
}
rotor_backend!(2);
rotor_backend!(3);

impl<T, A: Alignment> Random for Quaternion<T, A>
where
    T: Scalar + Random,
{
    type Input = T::Input;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        Self::from_array(Random::random(state, input))
    }
}

impl<const N: usize, T, A: Alignment> Random for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Input = Category;

    fn random(state: &mut u64, input: Self::Input) -> Self {
        Self::from_array(Random::random(state, input))
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

    use crate::test_utils::{Category, Input, Random};

    macro_rules! wide_impl_random {
        ($T:ident, $LANES:literal, $Wide:ident) => {
            impl Random for $Wide {
                type Input = [Category; $LANES];

                fn random(state: &mut u64, input: Self::Input) -> Self {
                    Self::new(std::array::from_fn(|lane| $T::random(state, input[lane])))
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

    impl<const N: usize> Input for [Category; N] {
        fn values() -> impl Iterator<Item = Self> {
            let mut categories = Category::values().peekable();

            std::iter::from_fn(move || {
                categories.peek().is_some().then(|| {
                    std::array::from_fn(|_| categories.next().unwrap_or(Category::MostlyZeroOne))
                })
            })
        }
    }
}
