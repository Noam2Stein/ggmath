use std::{mem::offset_of, ptr::copy_nonoverlapping};

use super::*;

#[macro_export]
macro_rules! vector {
    ($($item:expr), * $(,)?) => {
        $crate::new_vector(($($item,)*))
    };
}

#[macro_export]
macro_rules! vec2 {
    ($($item:expr), * $(,)?) => {
        {
            let output: $crate::Vec2<_> = $crate::new_vector::<$crate::VecAligned>(($($item,)*));
            output
        }
    };
}
#[macro_export]
macro_rules! vec3 {
    ($($item:expr), * $(,)?) => {
        {
            let output: $crate::Vec3<_> = $crate::new_vector::<$crate::VecAligned>(($($item,)*));
            output
        }
    };
}
#[macro_export]
macro_rules! vec4 {
    ($($item:expr), * $(,)?) => {
        {
            let output: $crate::Vec4<_> = $crate::new_vector::<$crate::VecAligned>(($($item,)*));
            output
        }
    };
}

#[macro_export]
macro_rules! vec2p {
    ($($item:expr), * $(,)?) => {
        {
            let output: $crate::Vec2P<_> = $crate::new_vector::<$crate::VecPacked>(($($item,)*));
            output
        }
    };
}
#[macro_export]
macro_rules! vec3p {
    ($($item:expr), * $(,)?) => {
        {
            let output: $crate::Vec3P<_> = $crate::new_vector::<$crate::VecPacked>(($($item,)*));
            output
        }
    };
}
#[macro_export]
macro_rules! vec4p {
    ($($item:expr), * $(,)?) => {
        {
            let output: $crate::Vec4P<_> = $crate::new_vector::<$crate::VecPacked>(($($item,)*));
            output
        }
    };
}

#[doc(hidden)]
#[inline(always)]
pub const fn new_vector<B: IntoVector, A: VecAlignment>(value: B) -> B::Output<A> {
    let mut output = unsafe { MaybeUninit::<B::Output<A>>::uninit().assume_init() };

    let value_ptr = (&value) as *const _ as *const B::T;
    let output_ptr = (&mut output) as *mut _ as *mut B::T;

    let mut src_index = 0;
    while src_index < B::SOURCES.len() {
        let src_offset = B::SOURCES[src_index];
        let dst_offset = src_index * size_of::<B::T>();

        let src_ptr = unsafe { value_ptr.byte_add(src_offset) };
        let dst_ptr = unsafe { output_ptr.byte_add(dst_offset) };

        unsafe {
            copy_nonoverlapping(src_ptr, dst_ptr, 1);
        }

        src_index += 1;
    }

    output
}

unsafe trait IntoVector: Construct {
    type T: Scalar;
    type Output<A: VecAlignment>: Construct;

    const SOURCES: &[usize];
}

// Into Vec2

unsafe impl<T: Scalar> IntoVector for (T, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<2, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector for (Vector<2, T, A0>,) {
    type T = T;
    type Output<A: VecAlignment> = Vector<2, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
    ];
}

// Into Vec3

unsafe impl<T: Scalar> IntoVector for (T, T, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<3, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector for (T, Vector<2, T, A1>) {
    type T = T;
    type Output<A: VecAlignment> = Vector<3, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector for (Vector<2, T, A0>, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<3, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector for (Vector<3, T, A0>,) {
    type T = T;
    type Output<A: VecAlignment> = Vector<3, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
    ];
}

// Into Vec4

unsafe impl<T: Scalar> IntoVector for (T, T, T, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
        offset_of!(Self, 3) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A2: VecAlignment> IntoVector for (T, T, Vector<2, T, A2>) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector for (T, Vector<2, T, A1>, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A1: VecAlignment> IntoVector for (T, Vector<3, T, A1>) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 2,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector for (Vector<2, T, A0>, T, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 2) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment, A1: VecAlignment> IntoVector
    for (Vector<2, T, A0>, Vector<2, T, A1>)
{
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 1) + size_of::<T>() * 0,
        offset_of!(Self, 1) + size_of::<T>() * 1,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector for (Vector<3, T, A0>, T) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
        offset_of!(Self, 1) + size_of::<T>() * 0,
    ];
}

unsafe impl<T: Scalar, A0: VecAlignment> IntoVector for (Vector<4, T, A0>,) {
    type T = T;
    type Output<A: VecAlignment> = Vector<4, T, A>;

    const SOURCES: &[usize] = &[
        offset_of!(Self, 0) + size_of::<T>() * 0,
        offset_of!(Self, 0) + size_of::<T>() * 1,
        offset_of!(Self, 0) + size_of::<T>() * 2,
        offset_of!(Self, 0) + size_of::<T>() * 3,
    ];
}
