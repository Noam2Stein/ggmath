use bytemuck::{NoUninit, Pod, Zeroable};

use crate::{
    Affine, Alignment, Length, Mask, Matrix, Projective, Scalar, SupportedLength, Vector,
    length::TwoOrThree,
};

// SAFETY: Vectors are equivalent to structs where all fields are `Pod`. The
// `[T; N]` part is `Pod` because `T` is `Pod`, and the padding is guaranteed
// to have initialized bytes, and accepts any bit-pattern.
unsafe impl<const N: usize, T, A: Alignment> Pod for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Pod,
{
}

// SAFETY: Vectors are equivalent to structs where all fields are `Zeroable`.
// The `[T; N]` part is `Zeroable` because `T` is `Zeroable`, and the padding is
// guaranteed to accept any bit-pattern.
unsafe impl<const N: usize, T, A: Alignment> Zeroable for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zeroable,
{
}

// SAFETY: Matrices are equivalent to structs where all fields are `Pod`. The
// `[Vector<N, T, A>; N]` part is `Pod` because `Vector<N, T, A>` is `Pod`, and
// the padding is guaranteed to have initialized bytes, and accepts any
// bit-pattern.
unsafe impl<const N: usize, T, A: Alignment> Pod for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Pod,
{
}

// SAFETY: Matrices are equivalent to structs where all fields are `Zeroable`.
// The `[Vector<N, T, A>; N]` part is `Zeroable` because `Vector<N, T, A>` is
// `Zeroable`, and the padding is guaranteed to accept any bit-pattern.
unsafe impl<const N: usize, T, A: Alignment> Zeroable for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zeroable,
{
}

// SAFETY: Affines are equivalent to structs where all fields are `Pod`. The
// `Matrix<N, T, A>` part is `Pod`, the `Vector<N, T, A>` part is `Pod`, and
// padding bytes are initialized and accept all bit-patterns.
unsafe impl<const N: usize, T, A: Alignment> Pod for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Pod,
{
}

// SAFETY: Affines are equivalent to structs where all fields are `Zeroable`.
// The `Matrix<N, T, A>` part is `Zeroable`, the `Vector<N, T, A>` part is
// `Zeroable`, and padding bytes are initialized and accept all bit-patterns.
unsafe impl<const N: usize, T, A: Alignment> Zeroable for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zeroable,
{
}

// SAFETY: Projective is a simple wrapper over `Matrix`, which also implements
// this trait.
unsafe impl<const N: usize, T, A: Alignment> Pod for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Pod,
{
}

// SAFETY: Projective is a simple wrapper over `Matrix`, which also implements
// this trait.
unsafe impl<const N: usize, T, A: Alignment> Zeroable for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zeroable,
{
}

// SAFETY: Masks are guaranteed to have no uninitialized bytes, and accept the
// zero bit-pattern, meaning they are inhabited.
unsafe impl<const N: usize, T, A: Alignment> NoUninit for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + 'static,
{
}

// SAFETY: Masks are guaranteed to accept the zero bit-pattern.
unsafe impl<const N: usize, T, A: Alignment> Zeroable for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}
