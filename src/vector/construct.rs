use std::{
    mem::{MaybeUninit, offset_of},
    ptr::copy_nonoverlapping,
};

use super::*;

/// Constructs a new aligned vector from flexible arguments like shaders.
///
/// ```
/// use ggmath::*;
///
/// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
/// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
/// ```
#[macro_export]
macro_rules! vec2 {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<2, _, $crate::VecAligned>
            = $crate::construct_vector::<2, _, $crate::VecAligned, _>(($($item,)*));

        output
    }};
}

/// Constructs a new packed vector from flexible arguments like shaders.
///
/// ```
/// use ggmath::*;
///
/// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
/// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
/// ```
#[macro_export]
macro_rules! vec2p {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<2, _, $crate::VecPacked>
            = $crate::construct_vector::<2, _, $crate::VecPacked, _>(($($item,)*));

        output
    }};
}

/// Constructs a new vector from flexible arguments like shaders, generic over alignment.
///
/// Unlike `vec{n}!` and `vec{n}p!` macros,
/// this macro does not decide the alignment of the vector.
///
/// Instead, it is not specified and requires an inference hint.
#[macro_export]
macro_rules! vec2g {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<2, _, _>
            = $crate::construct_vector::<2, _, _, _>(($($item,)*));

        output
    }};
}

/// Constructs a new aligned vector from flexible arguments like shaders.
///
/// ```
/// use ggmath::*;
///
/// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
/// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
/// ```
#[macro_export]
macro_rules! vec3 {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<3, _, $crate::VecAligned>
            = $crate::construct_vector::<3, _, $crate::VecAligned, _>(($($item,)*));

        output
    }};
}

/// Constructs a new packed vector from flexible arguments like shaders.
///
/// ```
/// use ggmath::*;
///
/// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
/// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
/// ```
#[macro_export]
macro_rules! vec3p {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<3, _, $crate::VecPacked>
            = $crate::construct_vector::<3, _, $crate::VecPacked, _>(($($item,)*));

        output
    }};
}

/// Constructs a new vector from flexible arguments like shaders, generic over alignment.
///
/// Unlike `vec{n}!` and `vec{n}p!` macros,
/// this macro does not decide the alignment of the vector.
///
/// Instead, it is not specified and requires an inference hint.
#[macro_export]
macro_rules! vec3g {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<3, _, _>
            = $crate::construct_vector::<3, _, _, _>(($($item,)*));

        output
    }};
}

/// Constructs a new aligned vector from flexible arguments like shaders.
///
/// ```
/// use ggmath::*;
///
/// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
/// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
/// ```
#[macro_export]
macro_rules! vec4 {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<4, _, $crate::VecAligned>
            = $crate::construct_vector::<4, _, $crate::VecAligned, _>(($($item,)*));

        output
    }};
}

/// Constructs a new packed vector from flexible arguments like shaders.
///
/// ```
/// use ggmath::*;
///
/// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
/// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
/// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
/// ```
#[macro_export]
macro_rules! vec4p {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<4, _, $crate::VecPacked>
            = $crate::construct_vector::<4, _, $crate::VecPacked, _>(($($item,)*));

        output
    }};
}

/// Constructs a new vector from flexible arguments like shaders, generic over alignment.
///
/// Unlike `vec{n}!` and `vec{n}p!` macros,
/// this macro does not decide the alignment of the vector.
///
/// Instead, it is not specified and requires an inference hint.
#[macro_export]
macro_rules! vec4g {
    ($($item:expr), * $(,)?) => {{
        let output: $crate::Vector<4, _, _>
            = $crate::construct_vector::<4, _, _, _>(($($item,)*));

        output
    }};
}

#[doc(hidden)]
#[inline(always)]
pub const fn construct_vector<const N: usize, T: Scalar, A: VecAlignment, I: IntoVector<N, T>>(
    value: I,
) -> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    let mut output = unsafe { MaybeUninit::uninit().assume_init() };

    let value_ptr = (&value) as *const _ as *const T;
    let output_ptr = (&mut output) as *mut _ as *mut T;

    let mut i = 0;
    while i < N {
        let src_offset = I::SOURCES[i];
        let dst_offset = size_of::<T>() * i;

        let src_ptr = unsafe { value_ptr.byte_add(src_offset) };
        let dst_ptr = unsafe { output_ptr.byte_add(dst_offset) };

        unsafe {
            copy_nonoverlapping(src_ptr, dst_ptr, 1);
        }

        i += 1;
    }

    output
}

#[doc(hidden)]
pub unsafe trait IntoVector<const N: usize, T: Scalar>: Construct {
    const SOURCES: [usize; N];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<2, T> for (Vector<2, T, A0>,) {
    const SOURCES: [usize; 2] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar> IntoVector<2, T> for (T, T) {
    const SOURCES: [usize; 2] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<3, T> for (Vector<3, T, A0>,) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<3, T> for (Vector<2, T, A0>, T) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector<3, T> for (T, Vector<2, T, A1>) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar> IntoVector<3, T> for (T, T, T) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<4, T> for (Vector<4, T, A0>,) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
        offset_of!(Self, 0) + size_of::<T>() * 3,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<4, T> for (Vector<3, T, A0>, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment, A1: VecAlignment> IntoVector<4, T>
    for (Vector<2, T, A0>, Vector<2, T, A1>)
{
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<4, T> for (Vector<2, T, A0>, T, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector<4, T> for (T, Vector<3, T, A1>) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 2,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector<4, T> for (T, Vector<2, T, A1>, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A2: VecAlignment> IntoVector<4, T> for (T, T, Vector<2, T, A2>) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar> IntoVector<4, T> for (T, T, T, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
        offset_of!(Self, 3) + size_of::<T>() * 0,
    ];
}
