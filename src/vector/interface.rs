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

    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        let mut output = Self::GARBAGE;

        unsafe {
            copy_nonoverlapping::<T>(array.as_ptr(), output.as_mut_ptr(), N);
        }

        output
    }

    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        unsafe { transmute_copy::<Vector<N, T, A>, [T; N]>(&self) }
    }

    #[inline(always)]
    pub const fn as_array(&self) -> &[T; N] {
        unsafe { transmute::<&Vector<N, T, A>, &[T; N]>(self) }
    }

    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute::<&mut Vector<N, T, A>, &mut [T; N]>(self) }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.as_array().as_ptr()
    }

    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.as_array_mut().as_mut_ptr()
    }

    #[inline(always)]
    pub fn from_fn<F: Fn(usize) -> T>(f: F) -> Self
    where
        Usize<N>: VecLen,
    {
        Vector::from_array(std::array::from_fn(f))
    }

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
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        unsafe { transmute::<&[T; N], &Vector<N, T, VecPacked>>(array) }
    }

    #[inline(always)]
    pub const fn from_array_mut(array: &mut [T; N]) -> &mut Self {
        unsafe { transmute::<&mut [T; N], &mut Vector<N, T, VecPacked>>(array) }
    }
}
