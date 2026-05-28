// Based on `https://github.com/Lokathor/wide/blob/main/tests/utils/random_iter.rs`.

use crate::{Affine, Alignment, Length, Mask, Matrix, Quaternion, Scalar, SupportedLength, Vector};

/// Returns an iterator over 100 random values of type `T`.
///
/// This is used for fuzz-testing.
#[expect(private_bounds)]
pub fn random_iter<T>() -> impl Iterator<Item = T>
where
    T: Random,
{
    const SEED: u64 = 0x123456789abcdef0;
    const ITERATIONS: usize = 100;

    let mut state = SEED;
    (0..ITERATIONS).map(move |_| T::random(&mut state))
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

trait Random {
    fn random(state: &mut u64) -> Self;
}

macro_rules! float_impl_random {
    ($T:ident, $UnsignedT:ident) => {
        impl Random for $T {
            fn random(state: &mut u64) -> Self {
                const MANTISSA_BITS: u32 = $T::MANTISSA_DIGITS - 1;
                const EXPONENT_BITS: u32 = $UnsignedT::BITS - MANTISSA_BITS - 1;

                let bits = random_bits(state);

                match bits % 59 {
                    0 => 0.0,
                    1 => -0.0,
                    2 => 1.0,
                    3 => -1.0,
                    4..=6 => $T::NAN,
                    7..=8 => $T::INFINITY,
                    9..=10 => $T::NEG_INFINITY,
                    _ => {
                        let mantissa_bits = (bits as $UnsignedT) % ((1 << $T::MANTISSA_DIGITS) - 1);

                        let exponent = match bits % 97 {
                            0 => bits as i32 % 103,
                            1 => -(bits as i32 % 103),
                            2..=20 => bits as i32 % 11,
                            _ => bits as i32 % 7,
                        };
                        let exponent_bias = (1i32 << (EXPONENT_BITS - 1)) - 1;
                        let exponent =
                            (exponent + exponent_bias).clamp(1, (1 << EXPONENT_BITS) - 2);
                        let exponent_bits = (exponent as $UnsignedT) << MANTISSA_BITS;

                        let sign_bit = (bits as $UnsignedT) & 1 << ($UnsignedT::BITS - 1);

                        $T::from_bits(mantissa_bits | exponent_bits | sign_bit)
                    }
                }
            }
        }

        impl<const N: usize, A: Alignment> Random for Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            fn random(state: &mut u64) -> Self {
                const MANTISSA_BITS: u32 = $T::MANTISSA_DIGITS - 1;
                const EXPONENT_BITS: u32 = $UnsignedT::BITS - MANTISSA_BITS - 1;

                let base = $T::random(state);

                Self::from_fn(|_| {
                    let bits = random_bits(state);

                    match bits % 157 {
                        0 => 0.0,
                        1 => -0.0,
                        2 => 1.0,
                        3 => -1.0,
                        4 => $T::NAN,
                        5 => $T::INFINITY,
                        6 => $T::NEG_INFINITY,
                        _ => {
                            let mantissa_bits =
                                (bits as $UnsignedT) % ((1 << $T::MANTISSA_DIGITS) - 1);

                            let exponent = bits as i32 % 7;
                            let exponent_bias = (1i32 << (EXPONENT_BITS - 1)) - 1;
                            let exponent =
                                (exponent + exponent_bias).clamp(1, (1 << EXPONENT_BITS) - 2);
                            let exponent_bits = (exponent as $UnsignedT) << MANTISSA_BITS;

                            let sign_bit = (bits as $UnsignedT) & 1 << ($UnsignedT::BITS - 1);

                            base * $T::from_bits(mantissa_bits | exponent_bits | sign_bit)
                        }
                    }
                })
            }
        }

        impl<const N: usize, A: Alignment> Random for Matrix<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            fn random(state: &mut u64) -> Self {
                const MANTISSA_BITS: u32 = $T::MANTISSA_DIGITS - 1;
                const EXPONENT_BITS: u32 = $UnsignedT::BITS - MANTISSA_BITS - 1;

                let base = Vector::<N, $T, A>::random(state);

                Self::from_row_fn(|_| {
                    let bits = random_bits(state);

                    let mantissa_bits = (bits as $UnsignedT) % ((1 << $T::MANTISSA_DIGITS) - 1);

                    let exponent = bits as i32 % 7;
                    let exponent_bias = (1i32 << (EXPONENT_BITS - 1)) - 1;
                    let exponent = (exponent + exponent_bias).clamp(1, (1 << EXPONENT_BITS) - 2);
                    let exponent_bits = (exponent as $UnsignedT) << MANTISSA_BITS;

                    let sign_bit = (bits as $UnsignedT) & 1 << ($UnsignedT::BITS - 1);

                    base * $T::from_bits(mantissa_bits | exponent_bits | sign_bit)
                })
            }
        }

        impl<A: Alignment> Random for Quaternion<$T, A> {
            fn random(state: &mut u64) -> Self {
                let result = Self::from_vector(Vector::random(state));
                result.normalize_or(result)
            }
        }

        impl<const N: usize, A: Alignment> Random for Affine<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            fn random(state: &mut u64) -> Self {
                const MANTISSA_BITS: u32 = $T::MANTISSA_DIGITS - 1;
                const EXPONENT_BITS: u32 = $UnsignedT::BITS - MANTISSA_BITS - 1;

                let base = Vector::<N, $T, A>::random(state);

                Self::from_row_fn(|_| {
                    let bits = random_bits(state);

                    let mantissa_bits = (bits as $UnsignedT) % ((1 << $T::MANTISSA_DIGITS) - 1);

                    let exponent = bits as i32 % 7;
                    let exponent_bias = (1i32 << (EXPONENT_BITS - 1)) - 1;
                    let exponent = (exponent + exponent_bias).clamp(1, (1 << EXPONENT_BITS) - 2);
                    let exponent_bits = (exponent as $UnsignedT) << MANTISSA_BITS;

                    let sign_bit = (bits as $UnsignedT) & 1 << ($UnsignedT::BITS - 1);

                    base * $T::from_bits(mantissa_bits | exponent_bits | sign_bit)
                })
            }
        }
    };
}
float_impl_random!(f32, u32);
float_impl_random!(f64, u64);

