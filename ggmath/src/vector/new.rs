use std::{
    mem::{MaybeUninit, offset_of},
    ptr::copy_nonoverlapping,
};

use super::*;

// Splat

/// Creates a `Vec2` where all components are given the same value.
#[inline(always)]
pub fn splat2<T: Scalar>(value: T) -> Vec2<T> {
    Vector::splat(value)
}

/// Creates a `Vec3` where all components are given the same value.
#[inline(always)]
pub const fn splat3<T: Scalar>(value: T) -> Vec3<T> {
    Vector::splat(value)
}

/// Creates a `Vec4` where all components are given the same value.
#[inline(always)]
pub const fn splat4<T: Scalar>(value: T) -> Vec4<T> {
    Vector::splat(value)
}

/// Creates a `Vec2P` where all components are given the same value.
#[inline(always)]
pub const fn splat2p<T: Scalar>(value: T) -> Vec2P<T> {
    Vector::splat(value)
}

/// Creates a `Vec3P` where all components are given the same value.
#[inline(always)]
pub const fn splat3p<T: Scalar>(value: T) -> Vec3P<T> {
    Vector::splat(value)
}

/// Creates a `Vec4P` where all components are given the same value.
#[inline(always)]
pub const fn splat4p<T: Scalar>(value: T) -> Vec4P<T> {
    Vector::splat(value)
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Creates a vector where all components are given the same value.
    #[inline(always)]
    pub const fn splat(value: T) -> Self {
        Self::from_array([value; N])
    }
}

// Construction

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

// Impl IntoVector

macro_loop! {
    // (_)

    @for N1 in 1..=4 {
        @let N = @N1;
        @let is_vec = @N >= 2 && @N <= 4;

        @if @is_vec {
            unsafe impl<
                T: Scalar,
                @if @N1 > 1 { A1: VecAlignment, }
            > IntoVector<@N, T> for (
                @if @N1 > 1 { Vector<@N1, T, A1> }
                @if @N1 == 1 { T },
            ) {
                const SOURCES: [usize; @N] = [
                    @for i in 0..@N1 {
                        offset_of!(Self, 0) + size_of::<T>() * @i,
                    }
                ];
            }
        }
    }

    // (_, _)

    @for N1 in 1..=4, N2 in 1..=4 {
        @let N = @N1 + @N2;
        @let is_vec = @N >= 2 && @N <= 4;

        @if @is_vec {
            unsafe impl<
                T: Scalar,
                @if @N1 > 1 { A1: VecAlignment, }
                @if @N2 > 1 { A2: VecAlignment, }
            > IntoVector<@N, T> for (
                @if @N1 > 1 { Vector<@N1, T, A1> }
                @if @N1 == 1 { T },
                @if @N2 > 1 { Vector<@N2, T, A2> }
                @if @N2 == 1 { T },
            ) {
                const SOURCES: [usize; @N] = [
                    @for i in 0..@N1 {
                        offset_of!(Self, 0) + size_of::<T>() * @i,
                    }
                    @for i in 0..@N2 {
                        offset_of!(Self, 1) + size_of::<T>() * @i,
                    }
                ];
            }
        }
    }

    // (_, _, _)

    @for N1 in 1..=4, N2 in 1..=4, N3 in 1..=4 {
        @let N = @N1 + @N2 + @N3;
        @let is_vec = @N >= 2 && @N <= 4;

        @if @is_vec {
            unsafe impl<
                T: Scalar,
                @if @N1 > 1 { A1: VecAlignment, }
                @if @N2 > 1 { A2: VecAlignment, }
                @if @N3 > 1 { A3: VecAlignment, }
            > IntoVector<@N, T> for (
                @if @N1 > 1 { Vector<@N1, T, A1> }
                @if @N1 == 1 { T },
                @if @N2 > 1 { Vector<@N2, T, A2> }
                @if @N2 == 1 { T },
                @if @N3 > 1 { Vector<@N3, T, A3> }
                @if @N3 == 1 { T },
            ) {
                const SOURCES: [usize; @N] = [
                    @for i in 0..@N1 {
                        offset_of!(Self, 0) + size_of::<T>() * @i,
                    }
                    @for i in 0..@N2 {
                        offset_of!(Self, 1) + size_of::<T>() * @i,
                    }
                    @for i in 0..@N3 {
                        offset_of!(Self, 2) + size_of::<T>() * @i,
                    }
                ];
            }
        }
    }

    // (_, _, _, _)

    @for N1 in 1..=4, N2 in 1..=4, N3 in 1..=4, N4 in 1..=4 {
        @let N = @N1 + @N2 + @N3 + @N4;
        @let is_vec = @N >= 2 && @N <= 4;

        @if @is_vec {
            unsafe impl<
                T: Scalar,
                @if @N1 > 1 { A1: VecAlignment, }
                @if @N2 > 1 { A2: VecAlignment, }
                @if @N3 > 1 { A3: VecAlignment, }
                @if @N4 > 1 { A4: VecAlignment, }
            > IntoVector<@N, T> for (
                @if @N1 > 1 { Vector<@N1, T, A1> }
                @if @N1 == 1 { T },
                @if @N2 > 1 { Vector<@N2, T, A2> }
                @if @N2 == 1 { T },
                @if @N3 > 1 { Vector<@N3, T, A3> }
                @if @N3 == 1 { T },
                @if @N4 > 1 { Vector<@N4, T, A4> }
                @if @N4 == 1 { T },
            ) {
                const SOURCES: [usize; @N] = [
                    @for i in 0..@N1 {
                        offset_of!(Self, 0) + size_of::<T>() * @i,
                    }
                    @for i in 0..@N2 {
                        offset_of!(Self, 1) + size_of::<T>() * @i,
                    }
                    @for i in 0..@N3 {
                        offset_of!(Self, 2) + size_of::<T>() * @i,
                    }
                    @for i in 0..@N4 {
                        offset_of!(Self, 3) + size_of::<T>() * @i,
                    }
                ];
            }
        }
    }
}
