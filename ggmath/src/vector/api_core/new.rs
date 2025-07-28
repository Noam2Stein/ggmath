use std::{mem::offset_of, ptr::copy_nonoverlapping};

use super::*;

macro_loop! {
    @for N in 2..=4 {
        /// Constructs a new aligned vector from flexible arguments like shaders.
        ///
        /// ```
        /// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
        /// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
        /// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
        /// ```
        #[macro_export]
        macro_rules! @[vec @N] {
            ($($item:expr), * $(,)?) => {{
                let output: $crate::@[Vec @N]<_> = $crate::new_vector(($($item,)*));
                output
            }};
        }

        /// Constructs a new packed vector from flexible arguments like shaders.
        ///
        /// ```
        /// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
        /// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
        /// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
        /// ```
        #[macro_export]
        macro_rules! @[vec @N p] {
            ($($item:expr), * $(,)?) => {{
                let output: $crate::@[Vec @N P]<_> = $crate::new_vector(($($item,)*));
                output
            }};
        }

        /// Constructs a new vector from flexible arguments like shaders, generic over alignment.
        /// This means that without an inference hint, there will be a compile error.
        ///
        /// ```
        /// const EXAMPLE_2: Vec2<f32> = vec2g!(1.0, 2.0);
        /// const EXAMPLE_3: Vec3P<f32> = vec3g!(1.0, 2.0, 3.0);
        /// const EXAMPLE_4: Vec4P<f32> = vec4g!(1.0, EXAMPLE_2, 4.0);
        /// ```
        #[macro_export]
        macro_rules! @[vec @N g] {
            ($($item:expr), * $(,)?) => {{
                let output: $crate::@[Vector]<@N, _, _> = $crate::new_vector(($($item,)*));
                output
            }};
        }
    }
}

#[doc(hidden)]
#[inline(always)]
pub const fn new_vector<const N: usize, T: Scalar, A: VecAlignment, I: IntoVector<N, T>>(
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

// Into Vec2

unsafe impl<T: Scalar> IntoVector<2, T> for (T, T) {
    const SOURCES: [usize; 2] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<2, T> for (Vector<2, T, A0>,) {
    const SOURCES: [usize; 2] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
    ];
}

// Into Vec3

unsafe impl<T: Scalar> IntoVector<3, T> for (T, T, T) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector<3, T> for (T, Vector<2, T, A1>) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<3, T> for (Vector<2, T, A0>, T) {
    const SOURCES: [usize; 3] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
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

// Into Vec4

unsafe impl<T: Scalar> IntoVector<4, T> for (T, T, T, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
        offset_of!(Self, 3) + size_of::<T>() * 0,
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

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector<4, T> for (T, Vector<2, T, A1>, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
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

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<4, T> for (Vector<2, T, A0>, T, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
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

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector<4, T> for (Vector<3, T, A0>, T) {
    const SOURCES: [usize; 4] = [
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
        offset_of!(Self, 1) + size_of::<T>() * 0,
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