macro_rules! integer_impl_random {
    ($T:ident) => {
        impl Random for $T {
            fn random(state: &mut u64) -> Self {
                let bits = random_bits(state) as Self;
                let exponent = match bits % 11 {
                    0..=3 => bits.rem_euclid(Self::BITS as Self),
                    _ => bits.rem_euclid(5),
                };

                bits.unbounded_shr((Self::BITS as Self - exponent) as u32)
            }
        }
    };
}
integer_impl_random!(i8);
integer_impl_random!(i16);
integer_impl_random!(i32);
integer_impl_random!(i64);
integer_impl_random!(i128);
integer_impl_random!(isize);
integer_impl_random!(u8);
integer_impl_random!(u16);
integer_impl_random!(u32);
integer_impl_random!(u64);
integer_impl_random!(u128);
integer_impl_random!(usize);

impl Random for bool {
    fn random(state: &mut u64) -> Self {
        random_bits(state) > u64::MAX / 2
    }
}

impl<T, const N: usize> Random for [T; N]
where
    T: Random,
{
    fn random(state: &mut u64) -> Self {
        std::array::from_fn(|_| T::random(state))
    }
}

impl<const N: usize, T, A: Alignment> Random for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    fn random(state: &mut u64) -> Self {
        Self::from_array(Random::random(state))
    }
}

#[cfg(feature = "wide")]
mod wide {
    use wide::{
        f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16, i16x32, i32x4,
        i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8,
        u32x16, u64x2, u64x4, u64x8,
    };

    use crate::utils::random_iter::Random;

    macro_rules! wide_impl_random {
        ($T:ident, $Simd:ident) => {
            impl Random for $Simd {
                fn random(state: &mut u64) -> Self {
                    Self::new(Random::random(state))
                }
            }
        };
    }
    wide_impl_random!(f32, f32x4);
    wide_impl_random!(f32, f32x8);
    wide_impl_random!(f32, f32x16);
    wide_impl_random!(f64, f64x2);
    wide_impl_random!(f64, f64x4);
    wide_impl_random!(f64, f64x8);
    wide_impl_random!(i8, i8x16);
    wide_impl_random!(i8, i8x32);
    wide_impl_random!(i16, i16x8);
    wide_impl_random!(i16, i16x16);
    wide_impl_random!(i16, i16x32);
    wide_impl_random!(i32, i32x4);
    wide_impl_random!(i32, i32x8);
    wide_impl_random!(i32, i32x16);
    wide_impl_random!(i64, i64x2);
    wide_impl_random!(i64, i64x4);
    wide_impl_random!(i64, i64x8);
    wide_impl_random!(u8, u8x16);
    wide_impl_random!(u8, u8x32);
    wide_impl_random!(u16, u16x8);
    wide_impl_random!(u16, u16x16);
    wide_impl_random!(u16, u16x32);
    wide_impl_random!(u32, u32x4);
    wide_impl_random!(u32, u32x8);
    wide_impl_random!(u32, u32x16);
    wide_impl_random!(u64, u64x2);
    wide_impl_random!(u64, u64x4);
    wide_impl_random!(u64, u64x8);
}
