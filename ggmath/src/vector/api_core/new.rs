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
pub const fn new_vector<const N: usize, T: Scalar, A: VecAlignment, I: IntoVector<N, T>>(
    value: I,
) -> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
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
