//! A module containing the [`bench!`] helper macro.

use ggmath::{Affine, Alignment, Length, Mask, Matrix, Rotor, Scalar, SupportedLength, Vector};
use rand::{RngExt, SeedableRng, rngs::StdRng};
use wide::{f32x4, f32x8, f64x4};

use std::hint::black_box;

use divan::Bencher;

/// Benchmarks multiple related functions by reading and writing into an array.
///
/// ```ignore
/// bench!(
///     {function-name},
///     {array-length},
///     ({variant-name}, {callable-function}),
///     ({variant-name}, {callable-function}),
///     ...
/// );
/// ```
///
/// `{function-name}` is the name of the function being compared.
///
/// `{array-length}` controls the number of input and output values used to call
/// the functions. The inputs are randomly generated based on function
/// signatures.
///
/// `{callable-function}` can be either a function path or a closure.
///
/// `{variant-name}` follows the convention:
///
/// ```ignore
/// unaligned
/// unaligned_{alternative-crate-name}
/// aligned
/// aligned_{alternative-crate-name}
/// x{soa-lane-count}_unaligned
/// x{soa-lane-count}_unaligned_{alternative-crate-name}
/// ```
#[macro_export(local_inner_macros)]
macro_rules! bench {
    ($fn_name:ident, $array_len:expr, $(($type_name:ident, $f:expr)),* $(,)?) => {
        mod $fn_name {
            use super::*;

            $(
                #[divan::bench(
                    sample_count = $crate::SAMPLE_COUNT,
                    sample_size = $crate::SAMPLE_SIZE,
                )]
                fn $type_name(bencher: divan::Bencher) {
                    $crate::bench::bench_helper(bencher, $f, $array_len);
                }
            )*
        }
    };
}

#[doc(hidden)]
#[expect(private_bounds)]
pub fn bench_helper<F, I, O>(bencher: Bencher, f: F, array_len: usize)
where
    F: VariadicFn<I, O>,
    I: BenchIo,
    O: BenchIo,
{
    const {
        assert!(I::LANES == O::LANES);
    }

    let inputs = Box::from_iter(random_iter::<I>().take(array_len / I::LANES));
    let mut outputs = Box::from_iter(random_iter::<O>().take(array_len / I::LANES));

    for _ in 0..array_len / I::LANES {
        let inputs = black_box(&inputs);
        let outputs = black_box(&mut outputs);

        for (input, output) in inputs.iter().copied().zip(outputs.iter_mut()) {
            *output = f.invoke(input);
        }

        black_box(inputs);
        black_box(outputs);
    }

    bencher.bench_local(|| {
        let inputs = black_box(&inputs);
        let outputs = black_box(&mut outputs);

        for (input, output) in inputs.iter().copied().zip(outputs.iter_mut()) {
            *output = f.invoke(input);
        }

        black_box(inputs);
        black_box(outputs);
    });

    black_box(inputs);
    black_box(outputs);
}

trait VariadicFn<I, O>: Copy {
    fn invoke(self, input: I) -> O;
}

impl<F, I0, O> VariadicFn<(I0,), O> for F
where
    F: Copy + Fn(I0) -> O,
{
    #[inline(always)]
    fn invoke(self, input: (I0,)) -> O {
        self(input.0)
    }
}

impl<F, I0, I1, O> VariadicFn<(I0, I1), O> for F
where
    F: Copy + Fn(I0, I1) -> O,
{
    #[inline(always)]
    fn invoke(self, input: (I0, I1)) -> O {
        self(input.0, input.1)
    }
}

impl<F, I0, I1, I2, O> VariadicFn<(I0, I1, I2), O> for F
where
    F: Copy + Fn(I0, I1, I2) -> O,
{
    #[inline(always)]
    fn invoke(self, input: (I0, I1, I2)) -> O {
        self(input.0, input.1, input.2)
    }
}

impl<F, I0, I1, I2, I3, O> VariadicFn<(I0, I1, I2, I3), O> for F
where
    F: Copy + Fn(I0, I1, I2, I3) -> O,
{
    #[inline(always)]
    fn invoke(self, input: (I0, I1, I2, I3)) -> O {
        self(input.0, input.1, input.2, input.3)
    }
}

fn random_iter<T>() -> impl Iterator<Item = T>
where
    T: BenchIo,
{
    let mut rng = StdRng::from_seed([
        45, 12, 90, 120, 102, 15, 162, 12, 36, 78, 250, 125, 195, 104, 26, 75, 28, 68, 11, 16, 104,
        45, 12, 90, 120, 102, 15, 162, 12, 89, 192, 53,
    ]);

    std::iter::from_fn(move || Some(T::random(&mut rng)))
}

trait BenchIo: Copy {
    const LANES: usize;

    fn random(rng: &mut StdRng) -> Self;
}

macro_rules! primitive_impl {
    ($T:ident) => {
        impl BenchIo for $T {
            const LANES: usize = 1;

            fn random(rng: &mut StdRng) -> Self {
                rng.random()
            }
        }
    };
}
primitive_impl!(f32);
primitive_impl!(f64);
primitive_impl!(bool);

