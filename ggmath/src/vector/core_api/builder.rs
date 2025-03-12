use std::{
    mem::{MaybeUninit, offset_of},
    ops::Range,
    ptr::copy_nonoverlapping,
};

use super::*;

pub use ggmath_proc_macros::{vec2, vec2p, vec3, vec3p, vec4, vec4p};

pub const fn build_vector<const N: usize, B: VectorBuilder<N>, A: VecAlignment>(
    value: B,
) -> Vector<N, B::T, A>
where
    ScalarCount<N>: VecLen,
{
    let mut output = unsafe { MaybeUninit::<Vector<N, B::T, A>>::uninit().assume_init() };

    let value_ptr = (&value) as *const _ as *const u8;

    let mut copy_index = 0;
    while copy_index < B::COPY.len() {
        let (dst_offset, src_offset) = &B::COPY[copy_index];

        let src = unsafe { value_ptr.add(*src_offset) as *const B::T };
        let dst = unsafe { output.as_mut_ptr().add(dst_offset.start) };
        let count = dst_offset.end - dst_offset.start;

        unsafe {
            copy_nonoverlapping(src, dst, count);
        }

        copy_index += 1;
    }

    output
}

/// Implemented by tuples that can be turned into a vector through the ```vecn!()``` macros.
///
/// For example ```(T, T)``` implements ```VectorBuilder<2>``` so you can perform ```vec2!(x, y)```,
/// and ```(Vector<2, T, impl VecAlignment>, T, T)``` implements ```VectorBuilder<4>``` so you can perform ```vec4!(xy, z, w)```.
pub unsafe trait VectorBuilder<const N: usize>: Copy
where
    ScalarCount<N>: VecLen,
{
    type T: Scalar;

    const COPY: &[(Range<usize>, usize)];
}

// N

unsafe impl<const N: usize, T: Scalar, AInput: VecAlignment> VectorBuilder<N>
    for Vector<N, T, AInput>
where
    ScalarCount<N>: VecLen,
{
    type T = T;

    const COPY: &[(Range<usize>, usize)] = &[(0..N, 0)];
}

// N = 2

unsafe impl<T: Scalar> VectorBuilder<2> for (T, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] =
        &[(0..1, offset_of!(Self, 0)), (1..2, offset_of!(Self, 1))];
}

// N = 3

unsafe impl<T: Scalar> VectorBuilder<3> for (T, T, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] = &[
        (0..1, offset_of!(Self, 0)),
        (1..2, offset_of!(Self, 1)),
        (2..3, offset_of!(Self, 2)),
    ];
}

unsafe impl<T: Scalar, AInput: VecAlignment> VectorBuilder<3> for (Vector<2, T, AInput>, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] =
        &[(0..2, offset_of!(Self, 0)), (2..3, offset_of!(Self, 1))];
}

unsafe impl<T: Scalar, AInput: VecAlignment> VectorBuilder<3> for (T, Vector<2, T, AInput>) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] =
        &[(0..1, offset_of!(Self, 0)), (1..3, offset_of!(Self, 1))];
}

// N = 4

unsafe impl<T: Scalar> VectorBuilder<4> for (T, T, T, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] = &[
        (0..1, offset_of!(Self, 0)),
        (1..2, offset_of!(Self, 1)),
        (2..3, offset_of!(Self, 2)),
        (3..4, offset_of!(Self, 3)),
    ];
}

unsafe impl<T: Scalar, AInput: VecAlignment> VectorBuilder<4> for (Vector<2, T, AInput>, T, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] = &[
        (0..2, offset_of!(Self, 0)),
        (2..3, offset_of!(Self, 1)),
        (3..4, offset_of!(Self, 2)),
    ];
}

unsafe impl<T: Scalar, AAInput: VecAlignment> VectorBuilder<4> for (T, Vector<2, T, AAInput>, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] = &[
        (0..1, offset_of!(Self, 0)),
        (1..3, offset_of!(Self, 1)),
        (3..4, offset_of!(Self, 2)),
    ];
}

unsafe impl<T: Scalar, AAInput: VecAlignment> VectorBuilder<4> for (T, T, Vector<2, T, AAInput>) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] = &[
        (0..1, offset_of!(Self, 0)),
        (1..2, offset_of!(Self, 1)),
        (2..4, offset_of!(Self, 2)),
    ];
}

unsafe impl<T: Scalar, AAInput0: VecAlignment, AAInput1: VecAlignment> VectorBuilder<4>
    for (Vector<2, T, AAInput0>, Vector<2, T, AAInput1>)
{
    type T = T;

    const COPY: &[(Range<usize>, usize)] =
        &[(0..2, offset_of!(Self, 0)), (2..4, offset_of!(Self, 1))];
}

unsafe impl<T: Scalar, AAInput: VecAlignment> VectorBuilder<4> for (Vector<3, T, AAInput>, T) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] =
        &[(0..3, offset_of!(Self, 0)), (3..4, offset_of!(Self, 1))];
}

unsafe impl<T: Scalar, AInput: VecAlignment> VectorBuilder<4> for (T, Vector<3, T, AInput>) {
    type T = T;

    const COPY: &[(Range<usize>, usize)] =
        &[(0..1, offset_of!(Self, 0)), (1..4, offset_of!(Self, 1))];
}
