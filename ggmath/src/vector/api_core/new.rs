use std::{mem::offset_of, ptr::copy_nonoverlapping};

use super::*;

macro_rules! new_vector_macro {
    ($macro_ident:ident $dollar:tt => $($output_type:tt)*) => {
        #[macro_export]
        macro_rules! $macro_ident {
            ($dollar($dollar item:expr), * $dollar(,)?) => {
                {
                    let output: $($output_type)* = $crate::new_vector(($dollar($dollar item,)*));
                    output
                }
            };
        }
    };
}

new_vector_macro! { vector $=> _ }
new_vector_macro! { vec2 $=> $crate::Vec2<_> }
new_vector_macro! { vec3 $=> $crate::Vec3<_> }
new_vector_macro! { vec4 $=> $crate::Vec4<_> }
new_vector_macro! { vec2p $=> $crate::Vec2P<_> }
new_vector_macro! { vec3p $=> $crate::Vec3P<_> }
new_vector_macro! { vec4p $=> $crate::Vec4P<_> }

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

#[doc(hidden)]
pub unsafe trait IntoVector: Construct {
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