impl<T, const N: usize> BenchIo for [T; N]
where
    T: BenchIo,
{
    const LANES: usize = T::LANES;

    fn random(rng: &mut StdRng) -> Self {
        std::array::from_fn(|_| T::random(rng))
    }
}

impl<T0> BenchIo for (T0,)
where
    T0: BenchIo,
{
    const LANES: usize = T0::LANES;

    fn random(rng: &mut StdRng) -> Self {
        (T0::random(rng),)
    }
}

impl<T0, T1> BenchIo for (T0, T1)
where
    T0: BenchIo,
    T1: BenchIo,
{
    const LANES: usize = T0::LANES;

    fn random(rng: &mut StdRng) -> Self {
        const {
            assert!(T0::LANES == T1::LANES);
        }

        (T0::random(rng), T1::random(rng))
    }
}

impl<T0, T1, T2> BenchIo for (T0, T1, T2)
where
    T0: BenchIo,
    T1: BenchIo,
    T2: BenchIo,
{
    const LANES: usize = T0::LANES;

    fn random(rng: &mut StdRng) -> Self {
        const {
            assert!(T0::LANES == T1::LANES);
            assert!(T1::LANES == T2::LANES);
        }

        (T0::random(rng), T1::random(rng), T2::random(rng))
    }
}

impl<T0, T1, T2, T3> BenchIo for (T0, T1, T2, T3)
where
    T0: BenchIo,
    T1: BenchIo,
    T2: BenchIo,
    T3: BenchIo,
{
    const LANES: usize = T0::LANES;

    fn random(rng: &mut StdRng) -> Self {
        const {
            assert!(T0::LANES == T1::LANES);
            assert!(T1::LANES == T2::LANES);
            assert!(T2::LANES == T3::LANES);
        }

        (
            T0::random(rng),
            T1::random(rng),
            T2::random(rng),
            T3::random(rng),
        )
    }
}

impl<const N: usize, T, A: Alignment> BenchIo for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + BenchIo,
{
    const LANES: usize = T::LANES;

    fn random(rng: &mut StdRng) -> Self {
        Self::from_fn(|_| T::random(rng))
    }
}

impl<const N: usize, T, A: Alignment> BenchIo for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + BenchIo,
{
    const LANES: usize = T::LANES;

    fn random(rng: &mut StdRng) -> Self {
        Self::from_row_fn(|_| Vector::random(rng))
    }
}

impl<T, A: Alignment> BenchIo for Rotor<2, T, A>
where
    T: Scalar + BenchIo,
{
    const LANES: usize = T::LANES;

    fn random(rng: &mut StdRng) -> Self {
        Self::from_raw_vector(BenchIo::random(rng))
    }
}

impl<T, A: Alignment> BenchIo for Rotor<3, T, A>
where
    T: Scalar + BenchIo,
{
    const LANES: usize = T::LANES;

    fn random(rng: &mut StdRng) -> Self {
        Self::from_raw_vector(BenchIo::random(rng))
    }
}

impl<const N: usize, T, A: Alignment> BenchIo for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    const LANES: usize = 1;

    fn random(rng: &mut StdRng) -> Self {
        Self::from_fn(|_| bool::random(rng))
    }
}

impl<const N: usize, T, A: Alignment> BenchIo for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + BenchIo,
{
    const LANES: usize = T::LANES;

    fn random(rng: &mut StdRng) -> Self {
        Self::from_row_fn(|_| Vector::random(rng))
    }
}

macro_rules! glam_vector_impl {
    ($T:ty) => {
        impl BenchIo for $T {
            const LANES: usize = 1;

            fn random(rng: &mut StdRng) -> Self {
                Self::from_array(rng.random())
            }
        }
    };
}
glam_vector_impl!(glam::Vec2);
glam_vector_impl!(glam::Vec3);
glam_vector_impl!(glam::Vec3A);
glam_vector_impl!(glam::Vec4);
glam_vector_impl!(glam::BVec2);
glam_vector_impl!(glam::BVec3);
glam_vector_impl!(glam::BVec3A);
glam_vector_impl!(glam::BVec4);
glam_vector_impl!(glam::BVec4A);
glam_vector_impl!(glam::Quat);

macro_rules! glam_matrix_impl {
    ($T:ty) => {
        impl BenchIo for $T {
            const LANES: usize = 1;

            fn random(rng: &mut StdRng) -> Self {
                Self::from_cols_array(&rng.random())
            }
        }
    };
}
glam_matrix_impl!(glam::Mat2);
glam_matrix_impl!(glam::Mat3);
glam_matrix_impl!(glam::Mat3A);
glam_matrix_impl!(glam::Mat4);
glam_matrix_impl!(glam::Affine2);
glam_matrix_impl!(glam::Affine3);
glam_matrix_impl!(glam::Affine3A);

macro_rules! wide_impl {
    ($LANES:literal, $Wide:ident) => {
        impl BenchIo for $Wide {
            const LANES: usize = $LANES;

            fn random(rng: &mut StdRng) -> Self {
                Self::new(rng.random())
            }
        }
    };
}
wide_impl!(4, f32x4);
wide_impl!(4, f32x8);
wide_impl!(4, f64x4);
