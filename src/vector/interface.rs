use std::{
    mem::{transmute, transmute_copy},
    ptr::copy_nonoverlapping,
};

use crate::{
    Usize,
    vector::{Scalar, VecAligned, VecAlignment, VecLen, VecLenEnum, VecPacked, Vector},
};

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// A constant with an unknown valid value.
    /// Can be used to initialize temporary vectors.
    pub const GARBAGE: Self = match A::IS_ALIGNED {
        true => match Usize::<N>::ENUM {
            VecLenEnum::Two => unsafe {
                transmute_copy::<Vector<2, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    2,
                    T,
                    VecAligned,
                > {
                    inner: T::INNER_ALIGNED_VEC2_GARBAGE,
                })
            },
            VecLenEnum::Three => unsafe {
                transmute_copy::<Vector<3, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    3,
                    T,
                    VecAligned,
                > {
                    inner: T::INNER_ALIGNED_VEC3_GARBAGE,
                })
            },
            VecLenEnum::Four => unsafe {
                transmute_copy::<Vector<4, T, VecAligned>, Vector<N, T, A>>(&Vector::<
                    4,
                    T,
                    VecAligned,
                > {
                    inner: T::INNER_ALIGNED_VEC4_GARBAGE,
                })
            },
        },
        false => unsafe {
            transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(&Vector {
                inner: [T::GARBAGE; N],
            })
        },
    };

    /// Creates a new vector from an array.
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        let mut output = Self::GARBAGE;

        unsafe {
            copy_nonoverlapping::<T>(array.as_ptr(), output.as_mut_ptr(), N);
        }

        output
    }

    /// Converts the vector to an array.
    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        unsafe { transmute_copy::<Vector<N, T, A>, [T; N]>(&self) }
    }

    /// Converts a vector reference to an array reference.
    #[inline(always)]
    pub const fn as_array(&self) -> &[T; N] {
        unsafe { transmute::<&Vector<N, T, A>, &[T; N]>(self) }
    }

    /// Converts a mutable vector reference to a mutable array reference.
    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute::<&mut Vector<N, T, A>, &mut [T; N]>(self) }
    }

    /// Returns a pointer to the first element of the vector.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.as_array().as_ptr()
    }

    /// Returns a mutable pointer to the first element of the vector.
    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.as_array_mut().as_mut_ptr()
    }

    /// Creates a new vector where each component is evaluated from the given function called with the component index.
    /// The function is called in order.
    #[inline(always)]
    pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self
    where
        Usize<N>: VecLen,
    {
        Vector::from_array(std::array::from_fn(f))
    }

    /// Maps each component of the vector to a new value using the given function.
    #[inline(always)]
    pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, A>
    where
        Usize<N>: VecLen,
    {
        Vector::from_array(self.as_array().map(f))
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
where
    Usize<N>: VecLen,
{
    /// Converts an array reference to a vector reference.
    ///
    /// This requires `VecPacked` alignment because a SIMD aligned vector reference is incompatible with an array reference.
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute::<&[T; N], &Vector<N, T, VecPacked>>(array) }
    }

    /// Converts a mutable array reference to a mutable vector reference.
    ///
    /// This requires `VecPacked` alignment because a SIMD aligned vector reference is incompatible with an array reference.
    #[inline(always)]
    pub const fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute::<&mut [T; N], &mut Vector<N, T, VecPacked>>(array) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{vec2, vec2p, vec3, vec3p, vec4, vec4p};

    use super::*;

    #[test]
    fn test_swizzle() {
        assert_eq!(vec2!(1, 2).yx(), vec2!(2, 1));
        assert_eq!(vec2!(1, 2).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec2!(1, 2).yxyy(), vec4!(2, 1, 2, 2));
        assert_eq!(vec3!(1, 2, 3).yz(), vec2!(2, 3));
        assert_eq!(vec3!(1, 2, 3).yxz(), vec3!(2, 1, 3));
        assert_eq!(vec3!(1, 2, 3).yxyz(), vec4!(2, 1, 2, 3));
        assert_eq!(vec4!(1, 2, 3, 4).yx(), vec2!(2, 1));
        assert_eq!(vec4!(1, 2, 3, 4).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec4!(1, 2, 3, 4).yxww(), vec4!(2, 1, 4, 4));

        assert_eq!(vec2p!(1, 2).yx(), vec2!(2, 1));
        assert_eq!(vec2p!(1, 2).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec2p!(1, 2).yxyy(), vec4!(2, 1, 2, 2));
        assert_eq!(vec3p!(1, 2, 3).yz(), vec2!(2, 3));
        assert_eq!(vec3p!(1, 2, 3).yxz(), vec3!(2, 1, 3));
        assert_eq!(vec3p!(1, 2, 3).yxyz(), vec4!(2, 1, 2, 3));
        assert_eq!(vec4p!(1, 2, 3, 4).yx(), vec2!(2, 1));
        assert_eq!(vec4p!(1, 2, 3, 4).yxy(), vec3!(2, 1, 2));
        assert_eq!(vec4p!(1, 2, 3, 4).yxww(), vec4!(2, 1, 4, 4));
    }
}
